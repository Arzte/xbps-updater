extern crate github_rs;
extern crate serde_json;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate serde_derive;

mod github;
mod update;

use failure::Error;
use update::check_update;
use github::user::get_user;

fn main() {
     if let Err(err) = run() {
        panic!("xbps-updater encountered an unrecoverable error:\n\n\t{}\n", err);
    }
}

fn run() -> Result<(), Error> {
    // println!("{}",get_user("Vaelatern@gmail.com").unwrap_or("Error getting email.".to_owned()));
    // println!("{}", check_update("chromium-widevine").unwrap_or("Error checking for updates".to_owned()));
    println!("done!");
    Ok(())
}
