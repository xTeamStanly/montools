use log::Log;
use log::Level::*;
use log::Metadata;
use colored::*;

pub struct Logger {
    pub verbose: bool
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= (if self.verbose { Debug } else { Info })
    }

    fn flush(&self) {}

    fn log(&self, record: &log::Record) {
        match record.level() {
            Debug => println!("{} {}", "[DBG] ".cyan(), record.args()),
            Info =>  println!("{} {}", "[INFO]".green(), record.args()),
            Warn =>  println!("{} {}", "[WARN]".yellow(), record.args()),
            Trace => println!("{} {}", "[TRC] ".blue(), record.args()),
            Error => println!("{} {}", "[ERR] ".red(), record.args())
        }
    }
}
