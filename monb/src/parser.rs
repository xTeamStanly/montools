use std::collections::{HashMap, HashSet};
use std::num::IntErrorKind;

use clap::{ArgMatches, parser::ValuesRef};
use regex::Captures;

use crate::logger::Logger;
use crate::cli::flags::Flags;
use crate::cli::regexes::{MAX_NAMES, MIN_NAMES, BARG_REGEX, BVALUE_REGEX};
use crate::cli::arguments::{MONITOR_INDICES, BRIGHTNESS_ARGUMENTS, BRIGHTNESS_VALUE, BArg, BScope, BValue, BAction, PROGRESSBAR_LENGTH, PROGRESSBAR_STYLE, DEFAULT_PROGRESSBAR_LENGTH, MAX_PROGRESSBAR_LENGTH, MIN_PROGRESSBAR_LENGTH, DEFAULT_PROGRESSBAR_STYLE};
use crate::monitor::{print_all_monitors, get_all_devices, Monitor, print_monitor, apply_bargs};
use crate::progressbar::{ProgressBarInfo, ProgressBarType};

pub fn get_command(arguments: &ArgMatches, flags: &Flags, logger: &Logger) {
    if let Some(("all", _)) = arguments.subcommand() {
        default_behaviour(arguments, flags, logger);
    } else {
        if let Some(monitor_indices) = arguments.get_many::<String>(MONITOR_INDICES) {

            let mut devices = match get_all_devices(logger, flags) {
                Ok(dev) => dev,
                Err(err) => {
                    logger.log_error(err);
                    return;
                }
            };

            let progressbar_info: ProgressBarInfo = parse_progressbar_info(arguments, logger);
            let indices: Vec<usize> = match parse_indices(monitor_indices, logger) {
                Ok(ind) => ind,
                Err(err) => {
                    logger.log_error(err);
                    return;
                }
            };

            for index in indices {
                match devices.remove(&index) {
                    None => {
                        logger.log_warn(format!("Index `{}` is out of bounds", index));
                    },
                    Some(monitor) => {
                        print_monitor(monitor, logger, &progressbar_info);
                    }
                }
            }

        } else {
            default_behaviour(arguments, flags, logger);
        }
    }
}

pub fn set_command(arguments: &ArgMatches, flags: &Flags, logger: &Logger) {
    if let Some(("all", all_arguments)) = arguments.subcommand() {
        let potential_bvalue: Option<&String> = all_arguments.get_one::<String>(BRIGHTNESS_VALUE);
        let bvalue: BValue = match parse_bvalue(potential_bvalue, logger) {
            Err(err) => {
                logger.log_error(err);
                return;
            },
            Ok(val) => val
        };

        let devices = match get_all_devices(logger, flags) {
            Err(err) => {
                logger.log_error(err);
                return;
            },
            Ok(dev) => dev
        };

        let barg: BArg = BArg {
            scope: BScope::All,
            value: bvalue
        };

        apply_bargs(devices, vec![barg], logger);
    } else {
        if let Some(raw_bargs) = arguments.get_many::<String>(BRIGHTNESS_ARGUMENTS) {
            let bargs: Vec<BArg> = match parse_bargs(raw_bargs, logger) {
                Err(err) => {
                    logger.log_error(err);
                    return;
                },
                Ok(args) => args
            };

            let devices = match get_all_devices(logger, flags) {
                Err(err) => {
                    logger.log_error(err);
                    return;
                },
                Ok(dev) => dev
            };

            apply_bargs(devices, bargs, logger);
        } else {
            logger.log_error("No brightness arguments provided");
        }
    }
}

fn parse_progressbar_length(input: Option<&String>, logger: &Logger) -> usize {
    let unparsed: &String = match input {
        None => {
            logger.log_warn(format!("No value provided for progressbar length. Using default: `{}`", DEFAULT_PROGRESSBAR_LENGTH));
            return DEFAULT_PROGRESSBAR_LENGTH;
        },
        Some(raw_value) => raw_value
    };

    return match unparsed.parse::<usize>() {
        Ok(num) => {
            if num > MAX_PROGRESSBAR_LENGTH {
                logger.log_verbose_warn(format!("Progressbar length `{}` too big, clamping to {}", num, MAX_PROGRESSBAR_LENGTH));
                return MAX_PROGRESSBAR_LENGTH;
            } else if num < MIN_PROGRESSBAR_LENGTH {
                logger.log_verbose_warn(format!("Progressbar length `{}` too small, clamping to {}", num, MIN_PROGRESSBAR_LENGTH));
                return MIN_PROGRESSBAR_LENGTH;
            } else {
                logger.log_verbose_info(format!("Succesfully parsed progressbar length: `{}`", num));
                return num;
            }
        },
        Err(err) => {
            logger.log_error(format!("Invalid progressbar length: `{}`\nUsing default: `{}`", err, DEFAULT_PROGRESSBAR_LENGTH));
            DEFAULT_PROGRESSBAR_LENGTH
        }
    };
}

fn parse_progressbar_style(input: Option<&String>, logger: &Logger) -> ProgressBarType {
    let unparsed: &String = match input {
        None => {
            logger.log_warn(format!("No value provided for progressbar style. Using default: `{}`", DEFAULT_PROGRESSBAR_STYLE));
            return ProgressBarType::default();
        },
        Some(raw_value) => raw_value
    };

    return ProgressBarType::from_str(unparsed, logger);
}

pub fn default_command(arguments: &ArgMatches, flags: &Flags, logger: &Logger) {

    let potential_bargs = arguments.get_many::<String>(BRIGHTNESS_ARGUMENTS);
    let raw_bargs: ValuesRef<String> = match potential_bargs {
        None => {
            logger.log_verbose_warn("No brightness arguments provided");
            default_behaviour(arguments, flags, logger);
            return;
        },
        Some(raw) => raw
    };

    let bargs: Vec<BArg> = match parse_bargs(raw_bargs, logger) {
        Err(err) => {
            logger.log_error(err);
            return;
        },
        Ok(args) => args
    };

    let devices = match get_all_devices(logger, flags) {
        Err(err) => {
            logger.log_error(err);
            return;
        },
        Ok(dev) => dev
    };

    apply_bargs(devices, bargs, logger);
}

fn parse_progressbar_info(arguments: &ArgMatches, logger: &Logger) -> ProgressBarInfo {
    let progressbar_length_unparsed: Option<&String> = arguments.get_one::<String>(PROGRESSBAR_LENGTH);
    let progressbar_length: usize = parse_progressbar_length(progressbar_length_unparsed, logger);

    let progressbar_type_unparsed: Option<&String> = arguments.get_one::<String>(PROGRESSBAR_STYLE);
    let progressbar_type: ProgressBarType = parse_progressbar_style(progressbar_type_unparsed, logger);

    return ProgressBarInfo {
        _type: progressbar_type,
        length: progressbar_length
    };
}

fn default_behaviour(arguments: &ArgMatches, flags: &Flags, logger: &Logger) {
    let progressbar_info: ProgressBarInfo = parse_progressbar_info(arguments, logger);
    let monitors: HashMap<usize, Monitor> = match get_all_devices(logger, flags) {
        Err(err) => {
            logger.log_error(err);
            return;
        },
        Ok(map) => map
    };

    print_all_monitors(logger, monitors, &progressbar_info);
}

fn parse_indices(input: ValuesRef<String>, logger: &Logger) -> Result<Vec<usize>, String> {
    if input.len() == 0 {
        return Err("No monitor indices provided".into());
    }

    // hashset for deduplication, vector for preserving order
    let mut indices_set: HashSet<usize> = HashSet::<usize>::new();
    let mut indices: Vec<usize> = Vec::<usize>::new();
    input.into_iter().for_each(|possible_usize| {
        match possible_usize.parse::<usize>() {
            Err(err) => match err.kind() {
                IntErrorKind::PosOverflow => { logger.log_error(format!("Index value `{}` is too big", possible_usize)); },
                _ => { logger.log_error(format!("Index value `{}` is not valid", possible_usize)); }
            },
            Ok(index) => {
                if !indices_set.contains(&index) {
                    indices_set.insert(index);
                    indices.push(index);
                } else {
                    logger.log_verbose_warn(format!("Index `{}` is a duplicate index", index));
                }
            }
        };
    });

    return Ok(indices);
}

fn parse_bargs(raw_bargs: ValuesRef<String>, logger: &Logger) -> Result<Vec<BArg>, String> {
    let mut bargs_scopes_deduplication: HashSet<BScope> = HashSet::<BScope>::new();
    let mut bargs: Vec<BArg> = Vec::<BArg>::new();

    raw_bargs.for_each(|potential_barg| {
        match BARG_REGEX.captures(potential_barg) {
            None => logger.log_error(format!("Input `{}` is not a valid brightness argument", potential_barg)),
            Some(c) => match parse_barg_from_regex(potential_barg, c, logger) {
                Err(err) => { logger.log_error(err); },
                Ok(barg) => {
                    if !bargs_scopes_deduplication.contains(&barg.scope) {
                        bargs_scopes_deduplication.insert(barg.scope.clone());
                        bargs.push(barg);
                    } else {
                        logger.log_verbose_warn(format!("Duplicate brightness argument for scope `{}`", match barg.scope {
                            BScope::All => "*".to_string(),
                            BScope::Index(i) => i.to_string()
                        }));
                    }
                }
            }
        }
    });

    if bargs.len() == 0 {
        return Err("No valid brightness arguments provided".into());
    } else {
        return Ok(bargs);
    }
}

fn parse_bvalue(raw_bvalue: Option<&String>, logger: &Logger) -> Result<BValue, String> {
    return match raw_bvalue {
        None => Err("No brightness value provided".into()),
        Some(potential_bvalue) => {
            match BVALUE_REGEX.captures(&potential_bvalue) {
                None => Err(format!("Input `{}` is not a valid brightness value", potential_bvalue)),
                Some(c) => match parse_bvalue_from_regex(&potential_bvalue, c, logger) {
                    Err(err) => Err(err),
                    Ok(bvalue) => Ok(bvalue)
                }
            }
        }
    };
}

fn parse_bvalue_from_regex(original: &String, captures: Captures, logger: &Logger) -> Result<BValue, String> {

    logger.log_verbose_info(format!("Parsing brightness value: `{}`", original));

    let mut action: BAction = match captures.get(2) {
        None => BAction::Set,
        Some(a) => match a.as_str() {
            "+" => BAction::Inc,
            "-" => BAction::Dec,
            _ => BAction::Set
        }
    };

    // check if min|max values are provided
    let min_or_max: Option<usize> = match captures.get(3) {
        None => return Err("No brightness string provided".into()),
        Some(m) => {
            let m_str: &str = m.as_str();
            if MAX_NAMES.contains(&m_str) {

                let val: Option<usize> = match action {
                    BAction::Dec => Some(0),
                    _ => Some(100)
                };

                action = BAction::Set;
                val
            } else if MIN_NAMES.contains(&m_str) {
                Some(1)
            } else {
                None
            }
        }
    };
    if let Some(min_or_max_value) = min_or_max {
        logger.log_verbose_info(format!("Brightness value `{}` parsed successfully", original));

        return Ok(BValue {
            action,
            brightness: min_or_max_value
        });
    };

    // no min|max values are provided
    let mut value: usize = match captures.get(4) {
        None => return Err("No brightness value provided".into()),
        Some(v) => match v.as_str().parse::<usize>() {
            Ok(vv) => vv,
            Err(err) => match err.kind() {
                IntErrorKind::PosOverflow => return Err(format!("Brightness value `{}` is too big", v.as_str())),
                _ => return Err(format!("Brightness value `{}` is not valid", v.as_str()))
            }
        }
    };
    if value > 100 {
        logger.log_verbose_warn(format!("Brightness value `{}` is bigger than 100, clamping to 100", value));
        value = 100;
    }

    let denominator: Option<usize> = match captures.get(6) {
        None => None,
        Some(d) => match d.as_str().parse::<usize>() {
            Ok(dd) => Some(dd),
            Err(err) => match err.kind() {
                IntErrorKind::PosOverflow => return Err(format!("Denominator value `{}` is too big", d.as_str())),
                _ => return Err(format!("Denominator value `{}` is not valid", d.as_str()))
            }
        }
    };

    let mut final_value: usize = {
        if let Some(denominator_value) = denominator {
            if denominator_value == 0 {
                return Err("Denominator value is `0`, cannot divide by zero".into());
            } else {

                // is the percent sign provided
                if captures.get(7).is_some() {
                    value * 100 / denominator_value
                } else {
                    value / denominator_value
                }

            }
        } else {
            value
        }
    };

    if final_value > 100 {
        logger.log_verbose_warn(format!("Brightness value `{}` is bigger than 100, clamping to 100", final_value));
        final_value = 100;
    }

    logger.log_verbose_info(format!("Brightness value `{}` parsed successfully", original));
    return Ok(BValue {
        action,
        brightness: final_value
    });
}

fn parse_barg_from_regex(original: &String, captures: Captures, logger: &Logger) -> Result<BArg, String> {

    logger.log_verbose_info(format!("Parsing brightness argument: `{}`", original));

    let scope: BScope = match captures.get(2) {
        None => BScope::All,
        Some(s) => match s.as_str() {
            "*" | "all" => BScope::All,
            ss => match ss.parse::<usize>() {
                Ok(num) => BScope::Index(num),
                Err(err) => match err.kind() {
                    IntErrorKind::PosOverflow => return Err(format!("Index `{}` is too big", ss)),
                    _ => return Err(format!("Index `{}` is not a number", ss))
                }
            }
        }
    };

    let mut action: BAction = match captures.get(4) {
        None => BAction::Set,
        Some(a) => match a.as_str() {
            "+" => BAction::Inc,
            "-" => BAction::Dec,
            _ => BAction::Set
        }
    };

    // check if min|max values are provided
    let min_or_max: Option<usize> = match captures.get(5) {
        None => return Err("No brightness string provided".into()),
        Some(m) => {
            let m_str: &str = m.as_str();
            if MAX_NAMES.contains(&m_str) {

                let val: Option<usize> = match action {
                    BAction::Dec => Some(0),
                    _ => Some(100)
                };

                action = BAction::Set;
                val
            } else if MIN_NAMES.contains(&m_str) {
                Some(1)
            } else {
                None
            }
        }
    };
    if let Some(min_or_max_value) = min_or_max {
        logger.log_verbose_info(format!("Brightness argument `{}` parsed successfully", original));

        return Ok(BArg {
            scope,
            value: BValue {
                action,
                brightness: min_or_max_value
            }
        });
    };

    // no min|max values are provided
    let mut value: usize = match captures.get(6) {
        None => return Err("No brightness value provided".into()),
        Some(v) => match v.as_str().parse::<usize>() {
            Ok(vv) => vv,
            Err(err) => match err.kind() {
                IntErrorKind::PosOverflow => return Err(format!("Brightness value `{}` is too big", v.as_str())),
                _ => return Err(format!("Brightness value `{}` is not valid", v.as_str()))
            }
        }
    };
    if value > 100 {
        logger.log_verbose_warn(format!("Brightness value `{}` is bigger than 100, clamping to 100", value));
        value = 100;
    }

    let denominator: Option<usize> = match captures.get(8) {
        None => None,
        Some(d) => match d.as_str().parse::<usize>() {
            Ok(dd) => Some(dd),
            Err(err) => match err.kind() {
                IntErrorKind::PosOverflow => return Err(format!("Denominator value `{}` is too big", d.as_str())),
                _ => return Err(format!("Denominator value `{}` is not valid", d.as_str()))
            }
        }
    };

    let mut final_value: usize = {
        if let Some(denominator_value) = denominator {
            if denominator_value == 0 {
                return Err("Denominator value is `0`, cannot divide by zero".into());
            } else {

                // is the percent sign provided
                if captures.get(9).is_some() {
                    value * 100 / denominator_value
                } else {
                    value / denominator_value
                }

            }
        } else {
            value
        }
    };

    if final_value > 100 {
        logger.log_verbose_warn(format!("Brightness value `{}` is bigger than 100, clamping to 100", final_value));
        final_value = 100;
    }

    logger.log_verbose_info(format!("Brightness argument `{}` parsed successfully", original));
    return Ok(BArg {
        scope,
        value: BValue {
            action,
            brightness: final_value
        }
    });
}
