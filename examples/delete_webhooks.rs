extern crate selene_helius_sdk;
use color_eyre::Result;
use selene_helius_sdk::api::webhook::Webhook;
use selene_helius_sdk::util::init_tracing;
use selene_helius_sdk::{Cluster, HeliusBuilder};

// delete webhook from https://webhook.site/
#[tokio::main]
async fn main() -> Result<()> {
  init_tracing()?;
  let api_key = std::env::var("HELIUS_API_KEY").expect("env HELIUS_API_KEY is not defined!");
  let helius = HeliusBuilder::new(&api_key).cluster(Cluster::MainnetBeta).build()?;
  let response = helius.get_all_webhooks().await?;
  let webhooks: Vec<Webhook> =
    response.into_iter().filter(|w| w.webhook_data.webhook_url.contains("webhook.site")).collect();
  for wh in webhooks {
    println!("deleting wh {} {}", wh.webhook_id, wh.webhook_data.webhook_url);
    helius.delete_webhook(&wh.webhook_id).await?;
  }
  Ok(())
}
