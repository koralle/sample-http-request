use anyhow::{self, Result};
use std::error::Error;

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let api_key = env::var("API_KEY")?;

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q=Tokyo&units=metric&APPID={api_key}",
        api_key = api_key
    );

    let body = reqwest::get(url).await?.text().await?;

    println!("body = {:?}", body);

    Ok(())
}
