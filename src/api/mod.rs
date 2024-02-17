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
const DAS_URL: &str = "https://mainnet.helius-rpc.com";

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

fn rpc_url_from_cluster(api_key: &str, cluster: Cluster) -> String {
  match cluster {
    Cluster::MainnetBeta => format!("https://mainnet.helius-rpc.com/?api-key={api_key}"),
    Cluster::Devnet => format!("https://devnet.helius-rpc.com/?api-key={api_key}"),
  }
}

#[derive(Clone)]
pub struct Helius {
  api_key: String,
  cluster: Cluster,
  rpc: Arc<RpcClient>,
  handler: RequestHandler,
}

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

    Ok(Helius {
      api_key: self.api_key,
      cluster: self.cluster,
      rpc: Arc::new(rpc),
      handler: RequestHandler::new(client),
    })
  }
}

impl Helius {
  pub fn new(api_key: &str, cluster: Cluster, client: reqwest::Client) -> Self {
    let endpoint = rpc_url_from_cluster(api_key, cluster);
    let connection = RpcClient::new(endpoint);
    Self { api_key: String::from(api_key), cluster, rpc: Arc::new(connection), handler: RequestHandler::new(client) }
  }

  #[must_use]
  pub fn connection(&self) -> &RpcClient {
    &self.rpc
  }

  fn get_url_v0(&self, method: &str) -> crate::Result<Url> {
    let url = match self.cluster {
      Cluster::MainnetBeta => API_URL_V0,
      Cluster::Devnet => DEV_API_URL_V0,
    };
    self.make_url(url, method)
  }

  fn get_das_url(&self) -> crate::Result<Url> {
    self.make_url(DAS_URL, "")
  }

  fn make_url(&self, base: &str, method: &str) -> crate::Result<Url> {
    let u = format!("{base}/{method}?api-key={}", self.api_key);
    Url::parse(&u).map_err(std::convert::Into::into)
  }
}
