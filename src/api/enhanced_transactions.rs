use crate::api::types::enhanced::{EnhancedTransaction, ParseTransactionsRequest};
use crate::{Helius, Result};

impl Helius {
  /// # Errors
  ///
  /// Will return [`crate::HeliusError`]
  #[tracing::instrument(skip(self, transactions))]
  pub async fn parse_transaction(&self, transactions: &ParseTransactionsRequest) -> Result<Vec<EnhancedTransaction>> {
    self.handler.post(self.make_url("transactions")?, transactions).await
  }

  /// # Errors
  ///
  /// Will return [`crate::HeliusError`]
  #[tracing::instrument(skip(self))]
  pub async fn parsed_transaction_history(&self, address: &str) -> Result<Vec<EnhancedTransaction>> {
    let method = format!("addresses/{address}/transactions");
    let url = self.make_url(&method)?;
    self.handler.get(url).await
  }
}
