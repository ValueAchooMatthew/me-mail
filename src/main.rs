use tokio;
use clap::*;

#[tokio::main]
async fn main() -> Result<(), tokio::io::Error> {

  let email_to_authenticate = Command::new("Me-mail")
    .author(crate_authors!(", "))
    .version(crate_version!())
    .about(crate_description!())
    .args(&[
      arg!(-e --email <EMAIL> "Email to be used for authentication"),
    ])
    .after_help(
      "lmaoing at you so hard rn"
    ).get_matches();
    
  let blank_email_message = String::from("No email provided");
  println!("{}", email_to_authenticate.get_one::<String>("email").unwrap_or(&blank_email_message));

  Ok(())
}
