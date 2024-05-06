use anyhow::Result;
use ethers::providers::{Provider, Ws};
use home::home_dir;
use log::info;
use sandooo::common::constants::Env;
use sandooo::common::streams::{stream_new_blocks, stream_pending_transactions, Event};
use sandooo::common::utils::setup_logger;
use sandooo::sandwich::strategy::run_sandwich_strategy;
use std::sync::Arc;
use tokio::sync::broadcast::{self, Sender};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<()> {
    let path = home_dir().and_then(|a| Some(a.join(".env"))).unwrap();
    dotenv::from_path(&path)?;

    setup_logger().unwrap();

    info!("Starting Sandooo");

    let env = Env::new();

    let ws = Ws::connect(env.wss_url.clone()).await.unwrap();
    let provider = Arc::new(Provider::new(ws));

    let (event_sender, _): (Sender<Event>, _) = broadcast::channel(512);

    let mut set = JoinSet::new();

    set.spawn(stream_new_blocks(provider.clone(), event_sender.clone()));
    set.spawn(stream_pending_transactions(
        provider.clone(),
        event_sender.clone(),
    ));

    set.spawn(run_sandwich_strategy(
        provider.clone(),
        event_sender.clone(),
    ));

    while let Some(res) = set.join_next().await {
        info!("{:?}", res);
    }

    Ok(())
}
