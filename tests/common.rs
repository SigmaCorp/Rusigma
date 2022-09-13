use anyhow::Result;
use rusigma::SigmaClient;

use dotenv::dotenv;
use std::env;

struct Credentials {
    username: String,
    password: String,
}

fn read_credentials() -> Result<Credentials> {
    dotenv().ok();
    Ok(Credentials {
        username: env::var("SIGMA_USERNAME")?,
        password: env::var("SIGMA_PASSWORD")?,
    })
}

pub async fn make_client() -> Result<SigmaClient> {
    let credentials = read_credentials()?;
    let mut sclient = SigmaClient::new();
    sclient
        .login_with_credentials(credentials.username, credentials.password)
        .await?;
    Ok(sclient)
}
