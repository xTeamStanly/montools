use owo_colors::OwoColorize;

enum Severity {
    Debug,
    Information,
    Warning,
    Error
}

fn log<T: Into<String>>(input: T, severity: Severity) {
    let message: String = input.into();
    match severity {
        Severity::Debug => println!("{} {}", "[DBG] ".cyan(), message),
        Severity::Information => println!("{} {}", "[INFO]".green(), message),
        Severity::Warning => println!("{} {}", "[WARN]".yellow(), message),
        Severity::Error => println!("{} {}", "[ERR] ".red(), message)
    }
}

pub fn log_debug<T: Into<String>>(input: T) { log(input, Severity::Debug); }
pub fn log_info<T: Into<String>>(input: T) { log(input, Severity::Information); }
pub fn log_warn<T: Into<String>>(input: T) { log(input, Severity::Warning); }
pub fn log_error<T: Into<String>>(input: T) { log(input, Severity::Error); }