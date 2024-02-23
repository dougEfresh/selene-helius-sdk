pub mod das;
pub mod enhanced_transactions;
mod name;
pub mod types;
pub mod webhook;

use crate::request_handler::RequestHandler;
use crate::Cluster;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

const API_URL_V0: &str = "https://api-mainnet.helius-rpc.com/v0";
const DEV_API_URL_V0: &str = "https://api-devnet.helius-rpc.com/v0";

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

fn rpc_url_from_cluster(api_key: &str, cluster: Cluster) -> String {
  match cluster {
    Cluster::MainnetBeta => format!("https://mainnet.helius-rpc.com/?api-key={api_key}"),
    Cluster::Devnet => format!("https://devnet.helius-rpc.com/?api-key={api_key}"),
  }
}

/// See [`HeliusBuilder`] to initialize this struct
#[derive(Clone)]
pub struct Helius {
  api_key: String,
  api_url: String,
  rpc_endpoint: Url,
  rpc: Arc<RpcClient>,
  handler: RequestHandler,
}

/// A builder to configure your [`Helius`] client
/// `HeliusBuilder` requires an api key from [https://dev.helius.xyz/dashboard/app](https://dev.helius.xyz/dashboard/app)
///
/// # Example
///```rust
/// use std::time::Duration;
/// let helius = selene_helius_sdk::HeliusBuilder::new("something")
///       .cluster(selene_helius_sdk::Cluster::Devnet)
///       .connect_timeout(Duration::from_secs(1))
///       .timeout(Duration::from_secs(1))
///       .http_client(reqwest::Client::new())
///       .build()
///       .expect("failed to create client");
/// ```
/// See also [`solana_client::nonblocking::rpc_client::RpcClient`] and [`solana_client::nonblocking::rpc_client::RpcClient::new_with_timeout_and_commitment`]
pub struct HeliusBuilder {
  api_key: String,
  cluster: Cluster,
  client: Option<reqwest::Client>,
  commitment_config: CommitmentConfig,
  timeout: Duration,
  connect_timeout: Duration,
}

impl HeliusBuilder {
  pub fn new(api_key: &str) -> Self {
    Self {
      api_key: api_key.to_string(),
      cluster: Cluster::MainnetBeta,
      commitment_config: CommitmentConfig::default(),
      timeout: Duration::from_secs(10),
      connect_timeout: Duration::from_secs(5),
      client: None,
    }
  }

  #[must_use]
  pub fn timeout(mut self, timeout: Duration) -> Self {
    self.timeout = timeout;
    self
  }

  #[must_use]
  pub fn connect_timeout(mut self, timeout: Duration) -> Self {
    self.connect_timeout = timeout;
    self
  }

  #[must_use]
  pub fn commit(mut self, commit: CommitmentConfig) -> Self {
    self.commitment_config = commit;
    self
  }

  #[must_use]
  pub fn cluster(mut self, cluster: Cluster) -> Self {
    self.cluster = cluster;
    self
  }

  #[must_use]
  pub fn http_client(mut self, client: reqwest::Client) -> Self {
    self.client = Some(client);
    self
  }

  #[allow(clippy::missing_errors_doc)]
  pub fn build(self) -> crate::Result<Helius> {
    let endpoint = rpc_url_from_cluster(&self.api_key, self.cluster);
    let rpc = RpcClient::new_with_timeout_and_commitment(endpoint, self.timeout, self.commitment_config);
    let client = match self.client {
      None => reqwest::ClientBuilder::new()
        .user_agent(APP_USER_AGENT)
        .connect_timeout(self.connect_timeout)
        .timeout(self.timeout)
        .build()?,
      Some(c) => c,
    };
    let api_url = String::from(match self.cluster {
      Cluster::MainnetBeta => API_URL_V0,
      Cluster::Devnet => DEV_API_URL_V0,
    });
    Ok(Helius {
      api_key: self.api_key.clone(),
      api_url,
      rpc_endpoint: Url::parse(&rpc_url_from_cluster(&self.api_key, self.cluster))?,
      rpc: Arc::new(rpc),
      handler: RequestHandler::new(client),
    })
  }
}

impl Helius {
  #[must_use]
  pub fn connection(&self) -> &RpcClient {
    &self.rpc
  }

  fn make_url(&self, method: &str) -> crate::Result<Url> {
    let u = format!("{}/{method}?api-key={}", self.api_url, self.api_key);
    Url::parse(&u).map_err(std::convert::Into::into)
  }
}

#[cfg(test)]
mod tests {
  use crate::api::DEV_API_URL_V0;
  use crate::Cluster::Devnet;
  use crate::HeliusBuilder;
  use std::time::Duration;

  #[test]
  fn helius_builder() {
    let helius = HeliusBuilder::new("something")
      .cluster(Devnet)
      .connect_timeout(Duration::from_secs(1))
      .timeout(Duration::from_secs(1))
      .http_client(reqwest::Client::new())
      .build()
      .expect("failed to create client");
    assert_eq!(helius.api_url, DEV_API_URL_V0);
  }
}
