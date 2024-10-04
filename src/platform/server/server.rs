/*
    Appellation: server <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{utils, ServerBuilder, ServerRouter};
use crate::Context;

use axum::Router;
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct Server {
    ctx: Arc<Context>,
    router: ServerRouter,
}

impl Server {
    pub fn new(ctx: Arc<Context>) -> Self {
        let router = ServerBuilder::new()
            .routes(ctx.clone())
            .with_tracing()
            .into_inner();
        Self { ctx, router }
    }

    pub fn builder() -> ServerBuilder {
        ServerBuilder::new()
    }

    pub fn from_context(ctx: Context) -> Self {
        Self::new(Arc::new(ctx))
    }

    pub async fn listen(&self) -> std::io::Result<TcpListener> {
        self.cnf().bind().await
    }

    pub fn cnf(&self) -> &crate::config::ServerConfig {
        self.ctx().settings().server()
    }

    pub fn ctx(&self) -> &Context {
        &self.ctx
    }

    pub fn router(&self) -> ServerRouter {
        self.router.clone()
    }

    pub async fn serve(self) -> std::io::Result<()> {
        let listener = self.listen().await?;
        let Server { ctx, router } = self;

        axum::serve(listener, router.into_make_service())
            .with_graceful_shutdown(utils::shutdown(ctx))
            .await
    }

    pub fn with_context(self, ctx: Context) -> Self {
        Self {
            ctx: Arc::new(ctx),
            ..self
        }
    }

    pub fn with_router(self, router: Router) -> Self {
        Self { router, ..self }
    }
}
