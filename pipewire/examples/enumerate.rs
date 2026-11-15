use anyhow::Result;
use pipewire::Pipewire;
use tracing::{Level, error};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .pretty()
        .init();
    if let Err(e) = run().await {
        error!("{e:?}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let pipewire = Pipewire::connect().await?;
    Ok(())
}
