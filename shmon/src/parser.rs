use std::num::IntErrorKind;
use std::str::FromStr;

use crate::cli::arguments::{Duration, Unit};
use crate::cli::regexes::DURATION_REGEX;
use log::debug;

pub fn parse_duration(input: Option<String>) -> Result<Duration, String> {

    let raw_duration: String = match input {
        Some(d) => d,
        None => {
            debug!("No duration value provided, turning off immediately");
            return Ok(Duration::default());
        }
    };

    let captures = match DURATION_REGEX.captures(&raw_duration) {
        Some(c) => c,
        None => return Err(format!("Input `{}` is not a valid duration argument", raw_duration))
    };

    let duration_value: usize = match captures.get(1) {
        None => return Err("No duration value provided".into()), // should not happen
        Some(raw_value) => match raw_value.as_str().parse::<usize>() {
            Ok(value) => value,
            Err(err) => match err.kind() {
                IntErrorKind::PosOverflow => return Err(format!("Duration value `{}` is too big", raw_value.as_str())),
                _ => return Err(format!("Duration value `{}` is not valid", raw_value.as_str())),
            }
        }
    };


    let duration_unit: Unit = match captures.get(2) {
        Some(raw_unit) => Unit::from_str(raw_unit.as_str()).map_err(|err| err.to_string())?,
        None => {
            debug!("No duration unit provided, using seconds");
            Unit::default()
        }
    };

    if duration_value == 0 {
        debug!("Provided duration value is `0`, turning off immediately");
    }

    return Ok(Duration { value: duration_value, unit: Some(duration_unit) });
}
