use logger;
use brightness::{Error, blocking::{brightness_devices, BrightnessDevice, Brightness}};

#[cfg(windows)]
use brightness::blocking::windows::BrightnessExt;

fn create_progress_bar(value: u32) -> String {

    let clamped_value: u32 = value.clamp(0, 100);

    let mut result: String = String::from("[");
    let percentage: u32 = clamped_value / 5; // progress bar is 20 chars long

    for i in 0..20 {
        if i < percentage {
            result.push('=');
        } else {
            result.push(' ');
        }
    }

    result.push_str(&format!("] {}%", clamped_value));
    return result;
}

pub fn get_devices() -> Vec<BrightnessDevice> {

    let mut devices: Vec<BrightnessDevice> = Vec::<BrightnessDevice>::new();

    let mut potential_devices: Vec<Result<BrightnessDevice, Error>> = brightness_devices().collect();
    let potential_device_length: usize = potential_devices.len();

    if potential_device_length == 0 {
        logger::log_warn(format!("No monitors found"));
        return devices;
    } else {
        logger::log_info(format!("{} monitor/s found", potential_device_length));
    }

    for i in 0..potential_device_length {
        match potential_devices.remove(i) {
            Err(why) => { logger::log_error(format!("Monitor #{} - {}", i + 1, why.to_string())); },
            Ok(device) => { devices.push(device); }
        }
    }

    let devices_length: usize = devices.len();

    if devices_length == 0 {
        logger::log_warn(format!("No valid monitors found"));
    } else if devices_length != potential_device_length {
        logger::log_warn(format!("{} monitor/s valid", devices_length));
    } else {
        logger::log_info(format!("All monitors are valid"));
    }

    return devices;
}

pub fn check_devices_brightness(devices: &Vec<BrightnessDevice>) {
    for i in 0..devices.len() {
        check_device_brightness(&devices[i], i);
    }
}

pub fn check_device_brightness(device: &BrightnessDevice, index: usize) {
    let mut result: String = get_device_name(&device, &(index as u32));

    if let Ok(brightness) = device.get() {
        result.push_str(&format!(" - {}", create_progress_bar(brightness)));
    } else {
        result.push_str(" doesn't support DDC/CI");
    }

    logger::log_info(result);
}

#[cfg(windows)]
fn get_device_name(device: &BrightnessDevice, index: &u32) -> String {
    if let Ok(monitor_description) = device.device_description() {
        return format!("Monitor ({})", monitor_description);
    } else {
        return format!("Monitor #{}", index + 1);
    }
}

#[cfg(target_os = "linux")]
fn get_device_name(device: &BrightnessDevice, index: &u32) -> String {
    return format!("Monitor #{}", index + 1);
}

pub fn set_device_brightness(device: &BrightnessDevice, index: &u32, brightness: u32) {
    let mut result: String = get_device_name(&device, index);

    if let Ok(current_brightness) = device.get() {
        if current_brightness == brightness {
            result.push_str(" no need to change brightness");
            logger::log_info(result);
            return;
        }
    }

    if let Ok(()) = device.set(brightness) {
        result.push_str(&format!(" successfully set monitor brightness to {}", brightness));
        logger::log_info(result);
    } else {
        result.push_str(" failed to set brightness");
        logger::log_error(result);
    }
}
