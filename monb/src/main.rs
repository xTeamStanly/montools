use clap::{Command, ArgAction, Arg};

mod parser;
mod cli;
mod logger;
mod monitor;
mod progressbar;

use cli::flags;
use cli::flags::{Flags, ZERO, COLOR, VERBOSE};
use cli::arguments;
use parser::{default_command, get_command, set_command};

fn main() {
    let cli_matches = clap::command!()
        .args_conflicts_with_subcommands(true)
        .args([
            Arg::new(arguments::BRIGHTNESS_ARGUMENTS)
                .value_name("BRIGHTNESS ARGUMENTS STRING")
                .action(ArgAction::Append)
                .help("List of brightness argument strings"),

            // --------------------------------------------- FLAGS ---------------------------------------------
            Arg::new(flags::ZERO)
                .short('z')
                .long("zero")
                .action(ArgAction::SetTrue)
                .global(true)
                .help("Enables zero-based monitor enumeration"),

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
            // -------------------------------------------------------------------------------------------------

            Arg::new(arguments::PROGRESSBAR_LENGTH)
                .short('l')
                .long("length")
                .action(ArgAction::Set)
                .value_name("PROGRESSBAR LENGTH")
                .global(true)
                .default_value(arguments::DEFAULT_PROGRESSBAR_LENGTH_STR)
                .help(format!("Sets the length of a progressbar, measured in characters. Minimal value is {}. Maximal value is {}.", arguments::MIN_PROGRESSBAR_LENGTH, arguments::MAX_PROGRESSBAR_LENGTH)),
            Arg::new(arguments::PROGRESSBAR_STYLE)
                .short('s')
                .long("style")
                .value_name("PROGRESSBAR STYLE")
                .action(ArgAction::Set)
                .default_value(arguments::DEFAULT_PROGRESSBAR_STYLE)
                .global(true)
                .help("Sets the progressbar style. Possible styles: [ `wsl`, `classic`, `arrow`, `wsl_arrow`, `filled` ]")
        ])
        .subcommands([

            // --------------------------------------------- GET COMMAND ---------------------------------------------
            Command::new("get")
                .about("Shows monitor information")
                .args_conflicts_with_subcommands(true)
                .arg(
                    Arg::new(arguments::MONITOR_INDICES)
                        .value_name("MONITOR INDICES")
                        .action(ArgAction::Append)
                        .help("Displays monitor information base on kist of monitor indices"),
                )
                .subcommand(
                    Command::new("all")
                        .visible_alias("*")
                        .about("Shows information about all monitors, same effect as calling the executable without arguments")
                ),

            // --------------------------------------------- SET COMMAND ---------------------------------------------
            Command::new("set")
                .about("Sets the monitor brightness")
                .args_conflicts_with_subcommands(true)
                .arg(
                    Arg::new(arguments::BRIGHTNESS_ARGUMENTS)
                        .value_name("BRIGHTNESS ARGUMENTS STRING")
                        .action(ArgAction::Append)
                        .help("List of brightness argument strings")
                        .required(true)
                )
                .subcommands([
                    Command::new("all")
                        .visible_alias("*")
                        .about("Sets the monitor brightness for all detected monitors")
                        .args([
                            Arg::new(arguments::BRIGHTNESS_VALUE)
                                .value_name("BRIGHTNESS VALUE")
                                .action(ArgAction::Set)
                                .help("Brightness value string")
                                .required(true)
                        ])
                ])
        ]).get_matches();

    // extract flags
    let cli_flags: Flags = Flags {
        zero: cli_matches.get_flag(ZERO),
        color: cli_matches.get_flag(COLOR),
        verbose: cli_matches.get_flag(VERBOSE)
    };
    if !cli_flags.color {
        std::env::set_var("NO_COLOR", "");
    }

    // setup logger
    let logger: crate::logger::Logger = crate::logger::Logger {
        verbose: cli_flags.verbose
    };

    match cli_matches.subcommand() {
        Some(("get", arguments)) => get_command(arguments, &cli_flags, &logger),
        Some(("set", arguments)) => set_command(arguments, &cli_flags, &logger),

        Some((name, _)) => logger.log_error(format!("Unrecognized command `{}`", name)),
        None => default_command(&cli_matches, &cli_flags, &logger)
    };

    logger.log_verbose_info("Exiting...");
}
