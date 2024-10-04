/*
    Appellation: builder <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::api::routes;
use crate::{AppState, Context};
use axum::{
    extract::Request,
    response::IntoResponse,
    routing::{MethodRouter, Route},
    Extension, Router,
};
use core::convert::Infallible;
use std::sync::Arc;
use tower::{Layer, Service};
use tower_http::{classify::StatusInRangeAsFailures, services, trace::TraceLayer};

pub struct ServerBuilder<S = ()>(pub Router<S>);

impl<S> ServerBuilder<S> where S: Clone + Send + Sync + 'static {}

impl<S> ServerBuilder<S>
where
    S: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self(Router::<S>::new())
    }

    pub fn from_router(router: Router<S>) -> Self {
        Self(router)
    }

    pub fn routes(self, ctx: AppState) -> ServerBuilder<S> {
        let router = self.into_inner().nest("/api", routes::v0(ctx));
        ServerBuilder(router)
    }

    pub fn with_context(self, ctx: Arc<Context>) -> Self {
        self.layer(Extension(ctx))
    }

    pub fn with_tracing(self) -> Self {
        self.layer(TraceLayer::new(
            StatusInRangeAsFailures::new(400..=599).into_make_classifier(),
        ))
    }

    pub fn into_inner(self) -> Router<S> {
        self.0
    }

    pub fn layer<L>(self, layer: L) -> Self
    where
        L: Clone + Layer<Route> + Send + 'static,
        L::Service: Clone + Service<Request> + Send + 'static,
        <L::Service as Service<Request>>::Error: Into<Infallible> + 'static,
        <L::Service as Service<Request>>::Future: Send + 'static,
        <L::Service as Service<Request>>::Response: IntoResponse + 'static,
    {
        Self(self.into_inner().layer(layer))
    }

    pub fn nest(self, path: &str, router: Router<S>) -> Self {
        let router = self.into_inner().nest(path, router);
        Self(router)
    }

    pub fn nest_service<T>(self, path: &str, svc: T) -> Self
    where
        T: Clone + Send + Service<Request, Error = Infallible> + 'static,
        T::Response: IntoResponse,
        T::Future: Send + 'static,
    {
        let router = self.into_inner().nest_service(path, svc);
        Self::from_router(router)
    }

    pub fn route(self, path: &str, method: MethodRouter<S>) -> Self {
        let router = self.into_inner().route(path, method);
        Self::from_router(router)
    }

    pub fn route_service<T>(self, path: &str, svc: T) -> Self
    where
        T: Clone + Send + Service<Request, Error = Infallible> + 'static,
        T::Response: IntoResponse,
        T::Future: Send + 'static,
    {
        let router = self.into_inner().route_service(path, svc);
        Self::from_router(router)
    }

    pub fn serve_dir(self, path: &str, workdir: &str) -> Self {
        self.route_service(path, services::ServeDir::new(workdir))
    }

    pub fn serve_file(self, target: &str) -> Self {
        self.route_service("/", services::ServeFile::new(target))
    }

    pub fn with_state<S2>(self, state: S) -> ServerBuilder<S2> {
        let router = self.into_inner().with_state(state);
        ServerBuilder(router)
    }
}
