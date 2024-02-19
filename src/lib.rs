pub mod api;
pub mod error;
mod request_handler;
pub mod util;

pub type Result<T> = std::result::Result<T, error::HeliusError>;

pub use api::{Helius, HeliusBuilder};
use serde::Serialize;

#[derive(Clone, Copy, Default, Serialize, PartialEq, Eq, Debug)]
pub enum Cluster {
  #[default]
  MainnetBeta,
  Devnet,
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
  use crate::api::das::{
    DisplayOptions, GetAssetBatchParams, GetAssetParams, GetAssetProofBatchParams, GetAssetProofParams,
    GetAssetsByAuthorityParams, GetAssetsByCreatorParams, GetAssetsByGroupParams, GetAssetsByOwnerParams, Pagination,
    SearchAssetsParams,
  };
  use crate::api::types::enhanced::ParseTransactionsRequest;
  use crate::api::types::{AccountWebhookEncoding, TokenType, TransactionType, TxnStatus};
  use crate::api::webhook::{CreateWebhookRequest, EditWebhookRequest, WebhookData, WebhookType};
  use crate::api::{Helius, HeliusBuilder};
  use color_eyre::eyre::format_err;
  use solana_client::rpc_config::RpcBlockConfig;
  use solana_sdk::clock::Slot;
  use solana_sdk::commitment_config::CommitmentConfig;
  use solana_sdk::transaction::VersionedTransaction;
  use solana_transaction_status::UiTransactionEncoding;
  use std::env;
  use std::sync::Once;
  use std::time::Duration;
  use tracing::info;
  use tracing_subscriber::EnvFilter;
  static INIT: Once = Once::new();

  fn setup() {
    INIT.call_once(|| {
      #[allow(clippy::unwrap_used)]
      color_eyre::install().unwrap();
      let filter = EnvFilter::from_default_env();
      let subscriber = tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter).with_target(true).finish();
      tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    });
  }

  #[rstest::fixture]
  fn config() -> Config {
    setup();
    Config::new()
  }

  struct Config {
    client: Option<Helius>,
  }

  impl Config {
    #[allow(clippy::unwrap_used)]
    pub fn client(&self) -> &Helius {
      self.client.as_ref().unwrap()
    }

    pub fn new() -> Self {
      let env = dotenvy::dotenv();
      if env.is_err() {
        info!("no .env file");
      }
      let key: Option<String> = env::var("HELIUS_API_KEY").ok();
      let client: Option<Helius> =
        key.map(|k| HeliusBuilder::new(&k).timeout(Duration::from_secs(15)).build().unwrap());
      Self { client }
    }
  }

  #[rstest::rstest]
  #[tokio::test]
  #[allow(clippy::unwrap_used)]
  async fn test_enhanced_txn(config: Config) -> color_eyre::Result<()> {
    if config.client.is_none() {
      return Ok(());
    }
    let client = config.client();
    let slot: Slot = 243_662_530 as Slot;

    let encoded_confirmed_block = client
      .connection()
      .get_block_with_config(
        slot,
        RpcBlockConfig {
          encoding: Some(UiTransactionEncoding::Base64),
          commitment: Some(CommitmentConfig::confirmed()),
          max_supported_transaction_version: Some(0),
          ..RpcBlockConfig::default()
        },
      )
      .await?;
    let transactions: Vec<Option<VersionedTransaction>> =
      encoded_confirmed_block.transactions.unwrap().iter().map(|t| t.transaction.decode()).collect();

    let mut sigs: Vec<String> = Vec::new();
    for t in transactions {
      if t.is_none() {
        continue;
      }
      for s in t.unwrap().signatures {
        sigs.push(format!("{s}"));
      }
    }
    let sigs = ParseTransactionsRequest::from_slice(&sigs);
    let res = client.parse_transaction(&sigs[0]).await?;
    assert!(!res.is_empty());
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn webhook(config: Config) -> color_eyre::Result<()> {
    if config.client.is_none() {
      return Ok(());
    }
    let client = config.client();
    for h in client.get_all_webhooks().await? {
      // clean up any old test created webhooks
      if h.webhook_data.webhook_url.contains("localhost") {
        client.delete_webhook(&h.webhook_id).await?;
      }
    }
    let req = CreateWebhookRequest {
      data: WebhookData {
        webhook_url: "https://localhost:3000".to_string(),
        transaction_types: TransactionType::all(),
        account_addresses: vec!["AKo9P7S8FE9NYeAcrtZEpimwQAXJMp8Lrt8p4dMkHkY2".to_string()],
        webhook_type: WebhookType::Enhanced,
        auth_header: None,
        txn_status: TxnStatus::All,
        encoding: AccountWebhookEncoding::JsonParsed,
      },
    };
    let current_hooks = client.get_all_webhooks().await?;
    let hook = client.create_webhook(&req).await?;
    let hooks = client.get_all_webhooks().await?.len();
    if hooks != current_hooks.len() + 1 {
      client.delete_webhook(&hook.webhook_id).await?;
      return Err(format_err!("hook not created"));
    }
    let mut hooky = client.get_webhook_by_id(hook.webhook_id.as_str()).await?;
    hooky.webhook_data.transaction_types.push(TransactionType::Fuse);
    let edited_hook = client
      .edit_webhook(&EditWebhookRequest { webhook_id: hooky.webhook_id.clone(), data: hooky.webhook_data })
      .await?;

    let add_addr = vec!["AKo9P7S8FE9NYeAcrtZEpimwQAXJMp8Lrt8p4dMkHkY2".to_owned()];
    client.append_addresses_to_webhook(&hooky.webhook_id, &add_addr).await?;
    client.delete_webhook(&edited_hook.webhook_id).await?;
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn test_get_names(config: Config) -> color_eyre::Result<()> {
    if config.client.is_none() {
      return Ok(());
    }
    setup();
    let result = config.client().get_names("86xCnPeV69n6t3DnyGvkKobf9FdN2H9oiVDdaMpo2MMY").await?;
    assert!(!result.domain_names.is_empty());
    let r = result.domain_names.into_iter().find(|d| d == "toly");
    assert!(r.is_some());
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn test_get_asset_nft(config: Config) -> color_eyre::Result<()> {
    if config.client.is_none() {
      return Ok(());
    }
    let client = config.client();
    let mad_libs = String::from("F9Lw3ki3hJ7PF9HQXsBzoY8GyE6sPoEZZdXJBsTTD2rk");
    let res = client.get_asset(&GetAssetParams { id: mad_libs.clone(), display_options: None }).await?;
    assert_eq!(res.id, mad_libs);
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn get_asset_fungible(config: Config) -> color_eyre::Result<()> {
    if config.client.is_none() {
      return Ok(());
    }
    let client = config.client();
    let jito = String::from("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn");
    let opts = DisplayOptions { show_fungible: true, show_inscription: false };
    let res = client.get_asset(&GetAssetParams { id: jito.clone(), display_options: Some(opts) }).await?;
    assert_eq!(res.id, jito);
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn get_asset_inscription(config: Config) -> color_eyre::Result<()> {
    if config.client.is_none() {
      return Ok(());
    }
    let client = config.client();
    let rando = String::from("AKo9P7S8FE9NYeAcrtZEpimwQAXJMp8Lrt8p4dMkHkY2");
    let opts = DisplayOptions { show_fungible: false, show_inscription: true };
    let res = client.get_asset(&GetAssetParams { id: rando.clone(), display_options: Some(opts) }).await?;
    assert_eq!(res.id, rando);
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn get_asset_batch(config: Config) -> color_eyre::Result<()> {
    if config.client.is_none() {
      return Ok(());
    }
    let client = config.client();
    let ids: Vec<String> = vec![
      "81bxPqYCE8j34nQm7Rooqi8Vt3iMHLzgZJ71rUVbQQuz".to_string(),
      "AKo9P7S8FE9NYeAcrtZEpimwQAXJMp8Lrt8p4dMkHkY2".to_string(),
    ];

    let res = client.get_asset_batch(&GetAssetBatchParams { ids, display_options: None }).await?;
    assert!(!res.is_empty());
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn get_asset_by_owner(config: Config) -> color_eyre::Result<()> {
    if config.client.is_none() {
      return Ok(());
    }
    let client = config.client();
    let rando = String::from("86xCnPeV69n6t3DnyGvkKobf9FdN2H9oiVDdaMpo2MMY");
    let opts = DisplayOptions { show_fungible: true, show_inscription: false };
    let res = client
      .get_assets_by_owner(&GetAssetsByOwnerParams {
        display_options: Some(opts),
        owner_address: rando.clone(),
        sort_by: None,
        pagination: Pagination::default(),
      })
      .await?;
    assert!(!res.items.is_empty());
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn asset_by_authority(config: Config) -> color_eyre::Result<()> {
    if config.client.is_none() {
      return Ok(());
    }
    let client = config.client();
    let rando = String::from("2RtGg6fsFiiF1EQzHqbd66AhW7R5bWeQGpTbv2UMkCdW");
    let res = client
      .get_assets_by_authority(&GetAssetsByAuthorityParams {
        authority_address: rando,
        display_options: Some(DisplayOptions::default()),
        ..Default::default()
      })
      .await?;
    assert!(!res.items.is_empty());
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn get_asset_by_creator(config: Config) -> color_eyre::Result<()> {
    if config.client.is_none() {
      return Ok(());
    }
    let client = config.client();
    let rando = String::from("D3XrkNZz6wx6cofot7Zohsf2KSsu2ArngNk8VqU9cTY3");
    let res = client
      .get_assets_by_creator(&GetAssetsByCreatorParams {
        creator_address: rando,
        only_verified: true,
        display_options: Some(DisplayOptions { show_fungible: true, show_inscription: true }),
        pagination: Pagination { limit: Some(300), ..Default::default() },
        ..Default::default()
      })
      .await?;
    assert!(!res.items.is_empty());
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn search_asset(config: Config) -> color_eyre::Result<()> {
    if config.client.is_none() {
      return Ok(());
    }
    let client = config.client();
    let rando = String::from("5aZZ4duJUKiMsJN9vRsoAn4SDX7agvKu7Q3QdFWRfWze");
    let types: Vec<TokenType> =
      vec![TokenType::All, TokenType::CompressedNft, TokenType::NonFungible, TokenType::RegularNft];
    for t in types {
      client
        .search_assets(&SearchAssetsParams {
          owner_address: Some(rando.clone()),
          token_type: Some(t),
          pagination: Pagination { limit: Some(100), ..Default::default() },
          ..Default::default()
        })
        .await?;
    }
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn asset_groups(config: Config) -> color_eyre::Result<()> {
    if config.client.is_none() {
      return Ok(());
    }
    let client = config.client();
    let rando = String::from("J1S9H3QjnRtBbbuD4HjPV6RpRhwuk4zKbxsnCHuTgh9w");
    client
      .get_assets_by_group(&GetAssetsByGroupParams {
        group_key: "collection".to_string(),
        group_value: rando,
        ..Default::default()
      })
      .await?;
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn asset_proof(config: Config) -> color_eyre::Result<()> {
    if config.client.is_none() {
      return Ok(());
    }
    let client = config.client();
    let rando = String::from("Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss");
    client.get_asset_proof(&GetAssetProofParams { id: rando }).await?;
    Ok(())
  }

  #[rstest::rstest]
  #[tokio::test]
  async fn asset_proof_batch(config: Config) -> color_eyre::Result<()> {
    if config.client.is_none() {
      return Ok(());
    }
    let client = config.client();
    let rando = String::from("Bu1DEKeawy7txbnCEJE4BU3BKLXaNAKCYcHR4XhndGss");
    client.get_asset_proof_batch(&GetAssetProofBatchParams { ids: vec![rando] }).await?;
    Ok(())
  }

  #[rstest::rstest]
  #[test]
  fn check_ci(config: Config) -> color_eyre::Result<()> {
    match env::var("CI") {
      Err(_) => Ok(()),
      Ok(_) => match config.client {
        Some(_) => Ok(()),
        None => Err(format_err!("client is not configured and you are running in CI")),
      },
    }
  }
}
