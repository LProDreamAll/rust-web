use tracing::Level;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Logging configuration
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    /// Log level (TRACE, DEBUG, INFO, WARN, ERROR)
    pub level: Level,
    /// Output JSON format
    pub json_format: bool,
    /// Optional log directory for file output
    pub log_dir: Option<String>,
    /// Colorize terminal output
    pub colorize: bool,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: Level::INFO,
            json_format: false,
            log_dir: None,
            colorize: true,
        }
    }
}

impl LoggingConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    pub fn json_format(mut self, enabled: bool) -> Self {
        self.json_format = enabled;
        self
    }

    pub fn log_dir(mut self, dir: impl Into<String>) -> Self {
        self.log_dir = Some(dir.into());
        self
    }

    pub fn colorize(mut self, enabled: bool) -> Self {
        self.colorize = enabled;
        self
    }
}

/// Initialize logging with the given configuration
pub fn init_logging(config: LoggingConfig) {
    let level_filter = match config.level {
        Level::TRACE => "trace",
        Level::DEBUG => "debug",
        Level::INFO => "info",
        Level::WARN => "warn",
        Level::ERROR => "error",
    };

    // Create env filter (can be overridden by RUST_LOG env var)
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level_filter));

    // Console layer
    let console_layer = fmt::layer()
        .with_target(false)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_ansi(config.colorize);

    // Build the subscriber
    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer);

    // Initialize
    subscriber.init();
}

/// Initialize logging with default settings
pub fn init_default() {
    init_logging(LoggingConfig::default());
}

/// Parse log level from string
pub fn parse_level(level: &str) -> Option<Level> {
    match level.to_lowercase().as_str() {
        "trace" => Some(Level::TRACE),
        "debug" => Some(Level::DEBUG),
        "info" => Some(Level::INFO),
        "warn" => Some(Level::WARN),
        "error" => Some(Level::ERROR),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging_config_default() {
        let config = LoggingConfig::default();
        assert_eq!(config.level, Level::INFO);
        assert!(!config.json_format);
        assert!(config.colorize);
        assert!(config.log_dir.is_none());
    }

    #[test]
    fn test_logging_config_builder() {
        let config = LoggingConfig::new()
            .level(Level::DEBUG)
            .json_format(true)
            .log_dir("/var/log")
            .colorize(false);

        assert_eq!(config.level, Level::DEBUG);
        assert!(config.json_format);
        assert!(!config.colorize);
        assert_eq!(config.log_dir, Some("/var/log".to_string()));
    }

    #[test]
    fn test_parse_level() {
        assert_eq!(parse_level("trace"), Some(Level::TRACE));
        assert_eq!(parse_level("DEBUG"), Some(Level::DEBUG));
        assert_eq!(parse_level("Info"), Some(Level::INFO));
        assert_eq!(parse_level("warn"), Some(Level::WARN));
        assert_eq!(parse_level("ERROR"), Some(Level::ERROR));
        assert_eq!(parse_level("invalid"), None);
    }
}
