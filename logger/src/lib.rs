use log::{Log, Level, LevelFilter, Metadata, SetLoggerError};
use colored::Colorize;

#[derive(Debug)]
pub struct Logger {
    verbose: bool,
    colored: bool
}

impl Default for Logger {
    fn default() -> Self {
        Logger {
            verbose: false,
            colored: false
        }
    }
}

impl Logger {
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn colored(mut self, colored: bool) -> Self {
        self.colored = colored;
        self
    }

    pub fn get_max_log_level(&self) -> LevelFilter {
        if self.verbose {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        }
    }

    pub fn init_logger(self) -> Result<(), SetLoggerError> {
        let boxed_logger: Box<Self>  = Box::new(self);
        let max_log_level: LevelFilter = boxed_logger.get_max_log_level();
        if let Err(err) = log::set_boxed_logger(boxed_logger) {
            return Err(err);
        } else {
            log::set_max_level(max_log_level);
            return Ok(());
        }
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.get_max_log_level()
    }

    fn flush(&self) {}

    fn log(&self, record: &log::Record) {
        if self.colored {
            match record.level() {
                Level::Debug => println!("{} {}", "[DBG]"  .cyan(),    record.args()),
                Level::Info =>  println!("{} {}", "[INF]"  .green(),   record.args()),
                Level::Warn =>  println!("{} {}", "[WRN]"  .yellow(),  record.args()),
                Level::Trace => println!("{} {}", "[TRC]"  .blue(),    record.args()),
                Level::Error => println!("{} {}", "[ERR]"  .red(),     record.args())
            }
        } else {
            match record.level() {
                Level::Debug => println!("{} {}", "[DBG]", record.args()),
                Level::Info =>  println!("{} {}", "[INF]", record.args()),
                Level::Warn =>  println!("{} {}", "[WRN]", record.args()),
                Level::Trace => println!("{} {}", "[TRC]", record.args()),
                Level::Error => println!("{} {}", "[ERR]", record.args())
            }
        }
    }
}
