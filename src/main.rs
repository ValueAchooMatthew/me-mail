mod utils;
mod structs;
mod endpoints;
mod types;
mod user_functions;

use actix_web::{App, HttpServer};
use types::Result;
use utils::get_client_id;
use endpoints::auth_flow_endpoint::handle_redirect;

use utils::open_url_on_user_brower;

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

#[actix_web::main]
async fn main() -> Result<()> {

  let client_id = get_client_id();

  // Scopes must be space delimited
  let scopes = "email https://www.googleapis.com/auth/gmail.send https://www.googleapis.com/auth/gmail.readonly https://www.googleapis.com/auth/gmail.compose https://www.googleapis.com/auth/gmail.insert";

  let auth_url = format!(
    "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope={}&access_type=offline&prompt=consent",
    client_id,
    urlencoding::encode(REDIRECT_URI),
    urlencoding::encode(scopes),
  );

  open_url_on_user_brower(&auth_url)?;

  HttpServer::new(|| {
    App::new()
      .service(handle_redirect)
  })
  .bind(("localhost", 8080))?
  .run()
  .await?;

  Ok(())
}

