use std::fmt::format;
use std::str;
use crate::{structs::{Email, GetEmailsResponse, TokenResponse}, types::Result};
use reqwest::Client;

const BASE_URL: &str =  "https://gmail.googleapis.com";

pub async fn get_user_emails(token_response: TokenResponse) -> Result<()> {

  let client = Client::new();
  let messages_endpoint = format!("{}/gmail/v1/users/me/messages", BASE_URL);

  let email_response: GetEmailsResponse = serde_json::from_str(&client
    .get(&messages_endpoint)
    .header("Authorization", &format!("Bearer {}", token_response.get_access_token()))
    .send()
    .await?
    .text()
    .await?
  )?;

  for message in email_response.get_email_metadata() {

    let message_endpoint = format!("{}/gmail/v1/users/me/messages/{}", BASE_URL, message.get_id());

    let email: Email = serde_json::from_str(&client
      .get(&message_endpoint)
      .header("Authorization", &format!("Bearer {}", token_response.get_access_token()))
      .query(&[("format", "full")])
      .send()
      .await?
      .text()
      .await?
    )?;

    email.get_contents()?;
  }

  Ok(())
}