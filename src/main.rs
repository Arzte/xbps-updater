extern crate github_rs;
extern crate serde_json;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate serde_derive;

use github_rs::StatusCode;
use github_rs::client::{Executor, Github};
use serde_json::Value;
use failure::Error;

trait TryExecute: Executor {
    fn try_execute(self) -> Result<Value, Error>
    where
        Self: Sized,
    {
        #[derive(Deserialize)]
        struct GitError {
            message: String,
        }

        // TODO: Replace format_err!() macro calls with proper non-string custom error handling
        match self.execute::<Value>() {
            Ok((_, StatusCode::Ok, Some(response))) => Ok(response),
            Ok((_, _, Some(response))) => {
                serde_json::from_value::<GitError>(response)
                    .map_err(|err| format_err!("Failed to parse error response: {}", err))
                    .and_then(|error| Err(format_err!("{}", error.message)))
            }
            Ok((_, _, None)) => Err(format_err!(
                "Received error response from github with no message"
            )),
            Err(err) => Err(format_err!("Failed to execute request: {}", err)),
        }
    }
}

impl<'a> TryExecute for ::github_rs::search::get::SearchUsersQ<'a> {}

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
            .clone(),
    )
}

fn main() {
    println!(
        "{}",
        get_user("Vaelatern@gmail.com").unwrap_or("Error getting email.".to_owned())
    );
}
