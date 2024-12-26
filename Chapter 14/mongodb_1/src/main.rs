use mongodb::{Client, options::ClientOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let _client = Client::with_options(client_options)?;

    Ok(())
}
