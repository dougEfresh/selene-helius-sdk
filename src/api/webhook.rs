pub use crate::api::types::webhook::*;
use crate::{Helius, Result};

#[allow(dead_code)]
const MAX_WEBHOOK_ADDRESSES: usize = 100_000;
const WEBHOOK_BASE: &str = "webhooks";

impl Helius {
  /// # Errors
  ///
  /// Will return `HeliusError`
  pub async fn get_all_webhooks(&self) -> Result<Vec<Webhook>> {
    self.handler.get(self.get_url_v0(WEBHOOK_BASE)?).await
  }

  /// # Errors
  ///
  /// Will return `HeliusError`
  pub async fn get_webhook_by_id(&self, webhook_id: &str) -> Result<Webhook> {
    self.handler.get(self.get_url_v0(format!("{WEBHOOK_BASE}/{webhook_id}").as_str())?).await
  }

  /// # Errors
  ///
  /// Will return `HeliusError`
  pub async fn create_webhook(&self, request: &CreateWebhookRequest) -> Result<Webhook> {
    self.handler.post(self.get_url_v0(WEBHOOK_BASE)?, request).await
  }

  /// # Errors
  ///
  /// Will return `HeliusError`
  pub async fn edit_webhook(&self, request: &EditWebhookRequest) -> Result<Webhook> {
    self.handler.put(self.get_url_v0(format!("{WEBHOOK_BASE}/{}", request.webhook_id).as_str())?, &request.data).await
  }

  /// # Errors
  ///
  /// Will return `HeliusError`
  pub async fn delete_webhook(&self, webhook_id: &str) -> Result<()> {
    self.handler.delete(self.get_url_v0(format!("{WEBHOOK_BASE}/{webhook_id}").as_str())?).await
  }

  /// # Errors
  ///
  /// Will return `HeliusError`
  pub async fn append_addresses_to_webhook(&self, webhook_id: &str, new_addresses: &[String]) -> Result<Webhook> {
    let mut webhook = self.get_webhook_by_id(webhook_id).await?;
    webhook.webhook_data.account_addresses.extend(new_addresses.to_vec());
    let edit_request = EditWebhookRequest { webhook_id: webhook_id.to_string(), data: webhook.webhook_data };
    self.edit_webhook(&edit_request).await
  }
}
