use mongodb::{Client, Database, Collection, bson::{doc, Document}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let database: Database = client.database("mydb");
    let collection: Collection<Document> = database.collection("mycollection");

    let filter = doc! { "name": "Abhishek" };
    let result = collection.find_one(Some(filter), None).await?;

    if let Some(document) = result {
        println!("{:?}", document);
    }

    Ok(())
}
