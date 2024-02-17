use crate::{Helius, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Names {
  pub domain_names: Vec<String>,
}

impl Helius {
  /// # Errors
  ///
  /// Will return `HeliusError`
  pub async fn get_names(&self, address: &str) -> Result<Names> {
    let method = format!("addresses/{address}/names");
    self.handler.get(self.get_url_v0(method.as_str())?).await
  }
}
