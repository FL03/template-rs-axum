/*
    Appellation: base <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::AppState;
use axum::extract::{Json, State};
use serde_json::{json, Value};

pub async fn base() -> Json<Value> {
    let data = json!({
        "message": "Welcome to the pzzld-api!",
    });
    Json(data)
}

pub async fn info(State(ctx): State<AppState>) -> Json<Value> {
    let data = json!({

        "version": ctx.settings.version,
    });
    Json(data)
}
