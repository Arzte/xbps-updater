extern crate github_rs;
extern crate serde_json;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate serde_derive;

mod github;

use failure::Error;
use github::user::get_user;

fn main() {
    println!(
        "{}",
        get_user("Vaelatern@gmail.com").unwrap_or("Error getting email.".to_owned())
    );
}
