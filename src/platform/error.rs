/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub type PlatformResult<T = ()> = std::result::Result<T, PlatformError>;

#[derive(Debug, Default, thiserror::Error)]
pub enum PlatformError {
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[default]
    #[error("Unknown error")]
    Unknown,
}
