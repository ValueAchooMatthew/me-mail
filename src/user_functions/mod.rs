use crate::{structs::TokenResponse, types::Result};
use reqwest::Client;

const BASE_URL: &str =  "https://gmail.googleapis.com";

pub async fn get_user_emails(token_response: TokenResponse) -> Result<()> {

  let client = Client::new();
  let endpoint = format!("{}/gmail/v1/users/me/messages", BASE_URL);

  let user_id = client
    .get(&endpoint)
    .header("Authorization", &format!("Bearer {}", token_response.get_access_token()))
    .send()
    .await?;
  println!("{:#?}", user_id.text().await?);

  Ok(())
}

// pub async fn get_user_id(token_response: TokenResponse) {



// }