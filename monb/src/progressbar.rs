use clap::ArgMatches;
use log::debug;
use std::num::IntErrorKind;
use strum::{EnumIter, IntoStaticStr, Display};

use crate::cli::params::{ARG_PROGRESSBAR_LENGTH_MAX, ARG_PROGRESSBAR_LENGTH_DEFAULT, ARG_PROGRESSBAR_LENGTH_MIN, ARG_PROGRESSBAR_LENGTH_ID, ARG_PROGRESSBAR_STYLE_ID};

#[derive(Debug, Default)]
#[derive(EnumIter, IntoStaticStr, Display)]
pub enum ProgressBarType {
    #[strum(serialize = "classic")]     Classic,
    #[strum(serialize = "arrow")]       Arrow,
    #[strum(serialize = "wsl")]         WSL,

    #[default]
    #[strum(serialize = "wsl_arrow")]   WSLArrow,

    #[strum(serialize = "filled")]      Filled
}

impl TryFrom<Option<&String>> for ProgressBarType {

    type Error = String;

    fn try_from(value: Option<&String>) -> Result<Self, Self::Error> {

        let potential_style: &String = match value {
            None => return Ok(Self::default()),
            Some(v) => v
        };

        debug!("Parsing progressbar style: `{}`", potential_style);

        let normalized_style: String = potential_style.trim().to_lowercase();
        let value: Result<Self, Self::Error> = match normalized_style.as_str() {
            "classic" => Ok(Self::Classic),
            "wsl" => Ok(Self::WSL),
            "arrow" => Ok(Self::Arrow),
            "wsl_arrow" => Ok(Self::WSLArrow),
            "filled" => Ok(Self::Filled),
            _ => Err(format!("Invalid progress bar type: `{}`", potential_style))
        };

        if value.is_ok() {
            debug!("Recognized progressbar style: `{}`", normalized_style);
        }

        return value;
    }
}

fn parse_progressbar_length(possible_input: Option<&String>) -> Result<usize, String> {

    let input: &String = match possible_input {
        None => return Ok(ARG_PROGRESSBAR_LENGTH_DEFAULT),
        Some(i) => i
    };

    debug!("Parsing progressbar length: `{}`", input);

    match input.parse::<usize>() {

        Ok(number) => {
            if number > ARG_PROGRESSBAR_LENGTH_MAX {
                debug!("Progressbar length `{}` is too big, clamping to {}", number, ARG_PROGRESSBAR_LENGTH_MAX);
                return Ok(ARG_PROGRESSBAR_LENGTH_MAX);
            } else if number < ARG_PROGRESSBAR_LENGTH_MIN {
                debug!("Progressbar length `{}` is too small, clamping to {}", number, ARG_PROGRESSBAR_LENGTH_MIN);
                return Ok(ARG_PROGRESSBAR_LENGTH_MIN);
            } else {
                return Ok(number);
            }
        },

        Err(err) => match err.kind() {
            IntErrorKind::PosOverflow => Err(format!("Progressbar length `{}` is too big", input)),
            _ => Err(format!("Invalid progressbar length: {}", input))
        }
    }
}

#[derive(Debug)]
pub struct ProgressBarInfo {
    pub _type: ProgressBarType,
    pub length: usize
}

impl TryFrom<&ArgMatches> for ProgressBarInfo {
    type Error = String;

    fn try_from(value: &ArgMatches) -> Result<Self, Self::Error> {
        let progressbar_length_unparsed: Option<&String> = value.get_one::<String>(ARG_PROGRESSBAR_LENGTH_ID);
        let progressbar_length: usize = parse_progressbar_length(progressbar_length_unparsed)?;

        let progressbar_type_unparsed: Option<&String> = value.get_one::<String>(ARG_PROGRESSBAR_STYLE_ID);
        let progressbar_type: ProgressBarType = ProgressBarType::try_from(progressbar_type_unparsed)?;

        Ok(ProgressBarInfo {
            _type: progressbar_type,
            length: progressbar_length
        })
    }
}

pub fn create_progressbar(clamped_brightness: u32, progressbar_info: &ProgressBarInfo) -> String {
    match progressbar_info._type {
        ProgressBarType::Classic => create_classic_progressbar(clamped_brightness, progressbar_info.length),
        ProgressBarType::Arrow => create_arrow_progressbar(clamped_brightness, progressbar_info.length),
        ProgressBarType::WSL => create_wsl_progressbar(clamped_brightness, progressbar_info.length),
        ProgressBarType::WSLArrow => create_wsl_arrow_progressbar(clamped_brightness, progressbar_info.length),
        ProgressBarType::Filled => create_filled_progressbar(clamped_brightness, progressbar_info.length)
    }
}

fn remap_value(value: f64, current_min: f64, current_max: f64, new_min: f64, new_max: f64) -> f64 {
    let clamped_value: f64 = value.clamp(current_min, current_max);
    return (clamped_value - current_min) / (current_max - current_min) * (new_max - new_min) + new_min;
}

fn create_wsl_progressbar(clamped_brightness: u32, length: usize) -> String {
    // create and fill the progressbar,
    let fill_length: usize = remap_value(clamped_brightness as f64, 0.0, 100.0, 0.0, length as f64) as usize;
    let empty_length: usize = length - fill_length;
    let mut progress_bar: String = format!("[{}{}]", "=".repeat(fill_length), " ".repeat(empty_length));

    // align percentage string
    let mut percentage: String = clamped_brightness.to_string();
    let padding: usize = length - percentage.len();
    let left_padding: usize = padding / 2;
    let right_padding: usize = padding - left_padding - 1; // -1 for the '%'

    percentage.push('%');
    progress_bar.replace_range(left_padding + 1..length - right_padding + 1, &percentage); // +1 for the '[' and ']'

    return progress_bar;
}

fn create_wsl_arrow_progressbar(clamped_brightness: u32, length: usize) -> String {
    // create and fill the progressbar,
    let fill_length: usize = remap_value(clamped_brightness as f64, 0.0, 100.0, 0.0, length as f64) as usize;
    let empty_length: usize = length - fill_length;
    let mut progress_bar: String = {
        if empty_length == 0 {
            format!("[{}]", "=".repeat(fill_length))
        } else if fill_length == 0 {
            format!("[{}]", " ".repeat(empty_length))
        } else {
            format!("[{}{}{}]", "=".repeat(fill_length - 1), '>', " ".repeat(empty_length))
        }
    };

    // align percentage string
    let mut percentage: String = clamped_brightness.to_string();
    let padding: usize = length - percentage.len();
    let left_padding: usize = padding / 2;
    let right_padding: usize = padding - left_padding - 1; // -1 for the '%'

    percentage.push('%');
    progress_bar.replace_range(left_padding + 1..length - right_padding + 1, &percentage); // +1 for the '[' and ']'

    return progress_bar;
}

fn create_classic_progressbar(clamped_brightness: u32, length: usize) -> String {
   return create_classic_progressbar_internal(clamped_brightness, length, "=", " ", "[", "]");
}

fn create_filled_progressbar(clamped_brightness: u32, length: usize) -> String {
    return create_classic_progressbar_internal(clamped_brightness, length, "█", " ", "¦", "¦");
}

fn create_classic_progressbar_internal(clamped_brightness: u32, length: usize, fill_symbol: &'static str, empty_symbol: &'static str, left_symbol: &'static str, right_symbol: &'static str) -> String {
    // create and fill the progressbar,
    let fill_length: usize = remap_value(clamped_brightness as f64, 0.0, 100.0, 0.0, length as f64) as usize;
    let empty_length: usize = length - fill_length;
    return format!("{}{}{}{} {}%", left_symbol, fill_symbol.repeat(fill_length), empty_symbol.repeat(empty_length), right_symbol, clamped_brightness);
}

fn create_arrow_progressbar(clamped_brightness: u32, length: usize) -> String {
    // create and fill the progressbar,
    let fill_length: usize = remap_value(clamped_brightness as f64, 0.0, 100.0, 0.0, length as f64) as usize;
    let empty_length: usize = length - fill_length;

    if empty_length == 0 {
        return format!("[{}] {}%", "=".repeat(fill_length), clamped_brightness);
    } else if fill_length == 0 {
        return format!("[{}] {}%", " ".repeat(empty_length), clamped_brightness);
    } else {
        return format!("[{}{}{}] {}%", "=".repeat(fill_length - 1), '>', " ".repeat(empty_length), clamped_brightness);
    }
}
