#[cfg(test)]
use mongodb::{Client, options::ClientOptions};
use rstest::rstest;

#[rstest]
#[tokio::test]
#[ignore]
async fn mongodb_should_connect() {
    let username = std::env::var("MONGO_INITDB_ROOT_USERNAME").expect("MONGO_INITDB_ROOT_USERNAME not set");
    let password = std::env::var("MONGO_INITDB_ROOT_PASSWORD").expect("MONGO_INITDB_ROOT_PASSWORD not set");
    let connection_string = format!("mongodb://{}:{}@localhost:27017", username, password);
    let client_options = ClientOptions::parse(connection_string).await.expect("Failed to create ClientOptions.");
    let client = Client::with_options(client_options).expect("Failed to create Client.");
    println!("Pinging MongoDB...");
    let db_names = client.list_database_names().await.expect("Failed to list databases.");
    println!("Successfully connected! Found {} databases.", db_names.len());
}
