extern crate selene_helius_sdk;
use color_eyre::Result;
use selene_helius_sdk::api::das::{GetAssetsByOwnerParams, Pagination};
use selene_helius_sdk::HeliusBuilder;

#[tokio::main]
async fn main() -> Result<()> {
  let api_key = std::env::var("HELIUS_API_KEY").expect("env HELIUS_API_KEY is not defined!");
  let helius = HeliusBuilder::new(&api_key).build()?;
  let result = helius
    .get_assets_by_owner(&GetAssetsByOwnerParams {
      owner_address: "86xCnPeV69n6t3DnyGvkKobf9FdN2H9oiVDdaMpo2MMY".to_string(),
      pagination: Pagination::default(),
      display_options: None,
      sort_by: None,
    })
    .await?;

  println!("total: {}", result.total);
  for asset in result.items {
    println!("{}", asset.id);
  }

  Ok(())
}
