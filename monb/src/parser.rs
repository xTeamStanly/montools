use std::collections::HashSet;
use std::num::IntErrorKind;
use std::str::FromStr;

use clap::parser::ValuesRef;
use regex::Captures;

use log::{debug, warn};

use crate::cli::regexes::{BARG_REGEX, SCOPE_GROUP, VALUE_GROUP, ACTION_GROUP, BRIGHTNESS_GROUP, DENOMINATOR_GROUP, PERCENTAGE_GROUP, MAX_GROUP, MIN_GROUP};
use crate::cli::params::{BArg, BScope, BValue, BAction, BArgs, Getter};

pub fn parse_bargs(possible_raw_bargs: Option<ValuesRef<String>>) -> Result<BArgs, String> {
    let raw_bargs: ValuesRef<String> = match possible_raw_bargs {
        Some(rb) => rb,
        None => return Ok(BArgs::default())
    };

    let mut parsed_bargs: Vec<BArg> = Vec::<BArg>::new();

    for potential_barg in raw_bargs {
        debug!("Parsing brightness argument: `{}`", potential_barg);

        let result: BArg = match BARG_REGEX.captures(potential_barg) {
            Some(c) => parse_barg_from_regex_captures(c)?,
            None => return Err(format!("Input `{}` is not a valid brightness argument", potential_barg))
        };
        debug!("Brightness argument `{}` parsed successfully into `{}`", potential_barg, result.to_string());
        parsed_bargs.push(result);
    }

    let mut global_get: bool = false; // used for merging indexed gets if global get is present; 1: 3: *: 2: --> *:
    let mut getter_bargs_scopes: HashSet<usize> = HashSet::<usize>::new();
    let mut getter_bargs: Vec<BScope> = Vec::<BScope>::new();

    let mut global_set: Option<BArg> = None; // used to put global barg at the end of the array
    let mut setter_bargs_scopes: HashSet<usize> = HashSet::<usize>::new();
    let mut setter_bargs: Vec<BArg> = Vec::<BArg>::new();

    for barg in parsed_bargs {

        if let BAction::Get = barg.value.action {
            if global_get {
                debug!("Global `getter` already set. Ignoring provided (`{}`) and previous `getters` global argument", barg.to_string());
                continue;
            }

            match barg.scope {
                BScope::Global => { global_get = true; },
                BScope::Index(index) => {
                    if !getter_bargs_scopes.contains(&index) {
                        getter_bargs_scopes.insert(index);
                        getter_bargs.push(barg.scope);
                    } else {
                        debug!("Duplicate `get` brightness argument for scope: `{}`", barg.scope.to_string())
                    }
                }
            }
        } else {

            match barg.scope {
                BScope::Global => {
                    if global_set.is_some() {
                        warn!("Global brightness argument already set. Ignoring provided global argument");
                    } else {
                        global_set = Some(barg);
                    }
                },

                BScope::Index(index) => {
                    if !setter_bargs_scopes.contains(&index) {
                        setter_bargs_scopes.insert(index);
                        setter_bargs.push(barg);
                    } else {
                        debug!("Duplicate brightness argument for scope: `{}`", index);
                    }
                }
            }
        }
    }

    if let Some(global_setter) = global_set {
        setter_bargs.push(global_setter);
    }

    let final_getter: Option<Getter> = if global_get {
        Some(Getter::Global)
    } else {
        if getter_bargs.len() != 0 {
            Some(Getter::Many(getter_bargs))
        } else {
            None
        }
    };

    return Ok(BArgs {
        getters: final_getter,
        setters: setter_bargs
    });
}

fn parse_brightness_value_from_str(brightness_raw: &str, is_fraction: bool) -> Result<usize, String> {
    let mut brightness: usize = match brightness_raw.parse::<usize>() {
        Ok(bv) => bv,
        Err(err) => match err.kind() {
            IntErrorKind::PosOverflow => return Err(format!("Brightness value `{}` is too big", brightness_raw)),
            _ => return Err(format!("Brightness value `{}` is not valid", brightness_raw))
        }
    };

    if brightness > 100 && !is_fraction {
        debug!("Brightness value `{}` is bigger than 100, clamping to 100", brightness_raw);
        brightness = 100;
    }

    return Ok(brightness);
}

fn parse_denominator_value_from_str(denominator_str: &str) -> Result<Option<usize>, String> {
    let denominator: usize = match denominator_str.parse::<usize>() {
        Ok(denom) => denom,
        Err(err) => match err.kind() {
            IntErrorKind::PosOverflow => return Err(format!("Denominator value `{}` is too big", denominator_str)),
            _ => return Err(format!("Denominator value `{}` is not valid", denominator_str))
        }
    };

    return Ok(Some(denominator));
}

fn calculate_final_value(brightness_value: usize, denominator: Option<usize>, percentage: bool) -> Result<usize, String> {
    if let Some(denominator_value) = denominator {

        if denominator_value == 0 {
            return Err("Denominator value is `0`, cannot divide by zero".into());
        }

        let mut final_value = if percentage {
            brightness_value * 100 / denominator_value
        } else {
            brightness_value / denominator_value
        };

        if final_value > 100 {
            debug!("Brightness value `{}` is bigger than 100, clamping to 100", final_value);
            final_value = 100;
        }

        return Ok(final_value);

    }

    return Ok(brightness_value);
}

fn parse_barg_from_regex_captures(captures: Captures) -> Result<BArg, String> {
    let scope: BScope = match captures.name(SCOPE_GROUP) {
        None => BScope::Global,
        Some(c) => BScope::from_str(c.as_str())?
    };

    if let None = captures.name(VALUE_GROUP) {
        return Ok(BArg {
            scope,
            value: BValue {
                action: BAction::Get,
                brightness: None
            }
        })
    };

    let action: BAction = match captures.name(ACTION_GROUP) {
        None => BAction::Set,
        Some(c) => BAction::from_str(c.as_str())?
    };

    // numeric brightness value
    if let Some(brightness_value_raw) = captures.name(BRIGHTNESS_GROUP) {

        // denominator value
        let denominator: Option<usize> = match captures.name(DENOMINATOR_GROUP) {
            None => None,
            Some(denom_raw) => parse_denominator_value_from_str(denom_raw.as_str())?
        };
        let is_fraction: bool = denominator.is_some();

        // main brightness value
        let brightness_value: usize = parse_brightness_value_from_str(brightness_value_raw.as_str(), is_fraction)?;

        let final_value: usize = calculate_final_value(brightness_value, denominator, captures.name(PERCENTAGE_GROUP).is_some())?;

        return Ok(BArg {
            scope,
            value: BValue {
                action,
                brightness: Some(final_value)
            }
        });
    }

    // +-----+--------+----------+
    // |     | Min    | Max      |
    // +-----+--------+----------+
    // | Inc | Inc(1) | Set(100) |
    // | Dec | Dec(1) | Set(0)   |
    // | Set | Set(0) | Set(100) |
    // +-----+--------+----------+

    // max brightness value
    if captures.name(MAX_GROUP).is_some() {
        let value: usize = match action {
            BAction::Dec => 0,
            _ => 100
        };

        return Ok(BArg {
            scope,
            value: BValue {
                action: BAction::Set,
                brightness: Some(value)
            }
        })
    };

    // min brightness value
    if captures.name(MIN_GROUP).is_some() {
        let value: usize = match action {
            BAction::Set => 0,
            _ => 1
        };

        return Ok(BArg {
            scope,
            value: BValue {
                action: BAction::Set,
                brightness: Some(value)
            }
        })
    };


    debug!("Should not happen!");
    return Err("Unrecognized brightness argument".into());
}
