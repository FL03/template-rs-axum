/*
    Appellation: pzzld-sdk <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # pzzld-sdk
//! 
//! 
pub use self::{
    config::{Context, Settings},
    platform::{Application, Server},
    primitives::*,
    types::prelude::*,
    utils::*,
};

pub(crate) mod primitives;
pub(crate) mod utils;

pub mod api;
pub mod config;
pub mod models;
pub mod platform;
pub mod types;

pub mod prelude {
    pub use crate::config::Settings;
    pub use crate::consts::*;
    pub use crate::platform::prelude::*;
    pub use crate::types::prelude::*;
    pub use crate::utils::*;
}
