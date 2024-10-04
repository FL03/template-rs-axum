# pzzld-api

[![Clippy](https://github.com/FL03/pzzld-api/actions-api/workflows/clippy.yml/badge.svg)](https://github.com/FL03/pzzld-api/actions/workflows/clippy.yml)
[![Rust](https://github.com/FL03/pzzld-api/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/pzzld-api/actions/workflows/rust.yml)

[![Docker](https://github.com/FL03/pzzld-api/actions/workflows/docker.yml/badge.svg)](https://github.com/FL03/pzzld-api/actions/workflows/docker.yml)

***

### ***_This project is a work in progress_***

Welcome to the pzzld-api, a RESTful API built with Rust employing the [axum](https://docs.rs/axum) web-framework alongside [Tokio](https://tokio.rs) and [sqlx](https://docs.rs/sqlx)

# Getting Started

## Building from the source

### _Clone the repository_

```bash
git clone https://github.com/FL03/pzzld-api.git
cd pzzld-api

cargo build -r -v 
```

## Usage

### cargo

```rust
cargo run -r
```

### Docker
```rust
docker run -d -p "8080:8080" jo3mccain/pzzld-api
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

- [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
- [MIT](https://choosealicense.com/licenses/mit/)
