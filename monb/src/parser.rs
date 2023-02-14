use std::collections::HashMap;

pub fn parse_arguments(arguments: Vec<String>) -> (HashMap<u32, u32>, Option<u32>) {

    let mut per_monitor_brightness_values: HashMap<u32, u32> = HashMap::<u32, u32>::new();
    let mut global_brightness_value: Option<u32> = None;

    for mut argument in arguments {
        if let Some(valid_argument) = parse_argument(&mut argument) {
            match valid_argument.0 {
                Some(device_index) => {

                    if device_index == 0 { continue; }

                    if !per_monitor_brightness_values.contains_key(&device_index) {
                        per_monitor_brightness_values.insert(device_index, valid_argument.1);
                    }
                },
                None => {
                    if let None = global_brightness_value {
                        global_brightness_value = Some(valid_argument.1);
                    }
                }
            };
        }
    }

    return (per_monitor_brightness_values, global_brightness_value);
}

fn parse_argument(argument: &mut String) -> Option<(Option<u32>, u32)> { // Option<u32> - None if global, u32 if indexed monitor

    if argument.starts_with("/") { // custom slash arguments
        argument.remove(0);

        let potential_argument: Vec<&str> = argument.split(":").take(2).collect();
        if potential_argument.len() < 2 { return None; }

        let device_index_or_none: Option<u32>;
        let global_brightness: bool = potential_argument[0] == "*";

        if global_brightness {
            device_index_or_none = None;
        } else {
            if let Ok(parsed_index) = potential_argument[0].parse::<u32>() {
                device_index_or_none = Some(parsed_index);
            } else {
                return None;
            }
        }

        let brightness: u32 = match potential_argument[1].parse::<u32>() {
            Ok(parsed_brightness) => parsed_brightness.clamp(0, 100),
            _ => { return None; }
        };

        return Some((device_index_or_none, brightness));

    } else { // global argument
        return match argument.parse::<u32>() {
            Ok(parsed_brightness) => Some((None, parsed_brightness.clamp(0, 100))),
            _ => None
        }
    }
}