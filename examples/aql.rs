use arangors::{AqlQuery, Connection};
use serde_json::value::Value;
const URL: &str = "http://localhost:8529";

#[tokio::main]
async fn main() {
    env_logger::init();

    let conn = Connection::establish_jwt(URL, "root", "KWNngteTps7XjrNv")
        .await
        .unwrap();

    let database = conn.db("test_db").await.unwrap();
    let aql = AqlQuery::new("FOR u IN test_collection LIMIT 3 RETURN u");
    println!("{:?}", aql);
    println!("{:?}", serde_json::to_string(&aql).unwrap());

    let resp: Vec<Value> = database.aql_query(aql).await.unwrap();
    println!("{:?}", resp);
}
