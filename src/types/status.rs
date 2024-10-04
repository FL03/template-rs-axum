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
pub enum Status {
    Error,
    #[default]
    Success,
}

impl Status {
    pub fn error() -> Self {
        Self::Error
    }

    pub fn success() -> Self {
        Self::Success
    }
}
