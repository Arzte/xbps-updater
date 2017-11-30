use github::try::TryExecute;
use github_rs::client::Github;
use serde_json::from_value;
use failure::Error;

#[derive(Deserialize)]
struct User {
    items: Vec<Items>,
}
#[derive(Deserialize)]
struct Items {
    login: String,
}

pub fn get_user(email: &str) -> Result<String, Error> {
    let client = Github::new(&::std::env::var("GITHUB_TOKEN").expect("token")).unwrap();
    let search = client
        .get()
        .search()
        .users()
        .q(&format!("{}+in:email", email))
        .try_execute();
    Ok(from_value::<User>(search?)?.items[0]
            .login
            // clone so that I'm not sending a reference (can't be moved out of index, so
            // cloning/references are the only way to do this)
            .clone())
}
