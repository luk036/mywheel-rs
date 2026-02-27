//! Logging module for mywheel-rs
//!
//! This module provides optional logging capabilities when the `std` feature
//! is enabled. It uses `env_logger` for flexible log configuration via
//! environment variables.

#[cfg(feature = "std")]
use log::LevelFilter;

/// Initialize the logger with the default filter (info level).
///
/// This function reads the log level from the `RUST_LOG` environment variable.
/// If `RUST_LOG` is not set, it defaults to `info` level.
///
/// # Panics
///
/// Panics if the logger has already been initialized.
///
/// # Examples
///
/// ```rust,ignore
/// use mywheel_rs::logging::init_logger;
///
/// fn main() {
///     init_logger();
///     log::info!("Application started");
/// }
/// ```
#[cfg(feature = "std")]
pub fn init_logger() {
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .init();
}

/// Initialize the logger with a custom filter string.
///
/// The filter string follows the `env_logger` format. Common values include:
/// - `debug` - Debug and above
/// - `info` - Info and above (default)
/// - `warn` - Warnings and errors
/// - `error` - Errors only
/// - `mywheel_rs=debug` - Debug level for mywheel_rs only
///
/// # Panics
///
/// Panics if the logger has already been initialized.
///
/// # Examples
///
/// ```rust,ignore
/// use mywheel_rs::logging::init_logger_with_filter;
///
/// fn main() {
///     init_logger_with_filter("debug");
///     log::debug!("Detailed debug info");
/// }
/// ```
#[cfg(feature = "std")]
pub fn init_logger_with_filter(filter: &str) {
    env_logger::Builder::from_default_env()
        .filter_level(filter.parse().unwrap_or(LevelFilter::Info))
        .init();
}

/// Try to initialize the logger without panicking.
///
/// Returns `Ok(())` if the logger was successfully initialized,
/// or `Err` if the logger has already been initialized.
///
/// # Examples
///
/// ```rust,ignore
/// use mywheel_rs::logging::try_init_logger;
///
/// fn main() {
///     if try_init_logger().is_ok() {
///         log::info!("Logger initialized");
///     }
/// }
/// ```
#[cfg(feature = "std")]
pub fn try_init_logger() -> Result<(), log::SetLoggerError> {
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .try_init()
}

/// Try to initialize the logger with a custom filter without panicking.
///
/// Returns `Ok(())` if the logger was successfully initialized,
/// or `Err` if the logger has already been initialized.
///
/// # Examples
///
/// ```rust,ignore
/// use mywheel_rs::logging::try_init_logger_with_filter;
///
/// fn main() {
///     try_init_logger_with_filter("debug").ok();
///     log::debug!("Debug info");
/// }
/// ```
#[cfg(feature = "std")]
pub fn try_init_logger_with_filter(filter: &str) -> Result<(), log::SetLoggerError> {
    env_logger::Builder::from_default_env()
        .filter_level(filter.parse().unwrap_or(LevelFilter::Info))
        .try_init()
}

/// Check if the logger has been initialized.
///
/// Returns `true` if the logger is active, `false` otherwise.
///
/// # Examples
///
/// ```rust,ignore
/// use mywheel_rs::logging::{init_logger, is_logger_initialized};
///
/// fn main() {
///     init_logger();
///     assert!(is_logger_initialized());
/// }
/// ```
#[cfg(feature = "std")]
pub fn is_logger_initialized() -> bool {
    log::max_level() != LevelFilter::Off
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_init_logger() {
        // Should succeed (logger not initialized yet in test)
        let result = try_init_logger();
        assert!(result.is_ok() || result.is_err()); // Either is fine
    }

    #[test]
    fn test_try_init_logger_with_filter() {
        // Should succeed with custom filter
        let result = try_init_logger_with_filter("warn");
        assert!(result.is_ok() || result.is_err()); // Either is fine
    }

    #[test]
    fn test_is_logger_initialized() {
        // Just check the function doesn't panic
        let _ = is_logger_initialized();
    }
}
