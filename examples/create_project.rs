//! Example: Create a new project
//!
//! Usage:
//! ```sh
//! RENTMAN_TOKEN=your-token cargo run --example create_project
//! ```

use rentman::{models::project::CreateProject, RentmanClient, Result};
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

    // Create a new project
    let new_project = CreateProject {
        name: "Test Event from Rust".to_string(),
        reference: Some("RUST-TEST-001".to_string()),
        number: None,
        custom: None,
    };

    println!("Creating project: {}", new_project.name);
    
    let project = client.projects().create(&new_project).await?;

    println!("\nProject created successfully!");
    println!("  ID: {}", project.common.id);
    if let Some(name) = &project.name {
        println!("  Name: {}", name);
    }
    if let Some(number) = &project.number {
        println!("  Number: {}", number);
    }
    if let Some(reference) = &project.reference {
        println!("  Reference: {}", reference);
    }

    Ok(())
}
