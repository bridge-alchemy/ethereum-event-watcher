use crate::primitives::AppState;
use ethers::{
    core::{ abi::AbiDecode, types::{ Address, BlockNumber, Filter, U256 } },
    providers::{ Middleware, Provider, StreamExt, Ws },
    contract::abigen,
};
use eyre::Result;
use std::sync::Arc;

abigen!(
    IERC20,
    r#"[
        event Transfer(address indexed from, address indexed to, uint256 value)
        event Approval(address indexed owner, address indexed spender, uint256 value)
    ]"#
);

const WSS_URL: &str = "wss://mainnet.infura.io/ws/v3/c60b0bb42f8a4c6481ecd229eddaca27";
const WETH_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";

pub async fn serve(_app_state: &AppState) -> Result<()> {
    let client = Provider::<Ws>::connect(WSS_URL).await.unwrap();
    let client = Arc::new(client);

    let last_block = client.get_block(BlockNumber::Latest).await.unwrap().unwrap().number.unwrap();
    println!("last_block: {last_block}");

    let erc20_transfer_filter = Filter::new()
        .address(WETH_ADDRESS.parse::<Address>().unwrap())
        .from_block(last_block - 25)
        .event("Transfer(address,address,uint256)");

    let mut stream = client.subscribe_logs(&erc20_transfer_filter).await.unwrap().take(2);

    while let Some(log) = stream.next().await {
        println!(
            "block: {:?}, tx: {:?}, token: {:?}, from: {:?}, to: {:?}, amount: {:?}",
            log.block_number,
            log.transaction_hash,
            log.address,
            Address::from(log.topics[1]),
            Address::from(log.topics[2]),
            U256::decode(log.data)
        );
    }
    Ok(())
}
