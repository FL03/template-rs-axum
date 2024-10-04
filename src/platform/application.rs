/*
    Appellation: application <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Initializer;
use crate::{Context, Server, Settings};
use std::sync::Arc;

/// The [Application] struct is an abstraction of the entire application. It is primarily used as the
/// primary entry point for initializing any system resources and running the server.
pub struct Application {
    pub(crate) ctx: Arc<Context>,
    pub(crate) server: Server,
}

impl Application {
    pub fn new(cnf: Settings) -> Initializer {
        Initializer::new(cnf)
    }

    pub fn ctx(&self) -> &Context {
        &self.ctx
    }

    pub fn cnf(&self) -> &Settings {
        self.ctx.settings()
    }

    #[tracing::instrument(skip_all, name = "run", target = "app")]
    pub async fn run(self) -> std::io::Result<()> {
        self.server.serve().await
    }
}

/*
 ************* Implementations *************
*/

impl AsRef<Context> for Application {
    fn as_ref(&self) -> &Context {
        &self.ctx
    }
}

impl AsRef<Settings> for Application {
    fn as_ref(&self) -> &Settings {
        self.ctx.settings()
    }
}
