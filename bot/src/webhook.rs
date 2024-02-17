use crate::command;
use crate::command::WebhookCommands;
use selene_helius_sdk::api::types::{AccountWebhookEncoding, TransactionType, TxnStatus};
use selene_helius_sdk::api::webhook::{CreateWebhookRequest, EditWebhookRequest, Webhook, WebhookData, WebhookType};
use selene_helius_sdk::{Cluster, Helius, HeliusBuilder};

async fn list(helius: Helius) -> color_eyre::Result<()> {
  let webhooks: Vec<Webhook> = helius.get_all_webhooks().await?;
  if webhooks.is_empty() {
    return Ok(println!("no webhooks found"));
  }
  Ok(println!("{webhooks:#?}"))
}

async fn create(helius: Helius, url: String, devnet: bool, addresses: Vec<String>) -> color_eyre::Result<()> {
  let webhook_type = if devnet { WebhookType::EnhancedDevnet } else { WebhookType::Enhanced };
  let req = CreateWebhookRequest {
    data: WebhookData {
      webhook_url: url,
      transaction_types: TransactionType::all(),
      account_addresses: addresses,
      webhook_type,
      auth_header: None,
      txn_status: TxnStatus::All,
      encoding: AccountWebhookEncoding::JsonParsed,
    },
  };
  let response = helius.create_webhook(&req).await?;
  Ok(println!("id {}", response.webhook_id))
}

async fn delete(helius: Helius, id: String) -> color_eyre::Result<()> {
  helius.delete_webhook(&id).await?;
  Ok(())
}

async fn add(helius: Helius, id: String, mut addresses: Vec<String>) -> color_eyre::Result<()> {
  let mut hook = helius.get_webhook_by_id(&id).await?;
  hook.webhook_data.account_addresses.append(&mut addresses);
  helius.edit_webhook(&EditWebhookRequest { webhook_id: hook.webhook_id.clone(), data: hook.webhook_data }).await?;
  Ok(())
}

pub(crate) async fn process_webhook(args: command::WebhookArgs) -> color_eyre::Result<()> {
  let helius = HeliusBuilder::new(&args.helius_api_key).cluster(Cluster::MainnetBeta).build()?;
  match args.command {
    None => list(helius).await,
    Some(WebhookCommands::List) => list(helius).await,
    Some(WebhookCommands::Create(args)) => create(helius, args.url, args.devnet, args.addresses).await,
    Some(WebhookCommands::Add(args)) => add(helius, args.id, args.addresses).await,
    Some(WebhookCommands::Delete(args)) => delete(helius, args.id).await,
  }
}
