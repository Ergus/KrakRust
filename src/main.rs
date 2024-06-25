use reqwest;
use std::collections::HashMap;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    let mut params = HashMap::new();
    params.insert("pair", "BTCEUR");

    let response = reqwest::Client::new()
        .post("https://api.kraken.com/0/public/Depth")
        .query(&params)
        .send()
        .await?;

    println!("Response status {}", response.status());

    // Get the response text
    let json_body = response.json::<serde_json::Value>().await?;

    println!("{}", serde_json::to_string_pretty(&json_body).unwrap());

    Ok(())

}
