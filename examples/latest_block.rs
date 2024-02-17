extern crate selene_helius_sdk;
use color_eyre::Result;
use selene_helius_sdk::HeliusBuilder;

#[tokio::main]
async fn main() -> Result<()> {
  let api_key = std::env::var("HELIUS_API_KEY").expect("env HELIUS_API_KEY is not defined!");
  let helius = HeliusBuilder::new(&api_key).build()?;
  let result = helius.connection().get_latest_blockhash().await?;

  println!("{result}");
  Ok(())
}
