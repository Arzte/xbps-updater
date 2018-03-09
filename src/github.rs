use failure::Error;
use hubcaps::{Credentials, Future, Github};
use hubcaps::issues::{Issue, IssueOptions};
use hyper::client::Connect;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use super::new_core;
use tokio_core::reactor::Core;

fn github() -> Github<HttpsConnector<HttpConnector>> {
    Github::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Some(Credentials::Token(
            ::std::env::var("GITHUB_TOKEN").expect("token"),
        )),
        &new_core().handle(),
    )
}

pub fn create_issue(repo: (&str, &str), title: &str, body: Option<&str>) -> Future<Issue> {
    let (owner, repos) = repo;
    github()
        .repo(owner, repos)
        .issues()
        .create(&IssueOptions::new::<_, &str, &str, _>(
            title,
            body,
            None,
            None,
            vec!["bug"],
        ))
}
