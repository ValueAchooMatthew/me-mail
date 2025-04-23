use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use scraper::{node, Html};
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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MessagePart {
  partId: String,
  mimeType: String,
  filename: String,
  headers: Headers,
  body: MessagePartBody,
  parts: Option<Vec<MessagePart>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MessagePartBody {
  attachmentId: Option<String>,
  size: u32,
  data: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Header {
  name: String,
  value: String
}

impl Email {
  pub fn get_contents(self) -> Result<()> {

    // Forwarded emails have no body. 
    if let Some(data) = &self.get_payload().get_desired_email(
      "Nabil Aldihni <aldihninabil@gmail.com>", 
      "Fwd: Your Enbridge Gas ebill is ready for viewing"
    ) {

      let data = &data.body.data.clone().unwrap();

      let html = Html::parse_fragment(
        str::from_utf8(&base64_url::decode(&self.remove_padding(&data))?)?
      );
      for node in html.tree {
        match node {
          node::Node::Text(text) => {
            if text.contains("Account Balance") {

              for fragment in text.split("\n").into_iter() {
                if fragment.contains("$") {
                  let amount_owed = &fragment[1..];
                  println!("Amount owed: {amount_owed}")
                }
              }
            }
          },
          _ => ()

        }
      }
    }

    // if let Some(parts) = self.payload.parts {
      // println!("{:?}", parts);
    // }

    Ok(())
  }

  // data in body is base-64 url encoded but contains base 64 padding at end which messes up decoding process
  // fixed by removing equal signs
  fn remove_padding(&self, encoded_string: &str) -> String {
    encoded_string.chars().filter(|c| c != &'=').collect()
  }

  fn get_payload(&self) -> MessagePart {
    self.payload.to_owned()
  }

  // Emails are forwarded by including the original email to be forwarded as a MessagePart in the parts vec
  // Of a new email sent to the recipient. Thus, to check a user's inbox for a desired email, we must check
  // All emails as well as their parts to see if it matches the criteria.
}


// Flawed aproach since if forwarded message has the same subject line as the original message,
// Will fire anwyays. But whatever
impl MessagePart {
  fn get_desired_email(&self, sender_email: &str, subject_line: &str) -> Option<Self> {
    if self.headers.is_message_sent_from_sender(sender_email, subject_line) {
      // Shittiest code ever written but working with emails sucks
      if self.headers.does_message_have_attachments() {
        return Some(self.parts.clone().unwrap().get(0).unwrap().clone())
      } else {
        return Some(self.clone());
      }
    }
    None
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Headers(Vec<Header>);

impl Headers {
  pub fn is_message_sent_from_sender(&self, sender_email: &str, subject_line: &str) -> bool {

    let mut sender_match = false;
    let mut subject_match = false;
    
    for header in self.0.iter() {
      if header.name == "From" && header.value == sender_email {
        sender_match = true;
      } else if header.name == "Subject" && header.value == subject_line {
        subject_match = true
      }
    };
    subject_match && sender_match
  }

  pub fn does_message_have_attachments(&self) -> bool {

    for header in self.0.iter() {
      if header.name == "Content-Type" && header.value.contains("multipart/alternative"){
        return true;
      }
    }
    false
  }
}