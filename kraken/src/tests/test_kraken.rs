use super::*;

#[tokio::test]
async fn pair_request()
{
    let request = get_info("pair", "BTCEUR").await.expect("request");
    assert_eq!(request.status(), reqwest::StatusCode::OK);
}
