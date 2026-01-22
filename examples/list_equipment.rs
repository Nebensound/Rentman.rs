//! Example: List equipment
//!
//! Usage:
//! ```sh
//! RENTMAN_TOKEN=your-token cargo run --example list_equipment
//! ```

use rentman::{models::common::QueryParams, RentmanClient, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file if it exists (for local development)
    dotenvy::dotenv().ok();
    
    // Get API token from environment variable
    let token = env::var("RENTMAN_TOKEN")
        .expect("RENTMAN_TOKEN environment variable not set");

    // Create client
    let client = RentmanClient::builder()
        .token(token)
        .build()?;

    // List equipment (with limit to avoid response size errors)
    println!("Fetching equipment...");
    let params = QueryParams::default().limit(50);
    let equipment = client.equipment().list_with_params(params).await?;

    println!("\nFound {} equipment items:\n", equipment.len());
    
    for item in equipment.iter().take(10) {
        println!("  ID: {}", item.common.id);
        if let Some(name) = &item.name {
            println!("  Name: {}", name);
        }
        if let Some(price) = item.price {
            println!("  Rental Price: €{:.2}", price);
        }
        if let Some(in_stock) = item.in_stock {
            println!("  In Stock: {}", in_stock);
        }
        if let Some(is_rental) = item.is_rental {
            println!("  Available for Rental: {}", is_rental);
        }
        println!();
    }

    if equipment.len() > 10 {
        println!("... and {} more", equipment.len() - 10);
    }

    Ok(())
}
