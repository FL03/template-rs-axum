/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub fn map_err<T>(err: T) -> T
where
    T: std::fmt::Display,
{
    tracing::error!("{err}");
    err
}
