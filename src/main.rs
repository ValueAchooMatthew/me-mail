use std::env;
use clap::{arg, crate_authors, crate_description, crate_version, Command};
use reqwest::Client;
use dotenv;
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};
use anyhow::Error;
use serde::{Deserialize, Serialize};

// For hot reload:
// watchexec -e rs -r cargo run
// If you don't have watchexec command: 
// Either run 
// $: cargo binstall watchexec-cli
// or:
// $: curl -fsSL https://apt.cli.rs/pubkey.asc | sudo tee -a /usr/share/keyrings/rust-tools.asc 
// && curl -fsSL https://apt.cli.rs/rust-tools.list | sudo tee /etc/apt/sources.list.d/rust-tools.list 
// && sudo apt update 
// && sudo apt install watchexec-cli

const REDIRECT_URI: &str = "http://localhost:8080/signin-google";

#[derive(Serialize, Deserialize, Debug)]
struct TokenResponse {
  access_token: String,
  expires_in: u32,
  refresh_token: String,
  scope: String,
  token_type: String,
  id_token: String
}

#[get("/signin-google")]
async fn handle_redirect(req: HttpRequest) -> Result<impl Responder, actix_web::Error> {
  let query = req.query_string();
  let code = urlencoding::decode(retrieve_value_from_query_key(query, "code"))
    .expect("Decoding failed")
    .to_string();

  let client_secret = get_client_secret();
  let client_id = get_client_id();
  let client = Client::new();

  println!("code: {code}, client_id: {client_id}, client_secret: {client_secret}");
  
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
    .await;
  
  if let Ok(token) = token.unwrap().error_for_status() {
    // Response is always given in json format
    let token_as_struct: Result<TokenResponse, serde_json::Error> = serde_json::from_str(&token.text().await.unwrap());
    println!("token: {token_as_struct:?}");
  }
    
  Ok(HttpResponse::Ok().body("Redirect received!"))
}

fn retrieve_value_from_query_key<'a>(query_string: &'a str, key: &'a str) -> &'a str {

  let parameters: Vec<&str> = query_string.split("&").collect();
  for parameter in parameters {

    let split_parameter = parameter.split("=").collect::<Vec<&str>>();
    let parameter_key = split_parameter
      .get(0)
      .expect("Expected key-value pair");
    
    if parameter_key == &key {
      return split_parameter.get(1).expect("Expected a key-value pair");
    } 
  };
  ""
}

fn get_email_to_authenticate() -> Option<String> {
  let email_to_authenticate= Command::new("Me-mail")
    .author(crate_authors!(", "))
    .version(crate_version!())
    .about(crate_description!())
    .args(&[
      arg!(-e --email <EMAIL> "Email to be used for authentication"),
    ])
    .after_help(
      "lmaoing at you so hard rn"
    ).get_matches()
    .get_one("email")
    .cloned();
  
  email_to_authenticate
}

fn get_client_id() -> String {

  dotenv::dotenv().ok().unwrap();
  env::var("CLIENT_ID").unwrap()
}

fn get_client_secret() -> String {
  dotenv::dotenv().ok().unwrap();
  env::var("CLIENT_SECRET").unwrap()
}

#[actix_web::main]
async fn main() -> Result<(), Error> {

  let client_id = get_client_id();

  // Scopes must be space delimited
  let scopes = "email";

  let auth_url = format!(
    "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope={}&access_type=offline&prompt=consent",
    client_id,
    urlencoding::encode(REDIRECT_URI),
    urlencoding::encode(scopes),
  );

  println!("{}", auth_url);

  HttpServer::new(|| {
    App::new()
      .service(handle_redirect)
  })
  .bind(("localhost", 8080))?
  .run()
  .await?;

  Ok(())
}
