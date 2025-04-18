use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResponse {
  access_token: String,
  expires_in: u32,
  refresh_token: String,
  scope: String,
  token_type: String,
  id_token: String
}

impl TokenResponse {
  pub fn get_access_token(&self) -> &str {
    &self.access_token
  }
}

// Taken from https://stackoverflow.com/a/76889764/14185351, just useful boilerplate for working with anyhow and actix
#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("an unspecified internal error occurred: {0}")]
  InternalError(#[from] anyhow::Error),
  #[error("HTTP request failed: {0}")]
  ReqwestError(#[from] reqwest::Error),
  #[error("JSON parse error: {0}")]
  JsonError(#[from] serde_json::Error),
  #[error("Actix error: {0}")]
  ActixError(#[from] actix_web::Error),
  #[error("Stdio error: {0}")]
  StdioError(#[from] std::io::Error),
}

impl ResponseError for Error {
  fn status_code(&self) -> StatusCode {
    match &self {
      Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Self::ReqwestError(_) => StatusCode::BAD_GATEWAY,
      Self::JsonError(_) => StatusCode::BAD_REQUEST,
      Self::ActixError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Self::StdioError(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
  fn error_response(&self) -> HttpResponse {
    HttpResponse::build(self.status_code()).body(self.to_string())
  }  
}
