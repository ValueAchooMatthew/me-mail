use std::env;

use clap::{arg, crate_authors, crate_description, crate_version, Command};

pub fn retrieve_value_from_query_key<'a>(query_string: &'a str, key: &'a str) -> &'a str {

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

pub fn get_email_to_authenticate() -> Option<String> {
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

pub fn get_client_id() -> String {
  dotenv::dotenv().ok().unwrap();
  env::var("CLIENT_ID").unwrap()
}

pub fn get_client_secret() -> String {
  dotenv::dotenv().ok().unwrap();
  env::var("CLIENT_SECRET").unwrap()
}
