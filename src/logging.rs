use log::{info, warn, error};
use env_logger;

/// Initializes the logger with the default configuration
pub fn init_logger() {
    env_logger::init();
}

/// Logs an informational message
pub fn log_info(message: &str) {
    info!("{}", message);
}

/// Logs a warning message
pub fn log_warning(message: &str) {
    warn!("{}", message);
}

/// Logs an error message
pub fn log_error(message: &str) {
    error!("{}", message);
}
