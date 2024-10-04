/*
    Appellation: database <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::DEFAULT_DB_URL;

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct DatabaseCnf {
    pub pool_size: Option<u32>,
    pub url: String,
}

impl DatabaseCnf {
    pub async fn connect<Db>(&self) -> sqlx::Result<sqlx::Pool<Db>>
    where
        Db: sqlx::Database,
    {
        tracing::debug!("Connecting to database: {}", self.url);
        sqlx::Pool::connect(&self.url).await
    }
}

impl Default for DatabaseCnf {
    fn default() -> Self {
        Self {
            pool_size: None,
            url: DEFAULT_DB_URL.to_string(),
        }
    }
}

#[derive(
    Clone,
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
pub struct DbUrl {
    pub prefix: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl DbUrl {
    pub fn new(
        prefix: impl ToString,
        host: impl ToString,
        port: u16,
        user: impl ToString,
        password: impl ToString,
        database: impl ToString,
    ) -> Self {
        Self {
            prefix: prefix.to_string(),
            host: host.to_string(),
            port,
            user: user.to_string(),
            password: password.to_string(),
            database: database.to_string(),
        }
    }

    pub fn postgresql(
        host: impl ToString,
        port: u16,
        user: impl ToString,
        password: impl ToString,
        database: impl ToString,
    ) -> Self {
        Self::new("postgresql", host, port, user, password, database)
    }

    pub fn to_string(&self) -> String {
        format!(
            "{prefix}://{user}:{password}@{host}:{port}/{database}",
            prefix = self.prefix,
            user = self.user,
            password = self.password,
            host = self.host,
            port = self.port,
            database = self.database
        )
    }
}

impl core::fmt::Display for DbUrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(&self.to_string())
    }
}
