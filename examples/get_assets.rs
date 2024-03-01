extern crate selene_helius_sdk;
use color_eyre::Result;
use selene_helius_sdk::api::das::GetAssetsByOwnerParams;
use selene_helius_sdk::HeliusBuilder;
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
  let api_key = std::env::var("HELIUS_API_KEY").expect("env HELIUS_API_KEY is not defined!");
  init_tracing()?;
  let helius = HeliusBuilder::new(&api_key).build()?;
  let result = helius
    .get_assets_by_owner(&GetAssetsByOwnerParams {
      owner_address: "86xCnPeV69n6t3DnyGvkKobf9FdN2H9oiVDdaMpo2MMY".to_string(),
      ..Default::default()
    })
    .await?;

  println!("total: {}", result.total);
  for asset in result.items {
    println!("{}\t{}", asset.id, asset.interface);
  }

  Ok(())
}
