use std::sync::Mutex;

/// Log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// Global logger state
static LOGGER: Mutex<Logger> = Mutex::new(Logger {
    level: LogLevel::Info,
    quiet: false,
});

struct Logger {
    level: LogLevel,
    quiet: bool,
}

/// Initialize the logger
pub fn init(level: LogLevel, quiet: bool) {
    if let Ok(mut logger) = LOGGER.lock() {
        logger.level = level;
        logger.quiet = quiet;
    }
}

/// Log a debug message
pub fn debug(message: &str) {
    log(LogLevel::Debug, message);
}

/// Log an info message
pub fn info(message: &str) {
    log(LogLevel::Info, message);
}

/// Log a warning message
pub fn warn(message: &str) {
    log(LogLevel::Warn, message);
}

/// Log an error message
pub fn error(message: &str) {
    log(LogLevel::Error, message);
}

fn log(level: LogLevel, message: &str) {
    if let Ok(logger) = LOGGER.lock() {
        if logger.quiet && level < LogLevel::Error {
            return;
        }

        if level >= logger.level {
            let prefix = match level {
                LogLevel::Debug => "[DEBUG]",
                LogLevel::Info => "[INFO]",
                LogLevel::Warn => "[WARN]",
                LogLevel::Error => "[ERROR]",
            };

            eprintln!("{prefix} {message}");
        }
    }
}
