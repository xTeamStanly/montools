use clap::{ArgAction, Arg, ArgMatches};

mod parser;
mod cli;
mod monitor;
mod progressbar;

use cli::params::{self, Arguments};
use log::{error, debug};
use logger::Logger;
use crate::monitor::apply_arguments;

fn main() {

    // quickfix: clap color errors
    // clap can print error message in color
    // before logger even starts
    if let Some(_) = std::env::args_os().find(|arg| arg.to_ascii_lowercase() == "--nocolor") {
        std::env::set_var("NO_COLOR", "1");
        std::env::set_var("CLICOLOR", "0");
    }

    let cli_matches: ArgMatches = clap::command!()
        .args_conflicts_with_subcommands(true)
        .args([
            Arg::new(params::ARG_BARGS_ID)
                .allow_negative_numbers(true)
                .value_name(params::ARG_BARGS_NAME)
                .help(params::ARG_BARGS_HELP.as_str())
                .action(ArgAction::Append),

            // --------------------------------------------- FLAGS ---------------------------------------------
            Arg::new(params::FLAG_ZERO_ID)
                .value_name(params::FLAG_ZERO_NAME)
                .short(params::FLAG_ZERO_SHORT_NAME)
                .long(params::FLAG_ZERO_LONG_NAME)
                .help(params::FLAG_ZERO_HELP)
                .action(ArgAction::SetTrue)
                .global(true),

            Arg::new(params::FLAG_COLOR_ID)
                .value_name(params::FLAG_COLOR_NAME)
                .long(params::FLAG_COLOR_LONG_NAME)
                .help(params::FLAG_COLOR_HELP)
                .action(ArgAction::SetFalse)
                .global(true),

            Arg::new(params::FLAG_VERBOSE_ID)
                .value_name(params::FLAG_VERBOSE_NAME)
                .short(params::FLAG_VERBOSE_SHORT_NAME)
                .long(params::FLAG_VERBOSE_LONG_NAME)
                .help(params::FLAG_VERBOSE_HELP)
                .action(ArgAction::SetTrue)
                .global(true),
            // -------------------------------------------------------------------------------------------------

            Arg::new(params::ARG_PROGRESSBAR_LENGTH_ID)
                .value_name(params::ARG_PROGRESSBAR_LENGTH_NAME)
                .short(params::ARG_PROGRESSBAR_LENGTH_SHORT_NAME)
                .long(params::ARG_PROGRESSBAR_LENGTH_LONG_NAME)
                .help(params::ARG_PROGRESSBAR_LENGTH_HELP)
                .default_value(params::ARG_PROGRESSBAR_LENGTH_DEFAULT_STR)
                .action(ArgAction::Set)
                .global(true),

            Arg::new(params::ARG_PROGRESSBAR_STYLE_ID)
                .value_name(params::ARG_PROGRESSBAR_STYLE_NAME)
                .short(params::ARG_PROGRESSBAR_STYLE_SHORT_NAME)
                .long(params::ARG_PROGRESSBAR_STYLE_LONG_NAME)
                .help(params::ARG_PROGRESSBAR_STYLE_HELP.as_str())
                .global(true)
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

    // extract brigntness arguments
    let args: Arguments = match Arguments::try_from(&cli_matches) {
        Ok(a) => a,
        Err(err) => {
            error!("{}", err);
            debug!("Exiting...");
            return;
        }
    };

    // apply brigntness arguments
    if let Err(err) = apply_arguments(args) {
        error!("{}", err);
    };

    debug!("Exiting...");
}
