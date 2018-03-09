#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate hubcaps;
extern crate hyper;
extern crate hyper_tls;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;

use failure::Error;
use github::create_issue;
use hubcaps::{Credentials, Github};
use hubcaps::issues::IssueOptions;
use tokio_core::reactor::Core;
use update::*;

mod github;
mod update;

pub fn new_core() -> Core {
    Core::new().expect("reactor fail")
}

fn main() {
    if let Err(err) = run() {
        panic!(
            "xbps-updater encountered an unrecoverable error:\n\n\t{}\n",
            err
        );
    }
}

fn run() -> Result<(), Error> {
    let issue = create_issue(
        ("Coding-Doctors", "void-packages"),
        "test",
        Some("tester"),
    );

    // Creates an issue, printing any results
    println!("{:?}", new_core().run(issue));

    // Checks for a package update
    println!("{:?}", update_package("chromium-widevine"));

    // if nothing errors out this will properly exit the program
    // If something does error, it won't reach this line and run will use the panic in main() to
    // exit the program with the xbps-updater encountered an ...
    Ok(())
}
