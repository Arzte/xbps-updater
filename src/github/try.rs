use github_rs::StatusCode;
use github_rs::client::Executor;
use serde_json::{Value, from_value};
use failure::Error;
use github_rs::*;

pub trait TryExecute: Executor {
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
                from_value::<GitError>(response)
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

// macro here because rust doesn't allow easy multiple type impl's
macro_rules! impl_try_exucute(
    ($($ty:ty),+) => (
            $(
                impl<'a> TryExecute for $ty {}
            )+
        );
);

// lifetime needs to be passed here, and not as part of macro
impl_try_exucute!(search::get::SearchUsersQ<'a>, users::get::User<'a>);
