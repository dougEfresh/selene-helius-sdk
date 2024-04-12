use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPriorityFeeEstimateRequest {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub transaction: Option<String>, // estimate fee for a serialized txn
  #[serde(skip_serializing_if = "Vec::is_empty")]
  pub account_keys: Vec<String>, // estimate fee for a list of accounts
  pub options: GetPriorityFeeEstimateOptions,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum GetPriorityFeeEstimateOptions {
  AllFeeLevels(AllFeeLevelsRequest),
  Priority(FeeLevelRequest),
}

impl Default for GetPriorityFeeEstimateOptions {
  fn default() -> Self {
    Self::AllFeeLevels(AllFeeLevelsRequest::default())
  }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AllFeeLevelsRequest {
  include_all_priority_fee_levels: bool,
  pub lookback_slots: u8, // number of slots to look back to calculate estimate. Valid number are 1-150, defualt is 150
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct FeeLevelRequest {
  pub priority_level: PriorityLevel, // Default to MEDIUM
}

impl Default for AllFeeLevelsRequest {
  fn default() -> Self {
    Self { include_all_priority_fee_levels: true, lookback_slots: 150 }
  }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PriorityLevel {
  Zero, // 0th percentile
  Low,  // 25th percentile
  #[default]
  Medium, // 50th percentile
  High, // 75th percentile
  VeryHigh, // 95th percentile
  // labelled unsafe to prevent people using and draining their funds by accident
  UnsafeMax, // 100th percentile
  Avg,       // 50th percentile
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GetPriorityFeeEstimateResponse {
  #[serde(rename = "priorityFeeEstimate")]
  Estimate(MicroLamportPriorityFee),
  #[serde(rename = "priorityFeeLevels")]
  Levels(MicroLamportPriorityFeeLevels),
}

impl Default for GetPriorityFeeEstimateResponse {
  fn default() -> Self {
    Self::Estimate(Default::default())
  }
}

type MicroLamportPriorityFee = f64;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MicroLamportPriorityFeeLevels {
  pub low: f64,
  pub medium: f64,
  pub high: f64,
  pub very_high: f64,
  pub unsafe_max: f64,
}
