/*
    Appellation: crud <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
// The `CRUD` enum is a simple enumeration describing the four basic operations a user can perform on a typical resource.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::EnumString,
    strum::VariantNames,
)]
#[repr(u8)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum CRUD {
    Create,
    #[default]
    Read,
    Update,
    Delete,
}

impl CRUD {
    pub fn create() -> Self {
        Self::Create
    }

    pub fn read() -> Self {
        Self::Read
    }

    pub fn update() -> Self {
        Self::Update
    }

    pub fn delete() -> Self {
        Self::Delete
    }
}
