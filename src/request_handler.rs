use crate::api::types::RpcError;
use crate::error::HeliusError;
use crate::Result;
use reqwest::{Client, Method};
use reqwest::{StatusCode, Url};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use tracing::debug;

#[derive(Clone)]
pub struct RequestHandler {
  pub http_client: Client,
}

impl RequestHandler {
  pub const fn new(client: Client) -> Self {
    Self { http_client: client }
  }

  #[allow(clippy::option_if_let_else)]
  async fn send<R, T>(&self, method: Method, url: Url, body: Option<&R>) -> Result<T>
  where
    R: Serialize + ?Sized + Debug + Send + Sync,
    T: DeserializeOwned + Default,
  {
    let path = String::from(url.path());
    #[cfg(not(debug))]
    debug!("sending request {method} {path}");

    #[cfg(debug)]
    match body {
      None => debug!("sending request {method} {path}"),
      Some(b) => {
        if let Ok(s) = serde_json::to_string(b) {
          debug!("sending request {method} {path} {:#?}", s);
        }
      },
    }

    let mut req = match method {
      Method::GET => self.http_client.get(url),
      Method::POST => self.http_client.post(url),
      Method::DELETE => self.http_client.delete(url),
      Method::PATCH => self.http_client.patch(url),
      Method::PUT => self.http_client.put(url),
      _ => todo!(),
    };

    if let Some(b) = body {
      req = req.json(b);
    }

    let resp = req.send().await?;
    let status_code = resp.status();
    let text = Self::handle_status(path, status_code, resp.text().await?)?;
    if text.is_empty() {
      return Ok(T::default());
    }
    match serde_json::from_str::<T>(&text) {
      Ok(deserialized) => Ok(deserialized),
      Err(err) => match serde_json::from_str::<RpcError>(&text) {
        Ok(rpc_error) => Err(HeliusError::RpcError { code: rpc_error.error.code, message: rpc_error.error.message }),
        Err(_) => Err(HeliusError::SerdeJson { err, text }),
      },
    }
  }

  fn handle_status(path: String, status: StatusCode, text: String) -> Result<String> {
    //TODO trim text, as it can be huge
    match status {
      StatusCode::OK | StatusCode::ACCEPTED | StatusCode::CREATED => Ok(text),
      StatusCode::NOT_FOUND => Err(HeliusError::NotFound { path }),
      StatusCode::BAD_REQUEST => Err(HeliusError::BadRequest { path, text }),
      StatusCode::UNAUTHORIZED => Err(HeliusError::Unauthorized { path, text }),
      StatusCode::TOO_MANY_REQUESTS => Err(HeliusError::TooManyRequests { path }),
      StatusCode::SERVICE_UNAVAILABLE | StatusCode::GATEWAY_TIMEOUT | StatusCode::INTERNAL_SERVER_ERROR => {
        Err(HeliusError::InternalError { code: status, text })
      },
      _ => Err(HeliusError::Unknown { code: status, text }),
    }
  }

  pub async fn get<T>(&self, url: Url) -> Result<T>
  where
    T: DeserializeOwned + Default,
  {
    self.send(Method::GET, url, None as Option<&str>).await
  }

  pub async fn post<R, T>(&self, url: Url, body: &R) -> Result<T>
  where
    R: Serialize + ?Sized + Debug + Send + Sync,
    T: DeserializeOwned + Default,
  {
    self.send(Method::POST, url, Some(body)).await
  }

  pub async fn put<R, T>(&self, url: Url, body: &R) -> Result<T>
  where
    R: Serialize + ?Sized + Debug + Sync + Send,
    T: DeserializeOwned + Default,
  {
    self.send(Method::PUT, url, Some(body)).await
  }

  pub async fn delete(&self, url: Url) -> Result<()> {
    self.send(Method::DELETE, url, None as Option<&str>).await
  }
}

#[cfg(test)]
mod tests {
  use crate::error::HeliusError;
  use crate::request_handler::RequestHandler;
  use color_eyre::eyre::format_err;
  use reqwest::StatusCode;

  #[test]
  fn handle_status_text() -> color_eyre::Result<()> {
    let body = String::from("body");
    let path = String::from("/");
    let text = RequestHandler::handle_status(path.clone(), StatusCode::OK, body.clone())?;
    assert_eq!(body, text);

    let text = RequestHandler::handle_status(path.clone(), StatusCode::ACCEPTED, body.clone())?;
    assert_eq!(body, text);

    let text = RequestHandler::handle_status(path.clone(), StatusCode::CREATED, body.clone())?;
    assert_eq!(body, text);

    let result = RequestHandler::handle_status(path.clone(), StatusCode::NOT_FOUND, body.clone());
    assert!(result.is_err());
    match result {
      Err(HeliusError::NotFound { .. }) => {},
      _ => return Err(format_err!("error should be NOT_FOUND ")),
    };

    let result = RequestHandler::handle_status(path.clone(), StatusCode::SERVICE_UNAVAILABLE, body.clone());
    assert!(result.is_err());
    match result {
      Err(HeliusError::InternalError { .. }) => {},
      _ => return Err(format_err!("error should be INTERNAL_ERROR ")),
    };

    let result = RequestHandler::handle_status(path.clone(), StatusCode::INTERNAL_SERVER_ERROR, body.clone());
    assert!(result.is_err());
    match result {
      Err(HeliusError::InternalError { .. }) => {},
      _ => return Err(format_err!("error should be INTERNAL_ERROR ")),
    };

    let result = RequestHandler::handle_status(path.clone(), StatusCode::BAD_REQUEST, body.clone());
    assert!(result.is_err());
    match result {
      Err(HeliusError::BadRequest { .. }) => {},
      _ => return Err(format_err!("error should be BadRequest ")),
    };

    let result = RequestHandler::handle_status(path.clone(), StatusCode::UNAUTHORIZED, body.clone());
    assert!(result.is_err());
    match result {
      Err(HeliusError::Unauthorized { .. }) => {},
      _ => return Err(format_err!("error should be UNAUTHORIZED ")),
    };

    let result = RequestHandler::handle_status(path.clone(), StatusCode::TOO_MANY_REQUESTS, body.clone());
    assert!(result.is_err());
    match result {
      Err(HeliusError::TooManyRequests { .. }) => {},
      _ => return Err(format_err!("error should be TOO_MANY_REQUESTS ")),
    };

    let result = RequestHandler::handle_status(path, StatusCode::IM_A_TEAPOT, body);
    assert!(result.is_err());
    match result {
      Err(HeliusError::Unknown { .. }) => {},
      _ => return Err(format_err!("error should be UNKNOWN")),
    };

    Ok(())
  }
}
