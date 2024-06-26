mod kraken;

#[tokio::main]
async fn main() {

    match kraken::get_info("pair", "BTCEUR").await {
        Ok(data) => {
            println!("{}", serde_json::to_string_pretty(&data).unwrap());
        },
        Err(e) => println!("Error: {}", e),
    }
}
