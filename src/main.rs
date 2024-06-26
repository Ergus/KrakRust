mod kraken;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {

    match kraken::get_info("pair", "BTCEUR").await {
        Ok(data) => {
            let json_data = data.json::<serde_json::Value>().await?;

            println!("{}", serde_json::to_string_pretty(&json_data).unwrap());
        },
        Err(e) => println!("Error: {}", e),
    };

    Ok(())
}
