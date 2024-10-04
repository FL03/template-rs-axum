/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::models::{ItemId, ItemModel};
use crate::{map_err, DbPool, Settings};
use sqlx::FromRow;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait ItemOps {
    async fn add_item(&self, title: String, description: String) -> sqlx::Result<ItemModel>;
    async fn get_items(&self) -> sqlx::Result<Vec<ItemModel>>;
    async fn get_item(&self, id: ItemId) -> sqlx::Result<ItemModel>;
    async fn remove_item(&self, id: ItemId) -> sqlx::Result<ItemModel>;
    async fn update_item(
        &self,
        id: ItemId,
        title: String,
        description: String,
    ) -> sqlx::Result<ItemModel>;
}

#[derive(Clone, Debug)]
pub struct Context {
    pub(crate) db: DbPool,
    pub(crate) settings: Settings,
}

impl Context {
    pub fn new(db: DbPool, settings: Settings) -> Self {
        Self { db, settings }
    }

    pub fn db(&self) -> &DbPool {
        &self.db
    }

    pub const fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn into_ext(self: Arc<Self>) -> axum::Extension<Arc<Self>> {
        axum::Extension(self)
    }

    pub fn into_shared(self) -> Arc<Self> {
        Arc::new(self)
    }
}

#[async_trait::async_trait]
impl ItemOps for Context {
    async fn add_item(&self, title: String, description: String) -> sqlx::Result<ItemModel> {
        let query =
            sqlx::query("INSERT INTO items (title, description) VALUES ($1, $2) RETURNING *")
                .bind(title)
                .bind(description)
                .fetch_one(&self.db)
                .await?;
        ItemModel::from_row(&query).map_err(map_err)
    }
    async fn get_items(&self) -> sqlx::Result<Vec<ItemModel>> {
        let query = sqlx::query("SELECT * FROM items")
            .fetch_all(&self.db)
            .await?;
        let samples = query
            .iter()
            .filter_map(|item| ItemModel::from_row(item).map_err(map_err).ok())
            .collect();
        Ok(samples)
    }

    async fn get_item(&self, id: ItemId) -> sqlx::Result<ItemModel> {
        let query = sqlx::query("SELECT * FROM items WHERE id = $1");

        let item = query.bind(id).fetch_one(&self.db).await?;
        ItemModel::from_row(&item).map_err(map_err)
    }

    async fn remove_item(&self, id: ItemId) -> sqlx::Result<ItemModel> {
        let query = sqlx::query("DELETE FROM items WHERE id = $1 RETURNING *")
            .bind(id)
            .fetch_one(&self.db)
            .await?;
        ItemModel::from_row(&query).map_err(map_err)
    }

    async fn update_item(
        &self,
        id: ItemId,
        title: String,
        description: String,
    ) -> sqlx::Result<ItemModel> {
        let query =
            sqlx::query("UPDATE items SET title = $1, description = $2 WHERE id = $3 RETURNING *")
                .bind(title)
                .bind(description)
                .bind(id)
                .fetch_one(&self.db)
                .await?;
        ItemModel::from_row(&query).map_err(map_err)
    }
}

/*
 ************* Implementations *************
*/
impl AsRef<Settings> for Context {
    fn as_ref(&self) -> &Settings {
        &self.settings
    }
}

impl AsMut<Settings> for Context {
    fn as_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }
}

impl core::borrow::Borrow<Settings> for Context {
    fn borrow(&self) -> &Settings {
        self.settings.borrow()
    }
}

impl core::borrow::BorrowMut<Settings> for Context {
    fn borrow_mut(&mut self) -> &mut Settings {
        self.settings.borrow_mut()
    }
}

impl core::ops::Deref for Context {
    type Target = Settings;

    fn deref(&self) -> &Self::Target {
        &self.settings
    }
}

impl core::ops::DerefMut for Context {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.settings
    }
}

impl core::fmt::Display for Context {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(&self.settings().to_string())
    }
}

unsafe impl core::marker::Send for Context {}

unsafe impl core::marker::Sync for Context {}
