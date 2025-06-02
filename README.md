# RinKokonoe - Discord Bot for Price Tracking and AI Product Monitoring

## Overview
RinKokonoe is a comprehensive Discord bot that combines price tracking, AI product monitoring, and validation capabilities. It's designed to help communities track prices, monitor AI product launches, and validate various resources.

## Features
- **Price Tracking**: Monitor product prices across various platforms
- **AI Product Monitoring**: Track new AI product launches and updates
- **Validation System**: Validate API keys, endpoints, and resources
- **Discord Integration**: Full command interface through Discord
- **Cloud Sync**: Automatic report syncing to Google Drive

## Requirements
- Go 1.22+
- SQLite
- Discord Bot Token
- rclone (for Google Drive integration)

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/rinkokonoe.git
   cd rinkokonoe
   ```

2. Copy the example environment file and configure it:
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. Install dependencies:
   ```bash
   go mod download
   ```

4. Build the project:
   ```bash
   go build ./...
   ```

5. Run the bot:
   ```bash
   go run cmd/main.go
   ```

## Configuration
Create a `.env` file with the following settings:
```env
ENVIRONMENT=development
DATABASE_PATH=./data/opsbot.db
DISCORD_TOKEN=your_bot_token
DISCORD_GUILD=your_guild_id
LOG_DIR=./logs
RCLONE_DRIVE_PATH=gdrive:opsbot
```

## Discord Commands
- `!help` - Display available commands
- `!track <url>` - Track a product price
- `!price <id>` - Check current price
- `!validate key <service> <key>` - Validate API key
- `!validate endpoint <url>` - Validate endpoint

## Project Structure
```
rinkokonoe/
├── cmd/
│   └── main.go
├── internal/
│   ├── config/
│   ├── database/
│   ├── discord/
│   └── cloud/
├── modules/
│   ├── infopulse/
│   ├── specter/
│   └── validator/
├── data/
└── logs/
```

## Contributing
1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License
This project is licensed under the MIT License - see the LICENSE file for details.

## Version
Current version: 0.1.0 (Initial Release)
