/*
    Appellation: settings <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::impl_server::*;
use config::builder::{ConfigBuilder, DefaultState};
use core::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
pub struct ServerConfig {
    pub addr: ServerAddr,
}

impl ServerConfig {
    pub fn new() -> Self {
        Self {
            addr: ServerAddr::default(),
        }
    }

    pub fn builder() -> ConfigBuilder<DefaultState> {
        config::Config::builder()
            .set_default("server.host", crate::LOCALHOST)
            .unwrap()
            .set_default("server.port", crate::DEFAULT_PORT)
            .unwrap()
            .add_source(
                config::Environment::default()
                    .separator("_")
                    .prefix("SERVER"),
            )
    }

    pub fn build() -> Result<Self, config::ConfigError> {
        Self::builder().build()?.try_deserialize()
    }

    pub const fn addr(&self) -> &ServerAddr {
        &self.addr
    }

    pub fn addr_mut(&mut self) -> &mut ServerAddr {
        &mut self.addr
    }

    pub fn as_socket_addr(&self) -> SocketAddr {
        self.addr().as_socket_addr()
    }

    pub async fn bind(&self) -> std::io::Result<TcpListener> {
        TcpListener::bind(self.as_socket_addr()).await
    }
}

/*
 ************* Implementations *************
*/
impl Default for ServerConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(&serde_json::to_string(self).unwrap())
    }
}
unsafe impl core::marker::Send for ServerConfig {}

unsafe impl core::marker::Sync for ServerConfig {}

mod impl_server {
    use crate::config::ConfigBuilder;
    use core::net::{IpAddr, SocketAddr};
    use std::str::FromStr;
    use tokio::net::TcpListener;

    pub trait NetworkAddress {
        type Ip;

        fn ip(&self) -> Self::Ip;
    }

    #[derive(
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        serde::Deserialize,
        serde::Serialize,
    )]
    pub struct A {
        pub host: [u8; 4],
        pub port: u16,
    }

    #[derive(
        Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
    )]
    pub struct ServerAddr {
        pub host: String,
        pub port: u16,
    }

    impl ServerAddr {
        pub fn new(host: impl ToString, port: u16) -> Self {
            Self {
                host: host.to_string(),
                port,
            }
        }

        pub fn from_addr(addr: SocketAddr) -> Self {
            Self::new(addr.ip().to_string(), addr.port())
        }

        pub fn builder() -> ConfigBuilder {
            config::Config::builder()
                .set_default("host", crate::LOCALHOST)
                .unwrap()
                .set_default("port", crate::DEFAULT_PORT)
                .unwrap()
                .add_source(
                    config::Environment::default()
                        .separator("_")
                        .prefix("SERVER"),
                )
                .set_override_option("host", std::env::var("SERVER_HOST").ok())
                .unwrap()
                .set_override_option("port", std::env::var("SERVER_PORT").ok())
                .unwrap()
        }

        pub fn build() -> Result<Self, config::ConfigError> {
            Self::builder().build()?.try_deserialize()
        }

        pub fn as_socket_addr(&self) -> SocketAddr {
            format!("{}:{}", self.host, self.port).parse().unwrap()
        }

        pub async fn bind(&self) -> std::io::Result<TcpListener> {
            TcpListener::bind(self.as_socket_addr()).await
        }

        pub fn ip(&self) -> IpAddr {
            self.as_socket_addr().ip()
        }

        pub fn host(&self) -> &str {
            &self.host
        }

        pub fn host_mut(&mut self) -> &mut String {
            &mut self.host
        }

        pub const fn port(&self) -> u16 {
            self.port
        }

        pub fn port_mut(&mut self) -> &mut u16 {
            &mut self.port
        }

        pub fn set_host(&mut self, host: impl ToString) {
            self.host = host.to_string();
        }

        pub fn set_port(&mut self, port: u16) {
            self.port = port;
        }

        pub fn with_host(self, host: impl ToString) -> Self {
            Self {
                host: host.to_string(),
                ..self
            }
        }

        pub fn with_port(self, port: u16) -> Self {
            Self { port, ..self }
        }
    }

    impl Default for ServerAddr {
        fn default() -> Self {
            Self {
                host: crate::LOCALHOST.to_string(),
                port: crate::DEFAULT_PORT,
            }
        }
    }

    impl core::fmt::Display for ServerAddr {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{}:{}", self.host, self.port)
        }
    }

    unsafe impl core::marker::Send for ServerAddr {}

    unsafe impl core::marker::Sync for ServerAddr {}

    impl From<SocketAddr> for ServerAddr {
        fn from(addr: SocketAddr) -> Self {
            Self::new(addr.ip().to_string(), addr.port())
        }
    }

    impl From<ServerAddr> for SocketAddr {
        fn from(config: ServerAddr) -> Self {
            config.as_socket_addr()
        }
    }

    impl From<(String, u16)> for ServerAddr {
        fn from((host, port): (String, u16)) -> Self {
            Self::new(host, port)
        }
    }

    impl From<([u8; 4], u16)> for ServerAddr {
        fn from((host, port): ([u8; 4], u16)) -> Self {
            let addr = SocketAddr::from((host, port));
            Self::from_addr(addr)
        }
    }

    impl FromStr for ServerAddr {
        type Err = std::io::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let parts: Vec<&str> = s.split(':').collect();
            let host = parts[0];
            let port = parts[1].parse().unwrap();
            Ok(Self::new(host, port))
        }
    }
}
