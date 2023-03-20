use std::collections::HashMap;

use brightness::Error;
use brightness::blocking::{brightness_devices, BrightnessDevice, Brightness};

use crate::cli::arguments::{BValue, BAction, BArg, BScope};
use crate::cli::flags::Flags;
use crate::logger::Logger;
use crate::progressbar::{ProgressBarInfo, create_progressbar};

#[macro_export]
macro_rules! ternary_operator {
    ($condition: expr, $true_value: expr, $false_value: expr) => {
        if $condition {
            $true_value
        } else {
            $false_value
        }
    };
}

#[derive(Debug)]
pub struct Monitor {
    pub name: String,
    pub device: BrightnessDevice
}

#[cfg(windows)]
mod name {
    use brightness::blocking::BrightnessDevice;
    use brightness::blocking::windows::BrightnessExt;

    pub fn get_device_name(device: &BrightnessDevice, index: usize) -> String {
        if let Ok(device_description) = device.device_description() {
            return format!("Monitor #{} ({})", index, device_description);
        } else {
            return format!("Monitor #{}", index);
        }
    }
}

#[cfg(target_os = "linux")]
mod name {
    use brightness::blocking::BrightnessDevice;

    pub fn get_device_name(_: &BrightnessDevice, index: usize) -> String {
        return format!("Monitor #{}", index);
    }
}

pub fn get_all_devices(logger: &Logger, flags: &Flags) -> Result<HashMap<usize, Monitor>, String> {
    let mut devices: HashMap<usize, Monitor> = HashMap::<usize, Monitor>::new();

    let potential_devices: Vec<Result<BrightnessDevice, Error>> = brightness_devices().collect();
    let potential_devices_len: usize = potential_devices.len();
    if potential_devices_len == 0 {
        return Err("No monitors found :(".into());
    } else {
        logger.log_info(format!("{} monitor/s found", potential_devices_len));
    }

    // if zero flags is enabled do not offset, else offset by 1
    let zero_offset = ternary_operator!(flags.zero, 0, 1);

    for (mut index, potential_device) in potential_devices.into_iter().enumerate() {
        index += zero_offset;

        match potential_device {
            Ok(device) => {
                devices.insert(
                    index,
                    Monitor { name: name::get_device_name(&device, index), device }
                );
            },
            Err(err) => {
                logger.log_error(format!("Monitor #{} - {}", index, err.to_string()));
            }
        };
    }

    if devices.is_empty() {
        return Err("No valid monitors found :(".into());
    }

    let devices_len: usize = devices.len();
    if devices_len == potential_devices_len {
        logger.log_verbose_info("All monitors are valid");
    } else {
        logger.log_warn(format!("{} monitor/s valid", devices_len));
    }

    return Ok(devices);
}

pub fn print_monitor(monitor: Monitor, logger: &Logger, progressbar_info: &ProgressBarInfo) {
    let mut current_brightness: u32 = match monitor.device.get() {
        Ok(b) => b,
        Err(err) => {
            logger.log_error(format!("{} - {}", monitor.name, err.to_string()));
            return;
        }
    };

    current_brightness = ternary_operator!(current_brightness > 100, {
        logger.log_verbose_warn(format!("Monitor returned brightness `{}`, clamping to 100", current_brightness));
        100
    }, current_brightness);

    logger.log_info(format!("{} - {}", monitor.name, create_progressbar(current_brightness, progressbar_info.length, &progressbar_info._type)));
}

pub fn print_all_monitors(logger: &Logger, monitors: HashMap<usize, Monitor>, progressbar_info: &ProgressBarInfo) {
    logger.log_verbose_info("Showing all monitors");

    for (_, monitor) in monitors {
        print_monitor(monitor, logger, progressbar_info);
    }
}

pub fn apply_bvalue(monitor: &Monitor, bvalue: &BValue, logger: &Logger) {
    let mut current_brightness: u32 = match monitor.device.get() {
        Ok(b) => b,
        Err(err) => {
            logger.log_error(format!("{} - {}", monitor.name, err.to_string()));
            return;
        }
    };
    current_brightness = ternary_operator!(current_brightness > 100, {
        logger.log_verbose_warn(format!("Monitor returned brightness `{}`, clamping to 100", current_brightness));
        100
    }, current_brightness);

    let desired_brightness: u32 = match bvalue.action {
        BAction::Dec => {
            let desired_decrement_unclamped: u32 = bvalue.brightness as u32;
            let (desired_brightness, overflow) = current_brightness.overflowing_sub(desired_decrement_unclamped);
            if overflow { // u32 subtraction overflow
                logger.log_warn("Desired brightness was smaller than 0, clamping to 0");
                0
            } else {
                desired_brightness
            }
        },
        BAction::Inc => {
            let desired_increment_unclamped: u32 = bvalue.brightness as u32;
            let (desired_brightness, overflow) = current_brightness.overflowing_add(desired_increment_unclamped);
            if overflow { // u32 addition overflow
                logger.log_warn("Desired brightness was bigger than 100, clamping to 100");
                100
            } else {
                ternary_operator!(desired_brightness > 100, {
                    logger.log_warn("Desired brightness was bigger than 100, clamping to 100");
                    100
                }, desired_brightness)
            }
        },
        BAction::Set => {
            if bvalue.brightness > 100 {
                logger.log_warn("Desired brightness was bigger than 100, clamping to 100");
                100
            } else {
                bvalue.brightness as u32
            }
        }
    };

    if desired_brightness == current_brightness {
        logger.log_info(format!("{} - no need to change brightness", monitor.name));
        return;
    }

    match monitor.device.set(desired_brightness) {
        Ok(()) => {
            logger.log_info(format!("{} - successfully set monitor brightness to `{}`", monitor.name, desired_brightness));
        },
        Err(err) => {
            logger.log_error(format!("{} - failed to set monitor brightness to `{}` ({})", monitor.name, desired_brightness, err.to_string()));
        }
    };
}

pub fn apply_bargs(mut devices: HashMap<usize, Monitor>, bargs: Vec<BArg>, logger: &Logger) {

    let mut potential_barg_all: Option<BArg> = None;

    // apply bargs for all indexes (Scope::Index), but not the Scope::All
    for barg in bargs.into_iter() {
        let index: usize = match barg.scope {
            BScope::All => {
                if potential_barg_all.is_some() {
                    logger.log_warn(format!("Global brightness argument already set. Ignoring provided global argument"));
                } else {
                    potential_barg_all = Some(barg);
                }
                continue;
            },
            BScope::Index(i) => i
        };

        match devices.remove(&index) {
            None => logger.log_warn(format!("Monitor with index `{}` not found. Check the `zero` flag if you think this is an error", index)),
            Some(monitor) => apply_bvalue(&monitor, &barg.value, logger)
        };
    }

    // apply Scope::All brightness argument to the remaining monitors
    if let Some(barg_all) = potential_barg_all {
        if devices.len() == 0 {
            logger.log_verbose_warn("No remaining monitors to apply global brightness argument");
            return;
        }

        devices.into_iter().for_each(|(_, mon)| apply_bvalue(&mon, &barg_all.value, logger));
    }

    logger.log_verbose_info("Successfully applied all brightness arguments");
}