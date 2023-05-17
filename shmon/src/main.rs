use clap::{Arg, ArgAction, ArgMatches};
use cli::arguments::{DURATION_ARGUMENT, Duration};
use cli::flags::{Flags, COLOR, VERBOSE};
use cli::{arguments, flags};
use log::error;
use log::LevelFilter::Trace;
use logger::Logger;

mod cli;
mod monitor;
mod parser;

use monitor::execute_duration;
use parser::parse_duration;

static LOGGER: Logger = Logger;
fn main() {

    // check for logger errors
    if let Err(err) = log::set_logger(&LOGGER) {
        eprintln!("{}", err);
        return;
    } else {
        log::set_max_level(Trace);
    };

    let cli_matches: ArgMatches = clap::command!()
        .args([
            Arg::new(arguments::DURATION_ARGUMENT)
                .value_name("DURATION")
                .action(ArgAction::Set)
                .num_args(0..=1)
                .help("Time duration before the displays are turned off.\nLooks like `[NUMBER][UNIT]`.\nUNIT includes `s`, `sec`, `m`, `min`, `h` representing seconds, minutes and hours, respectively"),

            // --------------------------------------------- FLAGS ---------------------------------------------
            Arg::new(flags::COLOR)
                .long("nocolor")
                .action(ArgAction::SetFalse)
                .global(true)
                .help("Disables colored terminal output"),

            Arg::new(flags::VERBOSE)
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .global(true)
                .help("Prints more information during execution"),
        ]).get_matches();

    // extract flags
    let cli_flags: Flags = Flags {
        support_color: cli_matches.get_flag(COLOR),
        verbose: cli_matches.get_flag(VERBOSE)
    };

    // disable colors, `owo-colors`
    if cli_flags.support_color == false {
        std::env::set_var("NO_COLOR", "");
        std::env::set_var("CLICOLOR", "0");
    }

    // extract argument
    let raw_duration_argument: Option<String> = match cli_matches.get_one::<String>(DURATION_ARGUMENT) {
        None => None,
        Some(r) => Some(r.trim().to_lowercase())
    };

    // parse duration
    let parsed_duration_argument: Duration = match parse_duration(raw_duration_argument, &cli_flags) {
        Ok(d) => d,
        Err(err) => {
            error!("{}", err);
            return;
        }
    };

    // execute duration
    if let Err(err) = execute_duration(parsed_duration_argument, &cli_flags) {
        error!("{}", err);
    };
}
