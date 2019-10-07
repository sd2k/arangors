use pretty_assertions::assert_eq;

use arangors::connection::Permission;
use arangors::Connection;
use common::{test_setup, NORMAL_PASSWORD, NORMAL_USERNAME};

pub mod common;

const URL: &str = "http://localhost:8529/";

#[tokio::test]
async fn test_list_databases() {
    test_setup();
    let conn = Connection::establish_jwt(URL, NORMAL_USERNAME, NORMAL_PASSWORD)
        .await
        .unwrap();
    let dbs = conn.accessible_databases().await.unwrap();

    assert_eq!(dbs.contains_key("test_db"), true);
    let db_permission = dbs.get("test_db").unwrap();
    match db_permission {
        Permission::ReadOnly | Permission::NoAccess => {
            assert!(false, "Invalid permission {:?}", db_permission)
        }
        _ => {}
    };
}

#[tokio::test]
async fn test_get_url() {
    test_setup();
    let conn = Connection::establish_jwt(URL, NORMAL_USERNAME, NORMAL_PASSWORD)
        .await
        .unwrap();
    let url = conn.get_url().clone().into_string();
    assert_eq!(url, URL)
}

#[tokio::test]
async fn test_get_database() {
    test_setup();
    let conn = Connection::establish_jwt(URL, NORMAL_USERNAME, NORMAL_PASSWORD)
        .await
        .unwrap();
    let database = conn.db("test_db").await;
    assert_eq!(database.is_err(), false);
    let database = conn.db("test_db_non_exist").await;
    assert_eq!(database.is_err(), true);
}

#[tokio::test]
async fn test_basic_auth() {
    test_setup();
    let conn = Connection::establish_jwt(URL, NORMAL_USERNAME, NORMAL_PASSWORD)
        .await
        .unwrap();
    let session = conn.get_session();
    let resp = session.get(URL).send().await.unwrap();
    let headers = resp.headers();
    assert_eq!(headers.get("Server").unwrap(), "ArangoDB");
}

#[tokio::test]
async fn test_jwt() {
    test_setup();
    async fn jwt(user: &str, passwd: &str) {
        let conn = Connection::establish_jwt(URL, user, passwd).await.unwrap();
        let session = conn.get_session();
        let resp = session.get(URL).send().await.unwrap();
        let headers = resp.headers();
        assert_eq!(headers.get("Server").unwrap(), "ArangoDB");
    }
    test_root_and_normal!(jwt);
}
