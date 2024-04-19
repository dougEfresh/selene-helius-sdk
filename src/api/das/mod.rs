pub mod fee;
mod types;

use crate::Result;
use crate::{error, Helius};
use bincode::serialize;
pub use fee::{
  AllFeeLevelsRequest, FeeLevelRequest, GetPriorityFeeEstimateOptions, GetPriorityFeeEstimateRequest,
  GetPriorityFeeEstimateResponse, MicroLamportPriorityFee, MicroLamportPriorityFeeLevels, PriorityLevel,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::SerializableTransaction;
use std::collections::HashMap;
use std::fmt::Debug;
pub use types::*;

impl Helius {
  #[tracing::instrument(skip(self, params))]
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
  /// Will return [`crate::HeliusError`]
  pub async fn get_asset(&self, params: &GetAssetParams) -> Result<Option<GetAssetResponse>> {
    self.post("getAsset", params).await
  }

  /// # Errors
  ///
  /// Will return [`crate::HeliusError`]
  pub async fn get_asset_batch(&self, params: &GetAssetBatchParams) -> Result<Vec<Option<GetAssetResponse>>> {
    self.post("getAssetBatch", params).await
  }

  /// # Errors
  ///
  /// Will return [`crate::HeliusError`]
  pub async fn get_asset_proof(&self, params: &GetAssetProofParams) -> Result<Option<GetAssetProofResponse>> {
    self.post("getAssetProof", params).await
  }

  /// # Errors
  ///
  /// Will return [`crate::HeliusError`]
  pub async fn get_asset_proof_batch(
    &self,
    params: &GetAssetProofBatchParams,
  ) -> Result<HashMap<String, GetAssetProofResponse>> {
    self.post("getAssetProofBatch", params).await
  }

  /// # Errors
  ///
  /// Will return [`crate::HeliusError`]
  pub async fn get_assets_by_owner(&self, params: &GetAssetsByOwnerParams) -> Result<GetAssetResponseList> {
    self.post("getAssetsByOwner", params).await
  }

  /// # Errors
  ///
  /// Will return [`crate::HeliusError`]
  pub async fn get_assets_by_authority(&self, params: &GetAssetsByAuthorityParams) -> Result<GetAssetResponseList> {
    self.post("getAssetsByAuthority", params).await
  }

  /// # Errors
  ///
  /// Will return [`crate::HeliusError`]
  pub async fn get_assets_by_creator(&self, params: &GetAssetsByCreatorParams) -> Result<GetAssetResponseList> {
    self.post("getAssetsByCreator", params).await
  }

  /// # Errors
  ///
  /// Will return [`crate::HeliusError`]
  pub async fn get_assets_by_group(&self, params: &GetAssetsByGroupParams) -> Result<GetAssetResponseList> {
    self.post("getAssetsByGroup", params).await
  }

  /// # Errors
  ///
  /// Will return [`crate::HeliusError`]
  pub async fn search_assets(&self, params: &SearchAssetsParams) -> Result<GetAssetResponseList> {
    self.post("searchAssets", params).await
  }

  /// # Errors
  ///
  /// Will return [`crate::HeliusError`]
  pub async fn get_token_accounts(&self, params: &GetTokenAccountsParams) -> Result<GetTokenAccountsResponse> {
    self.post("getTokenAccounts", params).await
  }

  async fn call_estimate_priority_fee(
    &self,
    params: &GetPriorityFeeEstimateRequest,
  ) -> Result<GetPriorityFeeEstimateResponse> {
    self.post("getPriorityFeeEstimate", vec![params]).await
  }

  /// [priority fee estimate](https://docs.helius.dev/solana-rpc-nodes/alpha-priority-fee-api#priority-fee-estimate) returning a range
  ///  
  /// # Errors
  ///
  /// Will return [`crate::HeliusError`]
  pub async fn get_estimate_priority_fee_levels(&self, accounts: Vec<String>) -> Result<MicroLamportPriorityFeeLevels> {
    let req = GetPriorityFeeEstimateRequest {
      transaction: None,
      account_keys: accounts,
      options: GetPriorityFeeEstimateOptions::AllFeeLevels(AllFeeLevelsRequest::default()),
    };
    match self.call_estimate_priority_fee(&req).await? {
      GetPriorityFeeEstimateResponse::Estimate(e) => {
        Err(error::HeliusError::InvalidFeeResponse { response: format!("{e:#?}") })
      },
      GetPriorityFeeEstimateResponse::Levels(r) => Ok(r),
    }
  }

  /// [priority fee estimate](https://docs.helius.dev/solana-rpc-nodes/alpha-priority-fee-api#priority-fee-estimate) for a given [level](fee::PriorityLevel)
  ///
  /// # Errors
  ///
  /// Will return [`crate::HeliusError`]
  pub async fn get_estimate_priority_fee(
    &self,
    accounts: Vec<String>,
    lvl: PriorityLevel,
  ) -> Result<MicroLamportPriorityFee> {
    let req = GetPriorityFeeEstimateRequest {
      transaction: None,
      account_keys: accounts,
      options: GetPriorityFeeEstimateOptions::Priority(FeeLevelRequest { priority_level: lvl }),
    };
    match self.call_estimate_priority_fee(&req).await? {
      GetPriorityFeeEstimateResponse::Estimate(e) => Ok(e),
      GetPriorityFeeEstimateResponse::Levels(r) => {
        Err(error::HeliusError::InvalidFeeResponse { response: format!("{r:#?}") })
      },
    }
  }

  /// [priority fee estimate](https://docs.helius.dev/solana-rpc-nodes/alpha-priority-fee-api#priority-fee-estimate) for a [`solana_sdk::transaction::Transaction`]
  ///
  /// # Errors
  ///
  /// Will return [`crate::HeliusError`]
  pub async fn get_estimate_priority_fee_transaction<T: SerializableTransaction + Sync>(
    &self,
    transaction: &T,
    lvl: PriorityLevel,
  ) -> Result<MicroLamportPriorityFee> {
    let s = bs58::encode(serialize(transaction)?).into_string();
    let req = GetPriorityFeeEstimateRequest {
      transaction: Some(s),
      account_keys: Vec::new(),
      options: GetPriorityFeeEstimateOptions::Priority(FeeLevelRequest { priority_level: lvl }),
    };
    match self.call_estimate_priority_fee(&req).await? {
      GetPriorityFeeEstimateResponse::Estimate(e) => Ok(e),
      GetPriorityFeeEstimateResponse::Levels(r) => {
        Err(error::HeliusError::InvalidFeeResponse { response: format!("{r:#?}") })
      },
    }
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
