extern crate selene_helius_sdk;
use color_eyre::eyre::format_err;
use color_eyre::Result;
use selene_helius_sdk::api::types::{AccountWebhookEncoding, TxnStatus};
use selene_helius_sdk::api::webhook::{CreateWebhookRequest, WebhookData, WebhookType};
use selene_helius_sdk::util::init_tracing;
use selene_helius_sdk::{Cluster, HeliusBuilder};
use std::env;

// get a webhook from https://webhook.site/
// cargo run --example -- <webhook url> <address>...
#[tokio::main]
async fn main() -> Result<()> {
  init_tracing()?;
  let api_key = std::env::var("HELIUS_API_KEY").expect("env HELIUS_API_KEY is not defined!");
  let args: Vec<String> = env::args().collect();
  if args.len() < 3 {
    return Err(format_err!("cargo run --example -- <webhook url> <address>..."));
  }
  let wh = args[1].clone();
  let addresses = args[2..].to_vec();
  let helius = HeliusBuilder::new(&api_key).cluster(Cluster::MainnetBeta).build()?;
  let response = helius
    .create_webhook(&CreateWebhookRequest {
      data: WebhookData {
        webhook_url: wh,
        transaction_types: Vec::new(), // get all types
        //vec![TransactionType::Transfer],
        account_addresses: addresses,
        webhook_type: WebhookType::EnhancedDevnet,
        auth_header: None,
        txn_status: TxnStatus::default(),
        encoding: AccountWebhookEncoding::default(),
      },
    })
    .await?;
  println!("new webhook ids {} {}", response.webhook_id, response.webhook_data.webhook_url);
  Ok(())
}
