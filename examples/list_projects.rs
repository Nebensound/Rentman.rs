//! Example: List all projects
//!
//! Usage:
//! ```sh
//! RENTMAN_TOKEN=your-token cargo run --example list_projects
//! ```

use rentman::{RentmanClient, Result};
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

    // List all projects
    println!("Fetching projects...");
    let projects = client.projects().list().await?;

    println!("\nFound {} projects:\n", projects.len());
    
    for project in projects.iter().take(10) {
        println!("  ID: {}", project.common.id);
        if let Some(name) = &project.name {
            println!("  Name: {}", name);
        }
        if let Some(number) = &project.number {
            println!("  Number: {}", number);
        }
        if let Some(customer) = &project.customer {
            println!("  Customer: {}", customer);
        }
        println!();
    }

    if projects.len() > 10 {
        println!("... and {} more", projects.len() - 10);
    }

    Ok(())
}
