use actix_web::{get, HttpRequest, HttpResponse, Responder};
use reqwest::Client;
use crate::{
    structs::TokenResponse, 
    utils::{
        get_client_id, 
        get_client_secret, 
        retrieve_value_from_query_key
    },
    types::Result,
    user_functions::get_user_emails,
    REDIRECT_URI
};

#[get("/signin-google")]
pub async fn handle_redirect(req: HttpRequest) -> Result<impl Responder> {
  let query = req.query_string();
  let code = urlencoding::decode(retrieve_value_from_query_key(query, "code"))
    .expect("Decoding failed")
    .to_string();

  let client_secret = get_client_secret();
  let client_id = get_client_id();
  let client = Client::new();

  let token = client
    .post("https://oauth2.googleapis.com/token")
    .form(&[
      ("code", code.as_str()),
      ("client_id", &client_id),
      ("client_secret", &client_secret),
      ("redirect_uri", REDIRECT_URI),
      ("grant_type", "authorization_code")
    ])
    .send()
    .await?;
  
  if let Ok(token) = token.error_for_status() {
    // Response is always given in json format
    let token_as_struct: TokenResponse = serde_json::from_str(&token.text().await?)?;
    // println!("token: {token_as_struct:?}");
    get_user_emails(token_as_struct).await?
  }
    
  Ok(HttpResponse::Ok().body("Redirect received!"))
}

