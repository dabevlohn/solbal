use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey,
};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    endpoint: String,
    pubkeys: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let f = std::fs::File::open("config/balances.yaml").expect("Could not open file.");
    let config: Config = serde_yaml::from_reader(f).expect("Could not read values.");
    println!("{:?}", config);
    let client = RpcClient::new_with_commitment(
        String::from(config.endpoint),
        CommitmentConfig::confirmed(),
    );

    for pk in config.pubkeys.iter() {
        let pubkey = Pubkey::from_str(pk)?;
        let balance = client.get_balance(&pubkey).await?;
        println!("{:#?} SOL", balance / LAMPORTS_PER_SOL);
    }

    Ok(())
}
