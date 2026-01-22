# Rentman.rs

Rust client library for the [Rentman API](https://api.rentman.net). Rentman is a rental management software for the event industry.

## Features

- ✅ **Type-safe API** - Full Rust type definitions for API resources
- ✅ **Async/await** - Built on `tokio` and `reqwest` for async operations
- ✅ **Token Authentication** - JWT-based authentication
- ✅ **Error Handling** - Comprehensive error types with `thiserror`
- ✅ **CRUD Operations** - Support for Create, Read, Update, Delete operations
- ✅ **Pagination** - Built-in support for paginated responses
- 🚧 **Rate Limiting** - Planned
- 🚧 **Webhooks** - Planned

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rentman = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

### Setup

```bash
# Clone the repository
git clone https://github.com/joschawagner/Rentman.rs
cd Rentman.rs

# Run setup script
./setup.sh

# Edit .env and add your token
nano .env  # or vim, code, etc.
```

Or manually:

```bash
# Copy environment template
cp .env.example .env

# Add your token to .env
echo "RENTMAN_TOKEN=your-token" >> .env

# Build
cargo build
```

### Usage

```rust
use rentman::{RentmanClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client with your API token
    let client = RentmanClient::builder()
        .token("your-api-token")
        .build()?;

    // List all projects
    let projects = client.projects().list().await?;

    for project in projects {
        println!("Project: {}", project.name.unwrap_or_default());
    }

    Ok(())
}
```

## Authentication

### Getting your API Token

Get your API token from the Rentman application:

1. Go to **Configuration → Extensions**
2. Generate a new **JWT token**
3. Copy the token

### Local Development

Create a `.env` file in your project root:

```bash
# Copy the example file
cp .env.example .env

# Edit .env and add your token
RENTMAN_TOKEN=your-actual-token-here
```

The examples will automatically load from `.env` file.

### GitHub Actions / CI

Add your token as a secret in GitHub:

1. Go to your repo → **Settings → Secrets and variables → Actions**
2. Click **New repository secret**
3. Name: `RENTMAN_TOKEN`
4. Value: Your API token
5. Click **Add secret**

The CI workflow will automatically use this secret for integration tests.

⚠️ **Security Note**:

- `.env` is in `.gitignore` and will never be committed
- Never commit tokens directly in code
- Rotate tokens regularly

## Examples

### List Projects

```rust
use rentman::{RentmanClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = RentmanClient::builder()
        .token(std::env::var("RENTMAN_TOKEN")?)
        .build()?;

    let projects = client.projects().list().await?;
    println!("Found {} projects", projects.len());

    Ok(())
}
```

### Create a Project

```rust
use rentman::{RentmanClient, models::project::CreateProject, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = RentmanClient::builder()
        .token(std::env::var("RENTMAN_TOKEN")?)
        .build()?;

    let new_project = CreateProject {
        name: "My Event".to_string(),
        reference: Some("EVENT-001".to_string()),
        number: None,
        custom: None,
    };

    let project = client.projects().create(&new_project).await?;
    println!("Created project with ID: {}", project.common.id);

    Ok(())
}
```

### List Contacts with Pagination

```rust
use rentman::{RentmanClient, models::common::QueryParams, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = RentmanClient::builder()
        .token(std::env::var("RENTMAN_TOKEN")?)
        .build()?;

    let params = QueryParams::new()
        .limit(50)
        .offset(0);

    let contacts = client.contacts().list_with_params(params).await?;

    for contact in contacts {
        println!("Contact: {}", contact.name.unwrap_or_default());
    }

    Ok(())
}
```

### Get Equipment Details

```rust
use rentman::{RentmanClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = RentmanClient::builder()
        .token(std::env::var("RENTMAN_TOKEN")?)
        .build()?;

    let equipment = client.equipment().get(123).await?;

    println!("Equipment: {}", equipment.name.unwrap_or_default());
    println!("In stock: {}", equipment.in_stock.unwrap_or(0));

    Ok(())
}
```

## Running Examples

The examples automatically load your token from `.env`:

```bash
# 1. Copy and configure .env
cp .env.example .env
# Edit .env and add your RENTMAN_TOKEN

# 2. Run any example
cargo run --example list_projects
cargo run --example create_project
cargo run --example list_contacts
cargo run --example list_equipment
```

Or use environment variable directly:

```bash
RENTMAN_TOKEN=your-token cargo run --example list_projects
```

- `list_projects` - List all projects
- `create_project` - Create a new project
- `list_contacts` - List contacts with pagination
- `list_equipment` - List equipment items

## API Coverage

Currently implemented endpoints:

### Core Resources

- ✅ Projects (list, get, create, update, delete)
- ✅ Subprojects (list)
- ✅ Contacts (list, get)
- ✅ Contact Persons (list)
- ✅ Equipment (list, get)
- ✅ Serial Numbers (list, get)

### Planned

- 🚧 Crew Members
- 🚧 Appointments
- 🚧 Quotations
- 🚧 Invoices
- 🚧 Stock Movements
- 🚧 And many more...

See the [Rentman API Documentation](https://api.rentman.net) for a complete list of available endpoints.

## Testing

Run unit tests:

```sh
cargo test
```

Run integration tests with real API (requires `.env` with `RENTMAN_TOKEN`):

```sh
cp .env.example .env
# Add your token to .env
cargo test -- --ignored
```

## Project Structure

```
rentman/
├── src/
│   ├── lib.rs           # Library entry point
│   ├── client.rs        # HTTP client implementation
│   ├── error.rs         # Error types
│   ├── models/          # API data models
│   │   ├── common.rs    # Common types (pagination, etc.)
│   │   ├── project.rs   # Project models
│   │   ├── contact.rs   # Contact models
│   │   ├── equipment.rs # Equipment models
│   │   └── ...
│   ├── endpoints/       # API endpoint implementations
│   │   ├── projects.rs  # Projects endpoint
│   │   ├── contacts.rs  # Contacts endpoint
│   │   ├── equipment.rs # Equipment endpoint
│   │   └── ...
│   └── prelude.rs       # Convenience re-exports
├── examples/            # Usage examples
├── tests/               # Integration tests
└── Cargo.toml
```

## Requirements

- **Rust**: 1.85.0 or later (Edition 2024)
- **Tokio**: Async runtime
- **Dependencies**: See [Cargo.toml](Cargo.toml)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Resources

- [Rentman API Documentation](https://api.rentman.net)
- [Rentman Website](https://rentman.io)
- [Repository](https://github.com/joschawagner/Rentman.rs)

## Disclaimer

This is an unofficial client library and is not affiliated with or endorsed by Rentman.

Trade to use Rentman API
