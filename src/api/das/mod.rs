mod types;

use crate::Helius;
use crate::Result;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
pub use types::*;

impl Helius {
  async fn post<P, T>(&self, method: &str, params: P) -> Result<T>
  where
    P: Serialize + Sized + Debug + Send + Sync,
    T: DeserializeOwned + Default,
  {
    let request = Req::new(method, params);
    let res: Res<T> = self.handler.post(self.rpc_endpoint.clone(), &request).await?;
    Ok(res.result)
  }

  /// # Errors
  ///
  /// Will return `HeliusError`
  pub async fn get_asset(&self, params: &GetAssetParams) -> Result<GetAssetResponse> {
    self.post("getAsset", params).await
  }

  /// # Errors
  ///
  /// Will return `HeliusError`
  pub async fn get_asset_batch(&self, params: &GetAssetBatchParams) -> Result<Vec<GetAssetResponse>> {
    self.post("getAssetBatch", params).await
  }

  /// # Errors
  ///
  /// Will return `HeliusError`
  pub async fn get_asset_proof(&self, params: &GetAssetProofParams) -> Result<GetAssetProofResponse> {
    self.post("getAssetProof", params).await
  }

  /// # Errors
  ///
  /// Will return `HeliusError`
  pub async fn get_asset_proof_batch(
    &self,
    params: &GetAssetProofBatchParams,
  ) -> Result<HashMap<String, GetAssetProofResponse>> {
    self.post("getAssetProofBatch", params).await
  }

  /// # Errors
  ///
  /// Will return `HeliusError`
  pub async fn get_assets_by_owner(&self, params: &GetAssetsByOwnerParams) -> Result<GetAssetResponseList> {
    self.post("getAssetsByOwner", params).await
  }

  /// # Errors
  ///
  /// Will return `HeliusError`
  pub async fn get_assets_by_authority(&self, params: &GetAssetsByAuthorityParams) -> Result<GetAssetResponseList> {
    self.post("getAssetsByAuthority", params).await
  }

  /// # Errors
  ///
  /// Will return `HeliusError`
  pub async fn get_assets_by_creator(&self, params: &GetAssetsByCreatorParams) -> Result<GetAssetResponseList> {
    self.post("getAssetsByCreator", params).await
  }

  /// # Errors
  ///
  /// Will return `HeliusError`
  pub async fn get_assets_by_group(&self, params: &GetAssetsByGroupParams) -> Result<GetAssetResponseList> {
    self.post("getAssetsByGroup", params).await
  }

  /// # Errors
  ///
  /// Will return `HeliusError`
  pub async fn search_assets(&self, params: &SearchAssetsParams) -> Result<GetAssetResponseList> {
    self.post("searchAssets", params).await
  }

  /// # Errors
  ///
  /// Will return `HeliusError`
  pub async fn get_token_accounts(&self, params: &GetTokenAccountsParams) -> Result<GetTokenAccountsResponse> {
    self.post("getTokenAccounts", params).await
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
struct Req<T> {
  pub jsonrpc: String,
  pub id: String,
  pub method: String,
  pub params: T,
}

impl<T> Req<T> {
  pub fn new(method: &str, params: T) -> Self {
    Self { jsonrpc: "2.0".to_string(), id: "1".to_string(), method: String::from(method), params }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Res<T> {
  pub jsonrpc: String,
  pub id: String,
  pub result: T,
}
