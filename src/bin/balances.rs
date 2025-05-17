use anyhow::Result;
use log::{error, info};
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, native_token::LAMPORTS_PER_SOL, pubkey::Pubkey,
};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    endpoint: String,
    pubkeys: Vec<String>,
}

async fn g_bal(pk: &str, ep: &str) {
    let milli = Duration::from_secs(2);
    let client = RpcClient::new_with_commitment(String::from(ep), CommitmentConfig::confirmed());
    loop {
        match Pubkey::from_str(pk) {
            Ok(pubkey) => match client.get_balance(&pubkey).await {
                Ok(balance) => info!(
                    "{:?} Acc: {} - {:#?} SOL",
                    std::thread::current().id(),
                    pubkey,
                    balance / LAMPORTS_PER_SOL
                ),
                Err(e) => error!("Could not get balance for account {}. Error: {}", pubkey, e),
            },
            Err(e) => error!("Wrong pubkey format [{}]. Error: {}", pk, e),
        }
        sleep(milli);
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<()> {
    env_logger::init();
    let f = std::fs::File::open("config/balances.yaml").expect("Could not open file.");
    let config: Config = serde_yaml::from_reader(f).expect("Could not read values.");

    let mut tasks = vec![];

    for pk in config.pubkeys.iter() {
        let ep = config.endpoint.clone();
        let pk0 = pk.clone();
        let task = tokio::spawn(async move {
            g_bal(&pk0, &ep).await;
        });
        tasks.push(task);
    }

    for task in tasks {
        let _ = task.await.map_err(|e| error!("Task failed {e}"));
    }

    Ok(())
}
