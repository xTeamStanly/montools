use clap::{Arg, ArgAction, ArgMatches};
use cli::params::{self, Duration};
use log::error;

use logger::Logger;

mod cli;
mod monitor;
mod parser;

use monitor::execute_duration;
use parser::parse_duration;

fn main() {

    // quickfix: clap color errors
    // clap can print error message in color
    // before logger even starts
    if let Some(_) = std::env::args_os().find(|arg| arg.to_ascii_lowercase() == "--nocolor") {
        std::env::set_var("NO_COLOR", "1");
        std::env::set_var("CLICOLOR", "0");
    }

    let cli_matches: ArgMatches = clap::command!()
        .args([
            Arg::new(params::ARG_DURATION_ID)
                .value_name(params::ARG_DURATION_NAME)
                .help(params::ARG_DURATION_HELP)
                .action(ArgAction::Set)
                .num_args(0..=1),

            // --------------------------------------------- FLAGS ---------------------------------------------
            Arg::new(params::FLAG_COLOR_ID)
                .value_name(params::FLAG_COLOR_NAME)
                .long(params::FLAG_COLOR_LONG_NAME)
                .help(params::FLAG_COLOR_HELP)
                .action(ArgAction::SetFalse)
                .global(true),

            Arg::new(params::FLAG_VERBOSE_ID)
                .value_name(params::FLAG_VERBOSE_NAME)
                .action(ArgAction::SetTrue)
                .short(params::FLAG_VERBOSE_SHORT_NAME)
                .long(params::FLAG_VERBOSE_LONG_NAME)
                .help(params::FLAG_VERBOSE_HELP)
                .global(true),
        ]).get_matches();

    // init logger and check for errors
    if let Err(err) = Logger::default()
        .colored(cli_matches.get_flag(params::FLAG_COLOR_ID))
        .verbose(cli_matches.get_flag(params::FLAG_VERBOSE_ID))
        .init_logger()
    {
        eprintln!("{}", err);
        return;
    }

    // extract argument
    let raw_duration_argument: Option<String> = cli_matches
        .get_one::<String>(params::ARG_DURATION_ID)
        .map(|x| x.trim().to_lowercase());

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
