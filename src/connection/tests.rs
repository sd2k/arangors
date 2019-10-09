use log::info;
use pretty_assertions::assert_eq;
use reqwest::Url;

use super::*;

const URL: &str = "http://localhost:8529";
const USERNAME: &str = "root";
const PASSWORD: &str = "KWNngteTps7XjrNv";

#[test]
fn test_setup() {
    env_logger::init();
}

#[tokio::test]
async fn test_jwt_auth() {
    // let _ = pretty_env_logger::try_init();
    let conn = Connection {
        arango_url: Url::parse(URL).unwrap(),
        username: String::from("root"),
        session: Arc::new(Client::new()),
        state: Normal,
        phantom: (),
    };
    let jwt = conn.jwt_login(USERNAME, PASSWORD).await.unwrap();
    info!("JWT login success. Token: {}", jwt);
    let not_empty = jwt.len() > 1;
    assert_eq!(not_empty, true);
}
