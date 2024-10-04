/*
    Appellation: oauth <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod providers {

    pub struct Credential {
        pub client_id: String,
        pub client_secret: String,
        pub redirect_uri: String,
    }

    pub struct AuthProvider {
        pub name: String,
        pub url: String,
        pub client_id: String,
        pub client_secret: String,
    }
}