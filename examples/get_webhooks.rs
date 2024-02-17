extern crate selene_helius_sdk;
use color_eyre::Result;
use selene_helius_sdk::api::webhook::Webhook;
use selene_helius_sdk::util::init_tracing;
use selene_helius_sdk::{Cluster, HeliusBuilder};

#[tokio::main]
async fn main() -> Result<()> {
  init_tracing()?;
  let api_key = std::env::var("HELIUS_API_KEY").expect("env HELIUS_API_KEY is not defined!");
  let helius = HeliusBuilder::new(&api_key).cluster(Cluster::MainnetBeta).build()?;
  let webhooks: Vec<Webhook> = helius.get_all_webhooks().await?;
  println!("{webhooks:#?}");
  Ok(())
}
