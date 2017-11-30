extern crate github_rs;
extern crate serde_json;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate serde_derive;

mod github;

use github_rs::client::Github;
use failure::Error;
use github::try::TryExecute;

#[derive(Deserialize)]
struct User {
    items: Vec<Items>,
}
#[derive(Deserialize)]
struct Items {
    login: String,
}

fn get_user(email: &str) -> Result<String, Error> {
    let client = Github::new(&::std::env::var("GITHUB_TOKEN").expect("token")).unwrap();
    let search = client
        .get()
        .search()
        .users()
        .q(&format!("{}+in:email", email))
        .try_execute();
    Ok(
        serde_json::from_value::<User>(search?)?.items[0]
            .login
            // clone so that I'm not sending a reference (can't be moved out of index, so
            // cloning/references are the only way to do this)
            .clone(),
    )
}

fn main() {
    println!(
        "{}",
        get_user("Vaelatern@gmail.com").unwrap_or("Error getting email.".to_owned())
    );
}
