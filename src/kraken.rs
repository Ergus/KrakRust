use reqwest;
use std::collections::HashMap;


pub async fn get_info(key: &str, value: &str) -> Result<serde_json::Value, reqwest::Error> {

    let mut params = HashMap::new();
    params.insert(key, value);

    let res = reqwest::Client::new()
        .post("https://api.kraken.com/0/public/Depth")
        .query(&params)
        .send()
        .await?;

        //println!("{}", serde_json::to_string_pretty(&json_body).unwrap());

    Ok(res.json::<serde_json::Value>().await?)

}