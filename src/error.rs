use reqwest::StatusCode;
use thiserror::Error;
use url::ParseError;

#[derive(Debug, Error)]
pub enum HeliusError {
  #[error("Deserialization Error: {err}. Response: {text}")]
  /// Serde JSON Error
  SerdeJson { err: serde_json::Error, text: String },

  #[error(transparent)]
  /// Thrown when submitting a POST/GET request fails
  ReqwestError(#[from] reqwest::Error),

  #[error(transparent)]
  UrlError(#[from] ParseError),

  #[error("Internal Error. HTTP Code {code} {text}")]
  InternalError { code: StatusCode, text: String },

  #[error("{path} not found. ")]
  NotFound { path: String },

  #[error("Bad Request for {path} {text} ")]
  BadRequest { path: String, text: String },

  #[error("Unauthorized for {path} {text} ")]
  Unauthorized { path: String, text: String },

  #[error("Unknown Error HTTP Code: {code} {text}")]
  Unknown { code: StatusCode, text: String },

  #[error("RPC Error  code:{code} message:{message}")]
  RpcError { code: i32, message: String },

  #[error(transparent)]
  SolanaClientError(#[from] solana_client::client_error::ClientError),

  #[error("Too Many Requests: {path}")]
  TooManyRequests { path: String },

  #[error("Invalid fee response type {response}")]
  InvalidFeeResponse { response: String },

  #[error(transparent)]
  TransactionEncodeError(#[from] bincode::Error),
}
