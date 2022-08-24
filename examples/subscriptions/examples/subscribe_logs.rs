use ethers::{
    core::{
        abi::AbiDecode,
        types::{Address, BlockNumber, Filter, U256},
    },
    providers::{Middleware, Provider, StreamExt, Ws},
};
use eyre::Result;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let url = if let Some(arg) = std::env::args().skip(1).next() {
        arg
    } else {
        "wss://eth-mainnet.g.alchemy.com/v2/API_KEY_HERE".to_string()
    };
    println!("URL: {}", &url);

    let client = Provider::<Ws>::connect(url).await?;
    let client = Arc::new(client);

    let last_block = client.get_block(BlockNumber::Latest).await?.unwrap().number.unwrap();
    println!("last_block: {last_block}");

    let erc20_transfer_filter =
        Filter::new().from_block(last_block - 25).event("Transfer(address,address,uint256)");

    let mut stream = client.subscribe_logs(&erc20_transfer_filter).await?.take(2);

    while let Some(log) = stream.next().await {
        println!(
            "block: {:?}\n    tx: {:?}\n    token: {:?}\n    from: {:?}\n    to: {:?}\n    amount: {:?}",
            log.block_number,
            log.transaction_hash,
            log.address,
            log.topics.get(1).map(|t| Address::from(t.clone())),
            log.topics.get(2).map(|t| Address::from(t.clone())),
            U256::decode(log.data)
        );
    }

    Ok(())
}
