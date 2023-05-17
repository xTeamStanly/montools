use clap::{Arg, ArgAction, ArgMatches};
use cli::arguments::{DURATION_ARGUMENT, Duration};
use cli::flags::Flags;
use cli::{arguments, flags};
use log::error;
use log::LevelFilter::Trace;
use logger::Logger;

mod cli;
mod monitor;
mod parser;

use monitor::execute_duration;
use parser::parse_duration;

fn main() {

    let cli_matches: ArgMatches = clap::command!()
        .args([
            Arg::new(arguments::DURATION_ARGUMENT)
                .value_name("DURATION")
                .action(ArgAction::Set)
                .num_args(0..=1)
                .help("Time duration before the displays are turned off.\nFormat: `[NUMBER][UNIT]`.\nUNIT includes `s`, `sec`, `m`, `min`, `h` representing seconds, minutes and hours, respectively"),

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
                .help("Prints debug information during execution"),
        ]).get_matches();

    // extract flags
    let cli_flags: Flags = Flags::from(&cli_matches);

    // init logger and check for errors
    let logger: Box<Logger> = Box::new(Logger { verbose: cli_flags.verbose });
    if let Err(err) = log::set_boxed_logger(logger) {
        eprintln!("{}", err);
        return;
    } else {
        log::set_max_level(Trace);
    };

    // disable colors
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
    let parsed_duration_argument: Duration = match parse_duration(raw_duration_argument) {
        Ok(d) => d,
        Err(err) => {
            error!("{}", err);
            return;
        }
    };

    // execute duration
    if let Err(err) = execute_duration(parsed_duration_argument) {
        error!("{}", err);
    };
}
