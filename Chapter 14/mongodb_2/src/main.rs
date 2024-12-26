use mongodb::{Client, Database, Collection, bson::Document};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let database: Database = client.database("mydb");

    // Specify the type explicitly as Collection<Document>
    let _collection: Collection<Document> = database.collection("mycollection");

    Ok(())
}
