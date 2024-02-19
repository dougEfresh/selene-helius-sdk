extern crate selene_helius_sdk;
use color_eyre::Result;
use selene_helius_sdk::api::webhook::Webhook;
use selene_helius_sdk::{Cluster, HeliusBuilder};
use tracing_subscriber::EnvFilter;

fn init_tracing() -> Result<()> {
  color_eyre::install()?;
  let filter = EnvFilter::from_default_env();
  let subscriber = tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter).with_target(true).finish();
  tracing::subscriber::set_global_default(subscriber)?;
  Ok(())
}

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
