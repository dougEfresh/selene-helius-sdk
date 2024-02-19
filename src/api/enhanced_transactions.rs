use crate::api::types::enhanced::{EnhancedTransaction, ParseTransactionsRequest};
use crate::{Helius, Result};

impl Helius {
  /// # Errors
  ///
  /// Will return `HeliusError`
  pub async fn parse_transaction(&self, transactions: &ParseTransactionsRequest) -> Result<Vec<EnhancedTransaction>> {
    self.handler.post(self.make_url("transactions")?, transactions).await
  }
}
