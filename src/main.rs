use std::env;
use clap::{arg, crate_authors, crate_description, crate_version, Command};
use reqwest::Client;
use dotenv;
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};

// For hot reload:
// watchexec -e rs -r cargo run

#[get("/signin-google")]
async fn handle_redirect(req: HttpRequest) -> impl Responder {
    println!("Hit the redirect endpoint! Request: {:?}", req);
    HttpResponse::Ok().body("Redirect received!")
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

fn get_client_id_and_client_secret() -> (String, String) {

  dotenv::dotenv().ok().unwrap();
  (env::var("CLIENT_ID").unwrap(), env::var("CLIENT_SECRET").unwrap())
}

#[actix_web::main]
// #[tokio::main]
async fn main() -> Result<(), std::io::Error> {

  let (client_id, _client_secret) = get_client_id_and_client_secret();
  let client = Client::new();

  let redirect_uri = "http://localhost:8080/signin-google";
  // Scopes must be space delimited
  let scopes = "openid profile email";

  // let url = ""
  let auth_url = format!(
    "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope={}&access_type=offline&prompt=consent",
    client_id,
    urlencoding::encode(redirect_uri),
    urlencoding::encode(scopes),
  );

  println!("{:?}", auth_url);

  HttpServer::new(|| {
    App::new()
      .service(handle_redirect)
  })
  .bind(("localhost", 8080))?
  .run()
  .await?;


  // let root_url = "https://oauth2.googleapis.com/token";

  // let response = client
  //   .post(root_url)
  //   // .form(&params)
  //   // .query(query)
  //   .send()
  //   .await?;
  
  // println!("{:?}", response);

  Ok(())
}