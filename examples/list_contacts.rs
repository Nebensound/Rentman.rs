//! Example: List contacts
//!
//! Usage:
//! ```sh
//! RENTMAN_TOKEN=your-token cargo run --example list_contacts
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

    // List contacts with pagination
    let params = QueryParams::new()
        .limit(20)
        .offset(0);

    println!("Fetching contacts...");
    let contacts = client.contacts().list_with_params(params).await?;

    println!("\nFound {} contacts:\n", contacts.len());
    
    for contact in contacts.iter().take(10) {
        println!("  ID: {}", contact.common.id);
        if let Some(name) = &contact.name {
            println!("  Name: {}", name);
        }
        if let Some(email) = &contact.email_1 {
            println!("  Email: {}", email);
        }
        if let Some(phone) = &contact.phone_1 {
            println!("  Phone: {}", phone);
        }
        println!();
    }

    if contacts.len() > 10 {
        println!("... and {} more", contacts.len() - 10);
    }

    Ok(())
}
