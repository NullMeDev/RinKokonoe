#!/bin/bash
# RinKokonoe Project Setup Script
# This script initializes the RinKokonoe coupon bot project by:
# - Creating necessary directories
# - Setting up environment variables
# - Building the project

# Exit on any error
set -e

# Print colorful messages
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== RinKokonoe Coupon Bot Setup ===${NC}"
echo "This script will set up the RinKokonoe coupon bot project."
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}Rust is not installed. Please install Rust first:${NC}"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo -e "${GREEN}[+]${NC} Rust is installed."

# Create necessary directories
echo "Creating necessary directories..."
mkdir -p data rss
echo -e "${GREEN}[+]${NC} Created data and rss directories."

# Copy .env.example to .env if it doesn't exist
if [ ! -f .env ]; then
    if [ -f .env.example ]; then
        cp .env.example .env
        echo -e "${GREEN}[+]${NC} Copied .env.example to .env."
        echo -e "${YELLOW}NOTE:${NC} Please edit the .env file to configure your Discord token and other settings."
    else
        cp .env .env.example
        echo -e "${GREEN}[+]${NC} Created .env.example from your current .env."
    fi
else
    echo -e "${GREEN}[+]${NC} .env file already exists."
fi

# Build the project
echo "Building the project with cargo..."
cargo build --release
echo -e "${GREEN}[+]${NC} Project built successfully."

# Final instructions
echo ""
echo -e "${BLUE}=== Setup Complete ===${NC}"
echo ""
echo "Next steps:"
echo "1. Edit the .env file to set your Discord token or webhook URL"
echo "   nano .env"
echo ""
echo "2. Run the bot with:"
echo "   cargo run --release"
echo ""
echo "3. Or use Docker Compose:"
echo "   docker-compose up -d"
echo ""
echo -e "${YELLOW}For more information, please refer to the README.md file.${NC}"
echo ""

