extern crate github_rs;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use github_rs::StatusCode;
use github_rs::client::{Executor, Github};
use serde_json::Value;

trait TryExecute: Executor {
    fn try_execute(self) -> Result<Value, String>
    where
        Self: Sized,
    {
        #[derive(Deserialize)]
        struct GithubError {
            message: String,
        }

        match self.execute::<Value>() {
            Ok((_, StatusCode::Ok, Some(response))) => Ok(response),
            Ok((_, _, Some(response))) => {
                serde_json::from_value::<GithubError>(response)
                    .map_err(|err| format!("Failed to parse error response: {}", err))
                    .and_then(|error| Err(error.message.into()))
            }
            Ok((_, _, None)) => Err("Received error response from github with no message".into()),
            Err(err) => Err(format!("Failed to execute request: {}", err)),
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

fn get_user(email: String) -> String {
    let client = Github::new("0f4724b053421cd81401d2827ffe14751f60e5d1").unwrap();
    let search = client
        .get()
        .search()
        .users()
        .q(&format!("{}+in:email", email))
        .try_execute();
    serde_json::from_value::<User>(search?)?.items[0].login
}

fn main() {
    println!("{}", get_user("Vaelatern@gmail.com".to_owned()));
}
