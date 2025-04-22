use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use std::str;
use crate::types::Result;

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
  #[error("Base64-url decoding error: {0}")]
  Base64UrlDecodingError(#[from] base64_url::base64::DecodeError),
  #[error("UTF-8 ecoding error: {0}")]
  UTF8EncodingError(#[from] std::str::Utf8Error),

}

impl ResponseError for Error {
  fn status_code(&self) -> StatusCode {
    match &self {
      Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Self::ReqwestError(_) => StatusCode::BAD_GATEWAY,
      Self::JsonError(_) => StatusCode::BAD_REQUEST,
      _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
  fn error_response(&self) -> HttpResponse {
    HttpResponse::build(self.status_code()).body(self.to_string())
  }  
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmailMetadata {
  id: String,
  threadId: String
}

impl EmailMetadata {
  pub fn get_id(self) -> String {
    self.id
  }  
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetEmailsResponse {
  messages: Vec<EmailMetadata>
}

impl GetEmailsResponse {
  pub fn get_email_metadata(self) -> Vec<EmailMetadata> {
    self.messages
  }
}

// Format described here: https://developers.google.com/workspace/gmail/api/reference/rest/v1/users.messages

#[derive(Serialize, Deserialize, Debug)]
pub struct Email {
  id: String,
  threadId: String,
  labelIds: Vec<String>,
  snippet: String,
  historyId: String,
  internalDate: String,
  sizeEstimate: u32,
  payload: MessagePart,
  // pub raw: String
}

#[derive(Serialize, Deserialize, Debug)]
struct MessagePart {
  partId: String,
  mimeType: String,
  filename: String,
  headers: Vec<Header>,
  body: MessagePartBody,
  parts: Option<Vec<MessagePart>>
}

#[derive(Serialize, Deserialize, Debug)]
struct MessagePartBody {
  attachmentId: Option<String>,
  size: u32,
  data: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct Header {
  name: String,
  value: String
}

impl Email {
  pub fn get_contents(self) -> Result<()> {

    if let Some(data) = &self.payload.body.data {
      let html = str::from_utf8(&base64_url::decode(&self.remove_padding(&data))?)?.to_owned();
      println!("{html:?}")
    }

    if let Some(parts) = self.payload.parts {
      println!("{:?}", parts);
    }

    Ok(())
  }

  // data in body is base-64 url encoded but contains base 64 padding at end which messes up decoding process
  // fixed by removing equal signs
  fn remove_padding(&self, encoded_string: &str) -> String {
    encoded_string.chars().filter(|c| c != &'=').collect()
  }

}