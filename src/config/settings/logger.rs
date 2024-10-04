/*
    Appellation: logger <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::log_level::*;

fn _fmt_rust_log(name: impl ToString, level: tracing::Level) -> String {
    format!("RUST_LOG={name}={level}", name = name.to_string())
}

pub(crate) fn init_tracing(level: tracing::Level) {
    use tracing_subscriber::{fmt::time, EnvFilter};

    tracing_subscriber::fmt()
        .compact()
        .with_ansi(true)
        .with_env_filter(EnvFilter::new(&format!(
            "{name}={level}",
            name = crate::APP_NAME
        )))
        .with_max_level(level)
        .with_target(false)
        .with_timer(time::uptime())
        .init();
    tracing::info!("Successfully initialized the tracing layers...");
}

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct LoggerConfig {
    pub level: LogLevel,
}

impl LoggerConfig {
    pub fn init_tracing(&self) {
        let level = self.level.as_tracing_level();
        if let Some(level) = level {
            init_tracing(level)
        }
    }
}

/*
 ************* Implementations *************
*/
impl core::fmt::Display for LoggerConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(&serde_json::to_string(self).unwrap())
    }
}

unsafe impl Send for LoggerConfig {}

unsafe impl Sync for LoggerConfig {}

mod log_level {
    #[derive(
        Clone,
        Copy,
        Debug,
        Default,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        serde::Deserialize,
        serde::Serialize,
        strum::AsRefStr,
        strum::Display,
        strum::EnumCount,
        strum::EnumIs,
        strum::EnumIter,
        strum::EnumString,
        strum::VariantNames,
    )]
    #[repr(u8)]
    #[serde(rename_all = "lowercase")]
    #[strum(serialize_all = "lowercase")]
    pub enum LogLevel {
        Debug,
        Error,
        Info,
        Trace,
        Warn,
        #[default]
        Off,
    }

    impl LogLevel {
        pub fn from_tracing(level: tracing::Level) -> Self {
            use tracing::Level;

            match level {
                Level::DEBUG => Self::Debug,
                Level::ERROR => Self::Error,
                Level::INFO => Self::Info,
                Level::TRACE => Self::Trace,
                Level::WARN => Self::Warn,
            }
        }

        pub fn debug() -> Self {
            Self::Debug
        }

        pub fn info() -> Self {
            Self::Info
        }

        pub fn warn() -> Self {
            Self::Warn
        }

        pub fn error() -> Self {
            Self::Error
        }

        pub fn off() -> Self {
            Self::Off
        }

        pub fn as_tracing_level(&self) -> Option<tracing::Level> {
            use tracing::Level;

            match self {
                Self::Debug => Some(Level::DEBUG),
                Self::Error => Some(Level::ERROR),
                Self::Info => Some(Level::INFO),
                Self::Trace => Some(Level::TRACE),
                Self::Warn => Some(Level::WARN),
                Self::Off => None,
            }
        }

        pub fn as_tracing_filter(&self) -> tracing_subscriber::filter::LevelFilter {
            use tracing_subscriber::filter::LevelFilter;

            match self {
                Self::Debug => LevelFilter::DEBUG,
                Self::Error => LevelFilter::ERROR,
                Self::Info => LevelFilter::INFO,
                Self::Trace => LevelFilter::TRACE,
                Self::Warn => LevelFilter::WARN,
                Self::Off => LevelFilter::OFF,
            }
        }
    }

    impl From<LogLevel> for config::Value {
        fn from(level: LogLevel) -> Self {
            level.to_string().into()
        }
    }

    impl From<tracing::Level> for LogLevel {
        fn from(level: tracing::Level) -> Self {
            Self::from_tracing(level)
        }
    }

    unsafe impl Send for LogLevel {}

    unsafe impl Sync for LogLevel {}
}
