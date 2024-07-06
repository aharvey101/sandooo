use crate::arbitrage::simulation::simulation;
use anyhow::Result;

use ethers::types::H160;
use ethers_providers::{Middleware, Provider, Ws};

use futures::StreamExt;
use revm::primitives::Address;
use std::sync::Arc;

pub async fn strategy(provider: Arc<Provider<Ws>>) -> Result<()> {
    //
    let pools = get_pools();
    let contract_address: Address = "0x3ffeea07a27fab7ad1df5297fa75e77a43cb5790".parse()?;

    // Define the event filter
    let swap_event = "Swap(address,address,int256,int256,uint160,uint128,int24)";
    let filter = ethers::core::types::Filter::new().event(swap_event);

    let dai_token_address: H160 = "0x6B175474E89094C44Da98b954EedeAC495271d0F".parse()?;
    let eth_token_address: H160 = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse()?;
    let usdt_token_address: H160 = "0xdAC17F958D2ee523a2206206994597C13D831ec7".parse()?;
    let pool_pair = vec![(
        "0x60594a405d53811d3BC4766596EFD80fd545A270", // USDT/eth v3
        "0xA478c2975Ab1Ea89e8196811F51A7B7Ade33eB11", // USDT/eth v2
    )];
    // Subscribe to the events
    let prov = provider.to_owned();
    let mut stream = prov.subscribe_logs(&filter).await?;

    while let Some(log) = stream.next().await {
        // Print the log data (you can parse it further based on the event structure)
        let address = log.address;
        if !pool_pair
            .clone()
            .into_iter()
            .any(|(pool_a, _)| pool_a == address.to_string())
        {
            continue;
        }
        println!("Log address: {:#?}", log.address);
        println!("Yay a pool we care about");
        // now do simulations!
        let target_pool: H160 = pool_pair[0].0.parse()?;
        let adjacent_pool: H160 = pool_pair[0].1.parse()?;
        // This should proabably be a tokio thread
        simulation(
            target_pool,
            adjacent_pool,
            usdt_token_address,
            provider.clone(),
        )
        .await?;
        // after we simulate, if the profit is >0, we should create and send the tx

        // let join_handle = tokio::spawn(simulation(address, adjacent_pool))
        //     .await?
        //     .expect("Error with spawn");
    }

    // subscribe to price changes on events?

    // 2. Subscribe to all pools
    // 3. When an event happens:
    //      - spawn a thread handling the event
    //      - spawned thread should run the simulation function
    //      - simulation function should calculate the potential for profit
    //      - profit is something like:
    //          - arb revenue - gas cost

    Ok(())
}
fn get_pools() -> Vec<(String, String)> {
    // todo!();
    // just define some pools, we gotta work out a better way to get pools that exist on v2 and v3
    //
    let pools: Vec<(String, String)> = vec![
        (
            "0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc".to_string(), //USDC/ETH v2 0.3% fee
            "0x4e68Ccd3E89f51C3074ca5072bbAC773960dFa36".to_string(), //ETH/USDT v3? 0.3% fee
        ),
        (
            "0x8ad599c3A0ff1De082011EFDDc58f1908eb6e6D8".to_string(), //USDC/ETH v3
            "0x0d4a11d5EEaaC28EC3F61d100daF4d40471f1852".to_string(), //ETH/USDT v2 0.3% fee
        ),
    ];

    pools
}
