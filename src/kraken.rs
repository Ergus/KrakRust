use reqwest;
use std::collections::HashMap;


pub async fn get_info(key: &str, value: &str) -> Result<reqwest::Response, reqwest::Error> {

    let mut params = HashMap::new();
    params.insert(key, value);

    let res = reqwest::Client::new()
        .post("https://api.kraken.com/0/public/Depth")
        .query(&params)
        .send()
        .await?;

    Ok(res)
}

#[cfg(test)]
mod test;