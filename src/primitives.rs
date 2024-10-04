/*
    Appellation: primitives <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused_imports)]
pub use self::consts::*;
pub(crate) use self::defaults::*;

pub mod consts {
    pub const APP_NAME: &str = "pzzld"; // env!("CARGO_PKG_NAME");
    pub const ENV_PREFIX: &str = "APP";

    pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

    pub const ASSETS_WORKDIR: &str = "assets";

    pub const CONFIG_WORKDIR: &str = ".config";
}

pub(crate) mod defaults {
    use crate::config::LogLevel;
    /// Default host address
    pub const LOCALHOST: &str = "127.0.0.1";
    /// Default port number
    pub const DEFAULT_PORT: u16 = 8080;
    /// Default database URL
    pub const DEFAULT_DB_URL: &str = "sqlite::memory:";
}
