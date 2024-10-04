/*
    Appellation: base <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::config::ItemOps;
use crate::models::{ItemId, ItemModel};
use crate::AppState;
use axum::extract::{Json, Path, Query, State};
use serde_json::Value;

pub async fn get_items(State(ctx): State<AppState>) -> Json<Vec<ItemModel>> {
    let items = ctx.get_items().await.unwrap_or_default();
    axum::Json(items)
}

pub async fn get_item(
    State(ctx): State<AppState>,
    Path(id): Path<ItemId>,
) -> Json<Option<ItemModel>> {
    let item = ctx.get_item(id).await.ok();

    axum::Json(item)
}

pub async fn add_item(State(ctx): State<AppState>, Path(title): Path<String>) -> Json<Value> {
    let item = ctx.add_item(title, String::new()).await;
    let data = match item {
        Ok(item) => {
            serde_json::json!({
                "result": "success",
                "data": item,
            })
        }
        Err(e) => {
            serde_json::json!({
                "result": "error",
                "message": e.to_string(),
            })
        }
    };
    axum::Json(data)
}

pub async fn remove_item(State(ctx): State<AppState>, Path(id): Path<ItemId>) -> Json<Value> {
    let item = ctx.remove_item(id).await;
    let data = match item {
        Ok(item) => {
            serde_json::json!({
                "result": "success",
                "data": item,
            })
        }
        Err(e) => {
            serde_json::json!({
                "result": "error",
                "message": e.to_string(),
            })
        }
    };
    axum::Json(data)
}

/// Update
pub async fn update_item(
    State(ctx): State<AppState>,
    Path(id): Path<ItemId>,
    Query(query): Query<ItemQuery>,
) -> Json<Value> {
    let item = {
        let cur = ctx.get_item(id).await.expect("Item not found");
        query.update_model(cur)
    };
    let data = match ctx.update_item(id, item.title, item.description).await {
        Ok(item) => {
            serde_json::json!({
                "result": "success",
                "data": item,
            })
        }
        Err(e) => {
            serde_json::json!({
                "result": "error",
                "message": e.to_string(),
            })
        }
    };
    axum::Json(data)
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
pub struct ItemQuery {
    pub title: Option<String>,
    pub description: Option<String>,
}

impl ItemQuery {
    pub fn update_model(self, model: ItemModel) -> ItemModel {
        ItemModel {
            id: model.id,
            title: self.title.unwrap_or(model.title),
            description: self.description.unwrap_or(model.description),
            created_at: model.created_at,
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
pub struct Response<T = Value> {
    pub(crate) data: T,
    pub(crate) status: String,
}

impl<T> Response<T> {
    pub fn new(status: String, data: T) -> Self {
        Self { status, data }
    }

    pub fn success(data: T) -> Self {
        Self::new("success".to_string(), data)
    }

    pub fn error(data: T) -> Self {
        Self::new("error".to_string(), data)
    }

    pub const fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn into_axum(self) -> axum::Json<Self> {
        axum::Json(self)
    }
}
