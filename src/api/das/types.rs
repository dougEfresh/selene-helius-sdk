use crate::api::types::{
  AssetSortBy, AssetSortDirection, Context, Interface, OwnershipModel, RoyaltyModel, Scope, TokenType, UseMethods,
};
use bigdecimal::{BigDecimal, Zero};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Pagination {
  pub page: u32,
  pub limit: Option<u32>,
  pub before: Option<u32>,
  pub after: Option<u32>,
}

impl Default for Pagination {
  fn default() -> Self {
    Self { page: 1, limit: None, before: None, after: None }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchAssetsParams {
  #[serde(flatten)]
  pub pagination: Pagination,
  pub sort_by: Option<AssetSortingRequest>,
  pub creator_address: Option<String>,
  pub owner_address: Option<String>,
  pub json_uri: Option<String>,
  pub grouping: Option<Vec<String>>,
  pub burnt: Option<bool>,
  pub frozen: Option<bool>,
  pub supply_mint: Option<String>,
  pub supply: Option<u32>,
  pub interface: Option<Interface>,
  pub token_type: Option<TokenType>,
  pub delegate: Option<u32>,
  pub owner_type: Option<OwnershipModel>,
  pub royalty_amount: Option<u32>,
  pub royalty_target: Option<String>,
  pub royalty_target_type: Option<RoyaltyModel>,
  pub compressible: Option<bool>,
  pub compressed: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetsByGroupParams {
  pub group_value: String,
  pub group_key: String,
  #[serde(flatten)]
  pub pagination: Pagination,
  pub display_options: Option<DisplayOptions>,
  pub sort_by: Option<AssetSortingRequest>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetsByCreatorParams {
  pub creator_address: String,
  #[serde(flatten)]
  pub pagination: Pagination,
  pub only_verified: bool,
  pub display_options: Option<DisplayOptions>,
  pub sort_by: Option<AssetSortingRequest>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetsByAuthorityParams {
  pub authority_address: String,
  #[serde(flatten)]
  pub pagination: Pagination,
  pub display_options: Option<DisplayOptions>,
  pub sort_by: Option<AssetSortingRequest>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetsByOwnerParams {
  pub owner_address: String,
  #[serde(flatten)]
  pub pagination: Pagination,
  pub display_options: Option<DisplayOptions>,
  pub sort_by: Option<AssetSortingRequest>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AssetSortingRequest {
  pub sort_by: AssetSortBy,
  pub sort_direction: AssetSortDirection,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct GetAssetProofParams {
  pub id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct GetAssetProofBatchParams {
  pub ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct GetAssetProofResponse {
  pub root: String,
  pub proof: Vec<String>,
  pub node_index: u32,
  pub leaf: String,
  pub tree_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct GetAssetParams {
  pub id: String,
  #[serde[rename = "displayOptions"]]
  pub display_options: Option<DisplayOptions>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct GetAssetBatchParams {
  pub ids: Vec<String>,
  #[serde[rename = "displayOptions"]]
  pub display_options: Option<DisplayOptions>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct GetAssetResponse {
  pub interface: Interface,
  pub id: String,
  pub content: Option<Content>,
  pub authorities: Option<Vec<Authorities>>,
  pub compression: Option<Compression>,
  pub grouping: Option<Vec<Grouping>>,
  pub royalty: Option<Royalty>,
  pub ownership: Ownership,
  pub creators: Option<Vec<Creators>>,
  pub uses: Option<Uses>,
  pub supply: Option<Supply>,
  pub mutable: bool,
  pub burnt: bool,
  pub token_info: Option<TokenInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct GetAssetResponseList {
  pub grand_total: Option<bool>,
  pub total: u32,
  pub limit: u32,
  pub page: u32,
  pub items: Vec<GetAssetResponse>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DisplayOptions {
  pub show_fungible: bool,
  pub show_inscription: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct TokenAccountDisplayOptions {
  pub show_zero_balance: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Ownership {
  pub frozen: bool,
  pub delegated: bool,
  pub delegate: Option<String>,
  pub ownership_model: OwnershipModel,
  pub owner: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Supply {
  pub print_max_supply: u32,
  pub print_current_supply: u32,
  pub edition_nonce: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Uses {
  pub use_method: UseMethods,
  pub remaining: u32,
  pub total: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Creators {
  pub address: String,
  pub share: u32,
  pub verified: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Royalty {
  pub royalty_model: RoyaltyModel,
  pub target: Option<String>,
  pub percent: f32,
  pub basis_points: u32,
  pub primary_sale_happened: bool,
  pub locked: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Grouping {
  pub group_key: String,
  pub group_value: String,
  pub verified: Option<bool>,
  pub collection_metadata: Option<CollectionMetadata>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct CollectionMetadata {
  pub name: Option<String>,
  pub symbol: Option<String>,
  pub image: Option<String>,
  pub description: Option<String>,
  pub external_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Authorities {
  pub address: String,
  pub scopes: Vec<Scope>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Links {
  pub external_url: Option<String>,
  pub image: Option<String>,
  pub animation_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Content {
  #[serde(rename = "$schema")]
  pub schema: String,
  pub json_uri: String,
  pub files: Option<Vec<File>>,
  pub metadata: Metadata,
  pub links: Links,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct File {
  pub uri: Option<String>,
  pub mime: Option<String>,
  pub cdn_uri: Option<String>,
  pub quality: Option<FileQuality>,
  pub contexts: Option<Vec<Context>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct FileQuality {
  pub schema: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Metadata {
  pub attributes: Option<Vec<Attribute>>,
  pub description: Option<String>,
  #[serde(default)]
  pub name: String,
  #[serde(default)]
  pub symbol: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Attribute {
  pub value: Value,
  pub trait_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct Compression {
  pub eligible: bool,
  pub compressed: bool,
  pub data_hash: String,
  pub creator_hash: String,
  pub asset_hash: String,
  pub tree: String,
  pub seq: u32,
  pub leaf_id: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct PriceInfo {
  pub price_per_token: BigDecimal,
  pub total_price: BigDecimal,
  pub currency: String,
}

impl Default for PriceInfo {
  fn default() -> Self {
    Self { price_per_token: BigDecimal::zero(), total_price: BigDecimal::zero(), currency: String::from("USDC") }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(default)]
pub struct TokenInfo {
  pub symbol: String,
  pub balance: u64,
  pub supply: u64,
  pub decimals: i32,
  pub token_program: String,
  pub associated_token_address: String,
  pub price_info: PriceInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct TokenAccount {
  pub address: String,
  pub mint: String,
  pub owner: String,
  pub amount: u64,
  pub delegated_amount: u64,
  pub frozen: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
pub struct GetTokenAccountsResponse {
  pub total: u32,
  pub limit: u32,
  pub page: u32,
  pub token_accounts: Vec<TokenAccount>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GetTokenAccountsParams {
  pub page: u32,
  pub limit: Option<u32>,
  pub display_options: TokenAccountDisplayOptions,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub owner: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub mint: Option<String>,
}

impl Default for GetTokenAccountsParams {
  fn default() -> Self {
    Self { page: 1, limit: None, display_options: TokenAccountDisplayOptions::default(), owner: None, mint: None }
  }
}
