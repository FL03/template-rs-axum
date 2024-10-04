/*
    Appellation: config <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{context::*, settings::*, utils::*};

pub(crate) mod context;
pub(crate) mod settings;

pub(crate) type ConfigBuilder<S = config::builder::DefaultState> = config::ConfigBuilder<S>;

pub(crate) mod utils {
    use std::path::PathBuf;

    /// Type alias for [config::File]
    pub(crate) type ConfigFile<Src, Fmt> = config::File<Src, Fmt>;
    /// Type alias for a collection of [crate::ConfigFile]
    pub(crate) type ConfigFileVec = Vec<ConfigFile<config::FileSourceFile, config::FileFormat>>;

    /// A generic function wrapper extending glob::glob
    fn gather<F, T>(pattern: &str, f: F) -> Vec<T>
    where
        F: Copy + Fn(PathBuf) -> T,
    {
        glob::glob(pattern)
            .expect("Failed to collect files")
            .filter_map(|r| r.ok().map(f))
            .collect()
    }

    /// Attempts to collect configuration files, following the given pattern, into a ConfigFileVec
    pub fn collect_configurations(pattern: &str, required: bool) -> ConfigFileVec {
        gather(pattern, |p| ConfigFile::from(p).required(required))
    }
}
