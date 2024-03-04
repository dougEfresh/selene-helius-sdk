use crate::api::types::{AccountWebhookEncoding, CollectionIdentifier, TransactionType, TxnStatus};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
  #[serde(rename = "webhookID")]
  pub webhook_id: String,
  pub wallet: String,
  #[serde(flatten)]
  pub webhook_data: WebhookData,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct WebhookData {
  #[serde(rename = "webhookURL")]
  pub webhook_url: String,
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub transaction_types: Vec<TransactionType>,
  pub account_addresses: Vec<String>,
  pub webhook_type: WebhookType,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub auth_header: Option<String>,
  #[serde(default)]
  pub txn_status: TxnStatus,
  #[serde(default)]
  pub encoding: AccountWebhookEncoding,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Clone, Debug, Default)]
pub enum WebhookType {
  #[serde(rename = "enhanced")]
  #[default]
  Enhanced,
  #[serde(rename = "enhancedDevnet")]
  EnhancedDevnet,
  #[serde(rename = "raw")]
  Raw,
  #[serde(rename = "rawDevnet")]
  RawDevnet,
  #[serde(rename = "discord")]
  Discord,
  #[serde(rename = "discordDevnet")]
  DiscordDevnet,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct CreateWebhookRequest {
  #[serde(flatten)]
  pub data: WebhookData,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateCollectionWebhookRequest {
  #[serde(flatten)]
  pub data: WebhookData,
  pub collection_query: CollectionIdentifier,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EditWebhookRequest {
  pub webhook_id: String,
  pub data: WebhookData,
}
