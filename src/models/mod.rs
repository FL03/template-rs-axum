/*
    Appellation: models <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::items::ItemModel;

pub mod items;

pub type Timezone = chrono::Local;

pub type DateT<Tz = Timezone> = chrono::DateTime<Tz>;

pub type ItemId = uuid::Uuid;

pub type BigInt = i64;

pub type BigSerial = BigInt;
