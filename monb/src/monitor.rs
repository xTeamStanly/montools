use std::collections::{HashMap, HashSet};

use brightness::Error;
use brightness::blocking::{brightness_devices, BrightnessDevice, Brightness};
use log::{error, info, warn, debug};

use crate::cli::params::{BAction, BArg, BScope, Arguments, Getter};
use crate::progressbar::{ProgressBarInfo, create_progressbar};

#[cfg(windows)]
use crate::monitor::windows::get_device_name;

#[cfg(target_os = "linux")]
use crate::monitor::linux::get_device_name;

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

impl Monitor {
    fn print(&self, progressbar_info: &ProgressBarInfo) {
        let mut current_brightness: u32 = match self.device.get() {
            Ok(b) => b,
            Err(err) => { error!("{} - {}", self.name, err.to_string()); return; }
        };
        if current_brightness > 100 {
            debug!("Monitor returned brightness `{}`, clamping to 100", current_brightness);
            current_brightness = 100;
        };

        info!("{} - {}", self.name, create_progressbar(current_brightness, progressbar_info));
    }
}

#[cfg(windows)]
mod windows {
    use brightness::blocking::BrightnessDevice;
    use brightness::blocking::windows::BrightnessExt;
    use log::debug;

    pub fn get_device_name(device: &BrightnessDevice, index: usize) -> String {
        match device.device_description() {
            Ok(device_description) => format!("Monitor #{} ({})", index, device_description),
            Err(err) => {
                debug!("Monitor #{} - {}", index, err.to_string());
                format!("Monitor #{}", index)
            }
        }
    }
}


#[cfg(target_os = "linux")]
mod linux {
    use brightness::blocking::BrightnessDevice;

    pub fn get_device_name(_: &BrightnessDevice, index: usize) -> String {
        return format!("Monitor #{}", index);
    }
}


pub fn get_all_devices(flag_zero: bool) -> Result<HashMap<usize, Monitor>, String> {
    let mut devices: HashMap<usize, Monitor> = HashMap::<usize, Monitor>::new();

    let potential_devices: Vec<Result<BrightnessDevice, Error>> = brightness_devices().collect();
    let potential_devices_len: usize = potential_devices.len();
    if potential_devices_len == 0 {
        return Err("No monitors found :(".into());
    } else {
        info!("{} monitor/s found", potential_devices_len);
    }

    // if zero flags is enabled do not offset, else offset by 1
    let zero_offset = ternary_operator!(flag_zero, 0, 1);

    for (mut index, potential_device) in potential_devices.into_iter().enumerate() {
        index += zero_offset;

        match potential_device {
            Ok(device) => {
                devices.insert(
                    index,
                    Monitor { name: get_device_name(&device, index), device }
                );
            },
            Err(err) => error!("Monitor #{} - {}", index, err.to_string())
        }
    };

    if devices.is_empty() {
        return Err("No valid monitors found :(".into());
    }

    let devices_len: usize = devices.len();
    if devices_len == potential_devices_len {
        debug!("All monitors are valid");
    } else {
        warn!("{} monitor/s valid", devices_len);
    }

    return Ok(devices);
}

pub fn apply_setter_barg(monitor: &Monitor, barg: &BArg) -> Result<(), String> {

    let mut current_brightness: u32 = match monitor.device.get() {
        Ok(b) => b,
        Err(err) => return Err(format!("{} - {}", monitor.name, err.to_string()))
    };
    if current_brightness > 100 {
        debug!("Monitor returned brightness `{}`, clamping to 100", current_brightness);
        current_brightness = 100;
    };

    let barg_scope: String = barg.scope.to_string();
    let bvalue_brightness: Option<usize> = barg.value.brightness;

    let mut desired_brightness: u32 = match barg.value.action {

        BAction::Dec => {
            let desired_decrement_unclamped: u32 = match bvalue_brightness {
                Some(b) => b as u32,
                None => return Err(format!("Unexpected error, brightness increment value for scope `{}` was `None`", barg_scope))
            };

            let (desired_brightness, overflow) = current_brightness.overflowing_sub(desired_decrement_unclamped);
            if overflow { // u32 subtraction overflow
                debug!("Desired brightness value for scope `{}` was smaller than 0, clamping to 0", barg_scope);
                0
            } else {
                desired_brightness
            }
        },

        BAction::Inc => {
            let desired_increment_unclamped: u32 = match bvalue_brightness {
                Some(b) => b as u32,
                None => return Err(format!("Unexpected error, brightness decrement value for scope `{}` was `None`", barg_scope))
            };

            let (desired_brightness, overflow) = current_brightness.overflowing_add(desired_increment_unclamped);
            if overflow {
                debug!("Desired brightness value for scope `{}` was bigger than 100, clamping to 100", barg_scope);
                100
            } else {
                desired_brightness
            }
        },

        BAction::Set => {
            let desired_brightness_value: u32 = match bvalue_brightness {
                Some(b) => b as u32,
                None => return Err(format!("Unexpected error, brightness increment value for scope `{}` was `None`", barg_scope))
            };

            if desired_brightness_value > 100 {
                debug!("Desired brightness value for scope `{}` was bigger than 100, clamping to 100", barg_scope);
                100
            } else {
                desired_brightness_value
            }
        },

        BAction::Get => return Err(format!("Unexpected error, brightness decrement value for scope `{}` was `None`", barg_scope))
    };

    if current_brightness == desired_brightness {
        warn!("{} - no need to change brightness", monitor.name);
        return Ok(());
    }

    if desired_brightness > 100 {
        debug!("Monitor desired brightness `{}`, clamping to 100", desired_brightness);
        desired_brightness = 100;
    };

    match monitor.device.set(desired_brightness) {
        Ok(_) => debug!("{} - successfully set monitor brightness to `{}`", monitor.name, desired_brightness),
        Err(err) => error!("{} - failed to set monitor brightness to `{}` ({})", monitor.name, desired_brightness, err.to_string())
    };

    return Ok(());
}

pub fn apply_arguments(arguments: Arguments) -> Result<(), String> {
    // get devices
    let devices: HashMap<usize, Monitor> = get_all_devices(arguments.flag_zero)?;

    let mut potential_set_global_barg: Option<&BArg> = None;
    let mut used_setter_indices: HashSet<usize> = HashSet::<usize>::new();

    // apply all setters without global scope
    // also add them to the hashset
    for barg in &arguments.bargs.setters {
        let index: usize = match barg.scope {
            BScope::Global => {
                if potential_set_global_barg.is_some() {
                    warn!("Global brightness argument already set. Ignoring provided global argument");
                } else {
                    potential_set_global_barg = Some(barg);
                }

                continue;
            },
            BScope::Index(i) => i
        };

        match devices.get(&index) {
            None => warn!("Monitor with index `{}` not found. Check the `zero` flag if you think this is an error", index),
            Some(monitor) => {
                used_setter_indices.insert(index);
                if let Err(err) = apply_setter_barg(monitor, &barg) {
                    error!("{}", err);
                };
            }
        }
    }

    // if global setter exists
    // apply global setter
    // to all monitors not in hashset
    if let Some(global_setter) = potential_set_global_barg {
        for (monitor_index, monitor) in &devices {
            if !used_setter_indices.contains(monitor_index) {
                if let Err(err) = apply_setter_barg(monitor, global_setter) {
                    error!("{}", err);
                }
            }

        }
    }

    debug!("Successfully applied all brightness arguments");

    // apply getters
    if let Some(getters) = arguments.bargs.getters {
        match getters {
            Getter::Global => devices.values().for_each(|monitor| monitor.print(&arguments.progressbar_info)), // print all monitors
            Getter::Many(indexed_getters) => {
                for scope in indexed_getters {
                    match scope {
                        BScope::Global => {
                            debug!("Should not happen!");
                            error!("Unexpected `global getter` inside `indexed getters` array");
                        },

                        BScope::Index(index) => {
                            match devices.get(&index) {
                                None => warn!("Monitor with index `{}` not found. Check the `zero` flag if you think this is an error", index),
                                Some(monitor) => monitor.print(&arguments.progressbar_info)
                            }
                        }
                    }
                };
            }
        };
    }

    return Ok(());
}
