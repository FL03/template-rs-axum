/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::PlatformError;
use crate::config::Settings;

pub struct PlatformState {
    pub(crate) cnf: Settings,
    pub(crate) db: sqlx::AnyPool,
}

impl PlatformState {
    pub async fn new(cnf: Settings) -> Result<Self, PlatformError> {
        let db = cnf.database.connect().await?;
        Ok(Self { cnf, db })
    }

    pub fn db(&self) -> &sqlx::AnyPool {
        &self.db
    }

    pub const fn settings(&self) -> &Settings {
        &self.cnf
    }
}
