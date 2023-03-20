use crate::{logger::Logger, cli::arguments::DEFAULT_PROGRESSBAR_STYLE};

#[derive(Debug, Default)]
pub enum ProgressBarType {
    Classic,
    Arrow,
    WSL,
    #[default]
    WSLArrow,
    Filled
}

impl ProgressBarType {
    pub fn from_str(original: &String, logger: &Logger) -> ProgressBarType {

        let normalized: String = original.trim().to_lowercase();
        let normalized_str: &str = normalized.as_str();
        let recognized: bool = match normalized_str {
            "classic" | "wsl" | "arrow" | "wsl_arrow" | "filled" => true,
            _ => false
        };

        match normalized_str {
            "classic" => {
                if recognized {
                    logger.log_verbose_info(format!("Recognized `{}` as `classic`", original));
                } else {
                    logger.log_verbose_info("Succesfully parsed progressbar style: `classic`");
                }
                return Self::Classic;
            },

            "wsl" => {
                if recognized {
                    logger.log_verbose_info(format!("Recognized `{}` as `wsl`", original));
                } else {
                    logger.log_verbose_info("Succesfully parsed progressbar style: `wsl`");
                }
                return Self::WSL;
            },

            "arrow" => {
                if recognized {
                    logger.log_verbose_info(format!("Recognized `{}` as `arrow`", original));
                } else {
                    logger.log_verbose_info("Succesfully parsed progressbar style: `arrow`");
                }
                return Self::Arrow;
            },

            "wsl_arrow" => {
                if recognized {
                    logger.log_verbose_info(format!("Recognized `{}` as `wsl_arrow`", original));
                } else {
                    logger.log_verbose_info("Succesfully parsed progressbar style: `wsl_arrow`");
                }
                return Self::WSLArrow;
            },

            "filled" => {
                if recognized {
                    logger.log_verbose_info(format!("Recognized `{}` as `filled`", original));
                } else {
                    logger.log_verbose_info("Succesfully parsed progressbar style: `filled`");
                }
                return Self::Filled;
            },

            _ => {
                logger.log_warn(format!("Invalid value provided for progressbar style. Using default: `{}`", DEFAULT_PROGRESSBAR_STYLE));
                return Self::default();
            }
        }
    }
}

#[derive(Debug)]
pub struct ProgressBarInfo {
    pub _type: ProgressBarType,
    pub length: usize
}

pub fn create_progressbar(clamped_brightness: u32, length: usize, progressbar_type: &ProgressBarType) -> String {
    match progressbar_type {
        ProgressBarType::Classic => create_classic_progressbar(clamped_brightness, length),
        ProgressBarType::Arrow => create_arrow_progressbar(clamped_brightness, length),
        ProgressBarType::WSL => create_wsl_progressbar(clamped_brightness, length),
        ProgressBarType::WSLArrow => create_wsl_arrow_progressbar(clamped_brightness, length),
        ProgressBarType::Filled => create_filled_progressbar(clamped_brightness, length)
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
