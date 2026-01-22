#!/usr/bin/env bash
# Setup script for local development

set -e

echo "🚀 Setting up Rentman.rs..."
echo ""

# Check if .env exists
if [ -f .env ]; then
    echo "✅ .env file already exists"
else
    echo "📝 Creating .env file from template..."
    cp .env.example .env
    echo "✅ Created .env file"
    echo ""
    echo "⚠️  Please edit .env and add your RENTMAN_TOKEN"
    echo "   Get your token from: Rentman → Configuration → Extensions"
fi

echo ""
echo "📦 Installing dependencies..."
cargo build

echo ""
echo "✅ Setup complete!"
echo ""
echo "Next steps:"
echo "  1. Edit .env and add your RENTMAN_TOKEN"
echo "  2. Run an example: cargo run --example list_projects"
echo "  3. Run tests: cargo test"
echo ""
