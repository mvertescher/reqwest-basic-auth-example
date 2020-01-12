//! Simple example to show basic authentication with tokio/reqwests

use serde_derive::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let user = "user";
    let pass = "pass";
    let url = format!("https://httpbin.org/basic-auth/{}/{}", user, pass);

    let response = reqwest::Client::new()
        .get(&url)
        .basic_auth(user, Some(pass))
        .send()
        .await?
        .json::<Response>()
        .await?;

    println!("{:#?}", response);

    Ok(())
}

#[derive(Debug, Deserialize)]
struct Response {
    authenticated: bool,
    user: String,
}
