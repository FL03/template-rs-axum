/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::prelude::*;

pub mod crud;
pub mod status;

use std::sync::{Arc, Mutex};

pub type Arcm<T = ()> = Arc<Mutex<T>>;

pub type AppState = Arc<crate::Context>;
/// A type alias for `axum`'s [Router](axum::Router)
pub type ApiRouter<S = ()> = axum::Router<S>;

pub type DbPool<P = sqlx::Postgres> = sqlx::Pool<P>;

pub(crate) mod prelude {
    pub use super::crud::CRUD;
    pub use super::status::Status;
    pub use super::{ApiRouter, AppState, Arcm, DbPool};
}
