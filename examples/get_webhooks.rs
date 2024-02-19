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

#[tokio::main]
async fn main() -> Result<()> {
  init_tracing()?;
  let api_key = std::env::var("HELIUS_API_KEY").expect("env HELIUS_API_KEY is not defined!");
  let helius = HeliusBuilder::new(&api_key).cluster(Cluster::MainnetBeta).build()?;
  let webhooks: Vec<Webhook> = helius.get_all_webhooks().await?;
  println!("{webhooks:#?}");
  Ok(())
}
