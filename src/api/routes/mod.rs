/*
    Appellation: routes <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod base;
pub mod items;

use crate::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub(crate) fn v0<S>(state: AppState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .nest("/", base_router())
        .nest("/items", items_router())
        .with_state(state)
}

fn base_router() -> Router<AppState> {
    Router::new()
        .route("/", get(base::base))
        .route("/info", get(base::info))
}

fn items_router() -> Router<AppState> {
    Router::new()
        .route("/", get(items::get_items))
        .route("/:id", get(items::get_item))
        .route("/:id", post(items::add_item))
        .route("/:id", delete(items::remove_item))
        .route("/:id", put(items::update_item))
}
