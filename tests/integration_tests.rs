//! Integration tests for the Rentman API client.

#[cfg(test)]
mod tests {
    use rentman::{models::common::QueryParams, RentmanClient, Result};

    #[tokio::test]
    async fn test_client_builder_requires_token() {
        let result = RentmanClient::builder().build();
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert!(e.to_string().contains("token"));
        }
    }

    #[tokio::test]
    async fn test_client_builder_with_token() {
        let result = RentmanClient::builder()
            .token("test-token")
            .build();
        
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_client_builder_with_custom_base_url() {
        let result = RentmanClient::builder()
            .token("test-token")
            .base_url("https://custom.api.com")
            .build();
        
        assert!(result.is_ok());
    }

    // NOTE: These tests require a valid API token and should only run
    // when RENTMAN_TOKEN is set in the environment
    
    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_list_projects() -> Result<()> {
        dotenvy::dotenv().ok();
        let token = std::env::var("RENTMAN_TOKEN")
            .expect("RENTMAN_TOKEN not set");
        
        let client = RentmanClient::builder()
            .token(token)
            .build()?;
        
        let projects = client.projects().list().await?;
        println!("Found {} projects", projects.len());
        
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_list_contacts() -> Result<()> {
        dotenvy::dotenv().ok();
        let token = std::env::var("RENTMAN_TOKEN")
            .expect("RENTMAN_TOKEN not set");
        
        let client = RentmanClient::builder()
            .token(token)
            .build()?;
        
        let params = QueryParams::default().limit(50);
        let contacts = client.contacts().list_with_params(params).await?;
        println!("Found {} contacts", contacts.len());
        
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn test_list_equipment() -> Result<()> {
        dotenvy::dotenv().ok();
        let token = std::env::var("RENTMAN_TOKEN")
            .expect("RENTMAN_TOKEN not set");
        
        let client = RentmanClient::builder()
            .token(token)
            .build()?;
        
        let params = QueryParams::default().limit(50);
        let equipment = client.equipment().list_with_params(params).await?;
        println!("Found {} equipment items", equipment.len());
        
        Ok(())
    }
}
