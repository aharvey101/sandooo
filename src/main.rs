use anyhow::Result;
use arboo::arbitrage::strategy::strategy;
use arboo::common::constants::Env;
use ethers::providers::{Provider, Ws};
use home::home_dir;
use log::info;
use std::sync::Arc;

//03-08-2024:
// Currently
// Simulator:
// - Needs to create the simulation transaction
// - Simulate transaction
// -
// Streamer:
// - Needs to stream logs, really should't be hard
// Stratey:
// - Copy strategy from arbooo1
#[tokio::main]
async fn main() -> Result<()> {
    let path = home_dir().and_then(|a| Some(a.join(".env"))).unwrap();
    dotenv::from_path(&path)?;

    info!("Starting Sandooo");

    let env = Env::new();

    let ws = Ws::connect(env.wss_url.clone()).await.unwrap();
    let provider = Arc::new(Provider::new(ws));
    strategy(provider).await.unwrap();
    Ok(())
}
