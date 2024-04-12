mod ccc_client;
use ccc_client::*;

// CODE DU MAIN ICI

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Exemple
    get_version().await?;
    
    Ok(())
}