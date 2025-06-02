# RinKokonoe - AI Product & Price Tracking Discord Bot

![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

A comprehensive Discord bot that combines price tracking, AI product monitoring, and validation capabilities. It helps communities track prices, monitor AI product launches, and validate various resources.

## ✨ Features

- **Price Tracking**: Monitor product prices across various platforms
  - Automated price checking
  - Price drop alerts
  - Historical price tracking
  - Custom price thresholds
- **AI Product Monitoring**: Track new AI product launches and updates
  - Early-stage AI products
  - Student discounts
  - AI IDE tools
  - Certification programs
- **Validation System**: Verify resources and credentials
  - API key validation
  - Endpoint health checks
  - Resource availability monitoring
- **Discord Integration**: Full command interface
  - Rich command system
  - Real-time notifications
  - Interactive responses
- **Cloud Integration**: Automatic report syncing to Google Drive

## 🛠️ System Requirements

- Go 1.22+
- SQLite
- Discord Bot Token
- rclone (for Google Drive integration)

## 📋 Setup Instructions

1. Clone the repository:
   ```bash
   git clone https://github.com/NullMeDev/RinKokonoe.git
   cd RinKokonoe
   ```

2. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

3. Configure the environment variables in `.env`:
   - `DISCORD_TOKEN`: Your Discord bot token
   - `DISCORD_GUILD`: Your Discord server ID
   - Other optional settings

4. Install dependencies:
   ```bash
   go mod download
   ```

5. Build and run:
   ```bash
   go build ./...
   go run cmd/main.go
   ```

## 🤖 Discord Commands

- `!help` - Display available commands
- `!track <url>` - Track a product price
- `!price <id>` - Check current price
- `!validate key <service> <key>` - Validate API key
- `!validate endpoint <url>` - Validate endpoint

## 📁 Project Structure

```
RinKokonoe/
├── cmd/
│   └── main.go          # Application entry point
├── internal/
│   ├── config/          # Configuration handling
│   ├── database/        # Database operations
│   ├── discord/         # Discord bot integration
│   └── cloud/           # Cloud sync utilities
├── modules/
│   ├── infopulse/       # AI product scraping
│   ├── specter/         # Price tracking
│   └── validator/       # Validation system
├── data/                # Local data storage
└── logs/                # Application logs
```

## 🔄 Versioning

RinKokonoe follows [Semantic Versioning](https://semver.org/):
- `MAJOR` version for incompatible API changes
- `MINOR` version for new functionality in a backward-compatible manner
- `PATCH` version for backward-compatible bug fixes

## 📝 Changelog

### v0.1.0 (June 2, 2025) - Initial Release
- Basic project structure
- Core functionality implementation
- Module system setup
- Documentation and configuration
- Discord integration
- Database setup
- Cloud sync capabilities

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

Created by NullMeDev
