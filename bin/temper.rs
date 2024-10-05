/*
    Appellation: pzzld <binary>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use template_rs_axum::{Application, Settings};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cnf = dbg!(Settings::build()?);
    let app = Application::new(cnf).with_tracing().init().await?;
    app.run().await?;

    Ok(())
}
