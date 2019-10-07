use log::trace;
use pretty_assertions::assert_eq;

use arangors::Connection;
use common::{test_setup, NORMAL_PASSWORD, NORMAL_USERNAME, ROOT_PASSWORD, ROOT_USERNAME};

pub mod common;

const URL: &str = "http://localhost:8529/";
const NEW_DB_NAME: &str = "example";

#[tokio::test]
async fn test_create_and_drop_database() {
    test_setup();
    let conn = Connection::establish_jwt(URL, ROOT_USERNAME, ROOT_PASSWORD)
        .await
        .unwrap()
        .into_admin()
        .await
        .unwrap();

    let result = conn.create_database(NEW_DB_NAME).await;
    if let Err(e) = result {
        assert!(false, "Fail to create database: {:?}", e)
    };
    let result = conn.db(NEW_DB_NAME).await;
    assert_eq!(result.is_err(), false);

    let mut conn = conn;
    let result = conn.drop_database(NEW_DB_NAME).await;
    if let Err(e) = result {
        assert!(false, "Fail to drop database: {:?}", e)
    };
    let result = conn.db(NEW_DB_NAME).await;
    assert_eq!(result.is_err(), true);
}

#[tokio::test]
async fn test_fetch_current_database_info() {
    test_setup();
    async fn fetch_current_database(user: &str, passwd: &str) {
        let conn = Connection::establish_jwt(URL, user, passwd).await.unwrap();
        let db = conn.db("test_db").await.unwrap();
        let info = db.info().await;
        let _ = match info {
            Ok(info) => {
                trace!("{:?}", info);
                assert_eq!(info.is_system, false)
            }
            Err(e) => assert!(false, "Fail to drop database: {:?}", e),
        };
    }
    test_root_and_normal!(fetch_current_database);
}

#[tokio::test]
async fn test_get_version() {
    test_setup();
    let conn = Connection::establish_jwt(URL, NORMAL_USERNAME, NORMAL_PASSWORD)
        .await
        .unwrap();
    let db = conn.db("test_db").await.unwrap();
    let version = db.arango_version().await.unwrap();
    trace!("{:?}", version);
    assert_eq!(version.license, "community");
    assert_eq!(version.server, "arango");

    let re = regex::Regex::new(r"3\.\d\.\d+").unwrap();
    assert_eq!(re.is_match(&version.version), true);
}
