use brightness::blocking::{BrightnessDevice};
use monitor::get_devices;
use parser::parse_arguments;

use crate::monitor::{check_devices_brightness, set_device_brightness};

mod parser;
mod monitor;

fn main() {

    let devices: Vec<BrightnessDevice> = get_devices();
    let devices_length = devices.len();
    if devices_length == 0 { return; }

    let arguments: Vec<String> = std::env::args().skip(1).collect();
    if arguments.len() == 0 { // no arguments
        check_devices_brightness(&devices);
        return;
    }

    let (valid_arguments, global_brightness) = parse_arguments(arguments);
    if valid_arguments.len() == 0 && global_brightness.is_none() { // no valid arguments
        check_devices_brightness(&devices);
        return;
    }

    for (index, brightness) in valid_arguments.iter() {
        if let Some(device) = devices.get(*index as usize - 1) {
            set_device_brightness(device, index, *brightness);
        } else {
            logger::log_warn(format!("Index `{}` out of range", index));
        }
    }

    if let Some(brightness) = global_brightness {
        for vector_index in 0..devices_length {
            let valid_arguments_index = (vector_index + 1) as u32;
            if !valid_arguments.contains_key(&valid_arguments_index) {

                if let Some(device) = devices.get(vector_index) {
                    set_device_brightness(device, &valid_arguments_index, brightness);
                }
            }
        }
    }
}