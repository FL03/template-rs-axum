/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{database::*, logger::*, mode::*, server::*};

pub(crate) mod database;
pub(crate) mod logger;
pub(crate) mod mode;
pub(crate) mod server;

use config::builder::{ConfigBuilder, DefaultState};

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct Settings {
    pub database: DatabaseCnf,
    pub logger: LoggerConfig,
    pub mode: Mode,
    pub server: ServerConfig,
    pub version: String,
}

impl Settings {
    pub fn debug() -> Self {
        Self {
            database: DatabaseCnf::default(),
            logger: LoggerConfig::default(),
            mode: Mode::development(),
            server: ServerConfig::default(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    fn builder_base() -> Result<ConfigBuilder<DefaultState>, config::ConfigError> {
        use crate::defaults::*;
        // initialize the builder with default values
        let mut builder = config::Config::builder()
            .set_default("database.url", DEFAULT_DB_URL)?
            .set_default("logger.level", LogLevel::default())?
            .set_default("mode", Mode::default())?
            .set_default("server.addr.host", LOCALHOST)?
            .set_default("server.addr.port", DEFAULT_PORT)?;
        // add sources
        builder = builder
            .add_source(config::Environment::with_prefix("PZZLD").separator("_"))
            .add_source(config::Environment::with_prefix("DATABASE").separator("_"))
            .add_source(config::File::with_name(".config/Puzzled.toml").required(false))
            .add_source(config::File::with_name(".config/default.config.toml").required(false))
            .add_source(config::File::with_name(".config/prod.config.toml").required(false));
        // setup overrides
        builder = builder
            .set_override("version", env!("CARGO_PKG_VERSION"))?
            .set_override_option("mode", std::env::var("MODE").ok())?
            .set_override_option("database.url", std::env::var("DATABASE_URL").ok())?
            .set_override_option("server.addr.host", std::env::var("SERVER_HOST").ok())?
            .set_override_option("server.addr.port", std::env::var("SERVER_PORT").ok())?;
        Ok(builder)
    }

    pub fn build() -> Result<Self, config::ConfigError> {
        Self::builder_base()?.build()?.try_deserialize()
    }

    pub fn build_from_file(file: &str) -> Result<Self, config::ConfigError> {
        Self::builder_base()?
            .add_source(config::File::with_name(file))
            .build()?
            .try_deserialize()
    }

    pub fn build_from_pattern(pattern: &str) -> Result<Self, config::ConfigError> {
        Self::builder_base()?
            .add_source(crate::config::collect_configurations(pattern, false))
            .build()?
            .try_deserialize()
    }

    pub const fn logger(&self) -> &LoggerConfig {
        &self.logger
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub const fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub fn server_mut(&mut self) -> &mut ServerConfig {
        &mut self.server
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn set_version(&mut self, version: impl ToString) {
        self.version = version.to_string();
    }
}

/*
 ************* Implementations *************
*/
impl Default for Settings {
    fn default() -> Self {
        Self::debug()
    }
}
impl core::fmt::Display for Settings {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(&serde_json::to_string(self).unwrap())
    }
}

unsafe impl core::marker::Send for Settings {}

unsafe impl core::marker::Sync for Settings {}
