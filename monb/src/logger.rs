#![allow(dead_code)]

use owo_colors::OwoColorize;

#[derive(Debug)]
pub struct Logger {
    pub verbose: bool
}

enum Severity {
    Debug,
    Information,
    Warning,
    Error
}

impl Logger {
    fn log<T: Into<String>>(&self, input: T, severity: Severity) {
        let message: String = input.into();
        match severity {
            Severity::Debug => println!("{} {}", "[DBG] ".if_supports_color(owo_colors::Stream::Stdout, |x| x.cyan()), message),
            Severity::Information => println!("{} {}", "[INFO]".if_supports_color(owo_colors::Stream::Stdout, |x| x.green()), message),
            Severity::Warning => println!("{} {}", "[WARN]".if_supports_color(owo_colors::Stream::Stdout, |x| x.yellow()), message),
            Severity::Error => println!("{} {}", "[ERR] ".if_supports_color(owo_colors::Stream::Stdout, |x| x.red()), message)
        }
    }

    pub fn log_debug<T: Into<String>>(&self, input: T) { self.log::<T>(input, Severity::Debug); }
    pub fn log_info<T: Into<String>>(&self, input: T) { self.log::<T>(input, Severity::Information); }
    pub fn log_warn<T: Into<String>>(&self, input: T) { self.log::<T>(input, Severity::Warning); }
    pub fn log_error<T: Into<String>>(&self, input: T) { self.log::<T>(input, Severity::Error); }

    pub fn log_verbose_debug<T: Into<String>>(&self, input: T) {
        if self.verbose {
            self.log::<T>(input, Severity::Debug);
        }
    }
    pub fn log_verbose_info<T: Into<String>>(&self, input: T) {
        if self.verbose {
            self.log::<T>(input, Severity::Information);
        }
    }
    pub fn log_verbose_warn<T: Into<String>>(&self, input: T) {
        if self.verbose {
            self.log::<T>(input, Severity::Warning);
        }
    }
    pub fn log_verbose_error<T: Into<String>>(&self, input: T) {
        if self.verbose {
            self.log::<T>(input, Severity::Error);
        }
    }
}
