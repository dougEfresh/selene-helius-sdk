use crate::api::types::{ProgramName, Source, TokenStandard, TransactionContext, TransactionType};
use crate::util::deserialize_str_to_number;
use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EnhancedTransaction {
  pub account_data: Vec<AccountData>,
  pub description: String,
  #[serde(rename = "type")]
  pub transaction_type: TransactionType,
  pub source: Source,
  pub fee: i32,
  pub fee_payer: String,
  pub signature: String,
  pub slot: i32,
  pub native_transfers: Option<Vec<NativeTransfer>>,
  pub token_transfers: Option<Vec<TokenTransfer>>,
  pub transaction_error: Option<TransactionError>,
  pub instructions: Vec<Instruction>,
  pub events: TransactionEvent,
  pub timestamp: u64,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParseTransactionsRequest {
  pub transactions: Vec<String>,
}

impl ParseTransactionsRequest {
  /// Split the signatures into 100 vec sized chunks.
  /// Helius has a limit of 100 transactions per call
  pub fn from_slice(signatures: &[String]) -> Vec<Self> {
    signatures.chunks(100).map(|chunk| Self { transactions: chunk.to_vec() }).collect()
  }
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEvent {
  pub nft: Option<NFTEvent>,
  pub swap: Option<SwapEvent>,
  pub compressed: Option<Vec<CompressedNftEvent>>,
  pub set_authority: Option<Vec<Authority>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompressedNftEvent {
  #[serde(rename = "type")]
  pub transaction_type: TransactionType,
  pub tree_id: String,
  pub leaf_index: Option<i32>,
  pub seq: Option<i32>,
  pub asset_id: String,
  pub instruction_index: Option<i32>,
  pub inner_instruction_index: Option<i32>,
  pub new_leaf_owner: Option<String>,
  pub old_leaf_owner: Option<String>,
  pub new_leaf_delegate: Option<String>,
  pub old_leaf_delegate: Option<serde_json::Value>,
  pub tree_delegate: Option<String>,
  pub metadata: Option<Metadata>,
  pub update_args: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SwapEvent {
  pub native_input: Option<NativeBalanceChange>,
  pub native_output: Option<NativeBalanceChange>,
  pub token_inputs: Vec<TokenBalanceChange>,
  pub token_outputs: Vec<TokenBalanceChange>,
  pub token_fees: Vec<TokenBalanceChange>,
  pub native_fees: Vec<NativeBalanceChange>,
  pub inner_swaps: Vec<TokenSwap>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenSwap {
  pub native_input: Option<NativeTransfer>,
  pub native_output: Option<NativeTransfer>,
  pub token_inputs: Vec<TokenTransfer>,
  pub token_outputs: Vec<TokenTransfer>,
  pub token_fees: Vec<TokenTransfer>,
  pub native_fees: Vec<NativeTransfer>,
  pub program_info: ProgramInfo,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProgramInfo {
  pub source: Source,
  pub account: String,
  pub program_name: ProgramName,
  pub instruction_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NFTEvent {
  pub seller: String,
  pub buyer: String,
  pub timestamp: Number,
  pub amount: Number,
  pub fee: Number,
  pub signature: String,
  pub source: Source,
  #[serde(rename = "type")]
  pub transaction_type: TransactionType,
  pub sale_type: TransactionContext,
  pub nfts: Vec<Token>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Token {
  pub mint: String,
  pub token_standard: TokenStandard,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransactionError {
  #[serde(rename = "InstructionError")]
  pub instruciton_error: serde_json::Value,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NativeBalanceChange {
  pub account: String,
  #[serde(deserialize_with = "deserialize_str_to_number")]
  pub amount: Number,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountData {
  pub account: String,
  pub native_balance_change: Number,
  pub token_balance_changes: Option<Vec<TokenBalanceChange>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalanceChange {
  pub user_account: String,
  pub token_account: String,
  pub raw_token_amount: RawTokenAmount,
  pub mint: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RawTokenAmount {
  pub token_amount: String,
  pub decimals: Number,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenTransfer {
  #[serde(flatten)]
  pub user_accounts: TransferUserAccounts,
  pub from_token_account: Option<String>,
  pub to_token_account: Option<String>,
  pub token_amount: Number,
  pub token_standard: TokenStandard,
  pub mint: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransferUserAccounts {
  pub from_user_account: Option<String>,
  pub to_user_account: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NativeTransfer {
  #[serde(flatten)]
  pub user_accounts: TransferUserAccounts,
  pub amount: Number,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Instruction {
  pub accounts: Vec<String>,
  pub data: String,
  pub program_id: String,
  pub inner_instructions: Vec<InnerInstruction>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InnerInstruction {
  pub accounts: Vec<String>,
  pub data: String,
  pub program_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
  pub key: String,
  pub verified: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
  pub name: String,
  pub symbol: String,
  pub uri: String,
  pub seller_fee_basis_points: i32,
  pub primary_sale_happened: bool,
  #[serde(rename = "isMutable")]
  pub mutable: bool,
  pub edition_nonce: Option<i32>,
  pub token_standard: Option<String>,
  pub collection: Option<Collection>,
  pub token_program_version: String,
  pub creators: Option<Vec<serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Authority {
  pub account: String,
  pub from: String,
  pub to: String,
  pub instruction_index: Option<i32>,
  pub inner_instruction_index: Option<i32>,
}
