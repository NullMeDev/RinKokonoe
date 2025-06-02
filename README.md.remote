# RinKokonoe

![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

A headless Discord coupon bot that scrapes, validates, and posts coupons for AI IDEs, LLMs, development tools, and software. The bot runs continuously on a VPS and notifies a Discord channel in real-time when new valid coupons are found.

![Coupon Bot Demo](https://i.imgur.com/example.png)

## 🔍 Overview

RinKokonoe is designed to help developers and AI enthusiasts save money by automatically finding and validating discount coupons for various AI and development tools. The bot scrapes multiple sources for coupon codes, validates them to ensure they're still active, and posts valid coupons to a Discord channel.

## ✨ Features

- **Automated Coupon Scraping**: Continuously monitors multiple sources for new coupons
- **Multi-Source Support**: Scrapes from various coupon sources including:
  - Cursor AI IDE
  - GitHub Student Developer Pack
  - Replit
  - Warp Terminal
  - Tabnine
  - Generic AI tool sources
- **Coupon Validation**: Verifies coupon codes are active before posting
- **Discord Integration**: Posts coupons as rich embeds to your Discord server
- **Duplicate Detection**: Prevents posting the same coupon multiple times
- **Expiration Management**: Tracks coupon expiration dates and cleans up expired coupons
- **SQLite Database**: Stores coupon information for tracking and deduplication
- **Docker Support**: Easy deployment with Docker and Docker Compose
- **Configurable**: Customizable scraping interval, sources, and other settings
- **Robust Error Handling**: Continues operation even if individual sources fail

## 🛠️ System Requirements

- **For Docker Deployment (Recommended)**:
  - Docker and Docker Compose
  - 256MB RAM minimum (512MB recommended)
  - 1GB free disk space
  - Internet connection

- **For Local Development/Deployment**:
  - Rust 1.70+ and Cargo
  - SQLite
  - 256MB RAM minimum
  - 1GB free disk space
  - Internet connection

## 📋 Setup Instructions

### Environment Configuration

1. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

2. Edit the `.env` file and configure the following variables:
   - `DISCORD_TOKEN` or `DISCORD_WEBHOOK_URL` (one is required)
   - `DISCORD_CHANNEL_ID` (required if using bot token)
   - Other optional settings

### Docker Deployment (Recommended)

1. Make sure Docker and Docker Compose are installed on your system

2. Build and start the container:
   ```bash
   docker-compose up -d
   ```

3. Check the logs to ensure the bot is running:
   ```bash
   docker-compose logs -f
   ```

### Local Development/Deployment

1. Install Rust and Cargo if not already installed:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/RinKokonoe.git
   cd RinKokonoe
   ```

3. Install dependencies and build the project:
   ```bash
   cargo build --release
   ```

4. Create data directories:
   ```bash
   mkdir -p data rss
   ```

5. Run the bot:
   ```bash
   cargo run --release
   ```

## 🚀 Usage

### Running the Bot

Once configured and started, the bot will automatically:
1. Scrape configured sources for coupons
2. Validate discovered coupons
3. Post valid coupons to your Discord channel
4. Repeat the process at the configured interval

### Configuration Options

You can customize the bot's behavior by editing `config.toml`:

- `discord.command_prefix`: Prefix for bot commands (default: `!`)
- `scraping.default_interval`: Scraping interval in minutes (default: `60`)
- `scraping.max_concurrent`: Maximum concurrent scraping operations (default: `10`)
- `validation.enable`: Enable/disable coupon validation (default: `true`)
- `validation.timeout`: Validation timeout in seconds (default: `30`)

### Adding New Coupon Sources

To add a new coupon source:

1. Create a new scraper in `src/scraper.rs` by implementing the `Scraper` trait
2. Add your new scraper to the `initialize_scrapers` function
3. Create a corresponding validator in `src/validator.rs` if needed
4. Rebuild and restart the bot

Example of a simple scraper implementation:

```rust
pub struct MyScraper;

#[async_trait]
impl Scraper for MyScraper {
    fn name(&self) -> &'static str {
        "My Scraper"
    }
    
    fn source(&self) -> String {
        "MySource".to_string()
    }
    
    async fn scrape(&self, client: &Client) -> Result<Vec<Coupon>> {
        // Scraping implementation here
        // ...
        Ok(coupons)
    }
}
```

## 📁 Project Structure

```
RinKokonoe/
├── .env                 # Environment configuration
├── Cargo.toml           # Rust dependencies and project metadata
├── config.toml          # Application configuration
├── docker-compose.yml   # Docker Compose configuration
├── Dockerfile           # Docker build instructions
├── migrations/          # Database migration files
└── src/                 # Source code
    ├── main.rs          # Application entry point
    ├── config.rs        # Configuration loading
    ├── db.rs            # Database operations
    ├── discord.rs       # Discord integration
    ├── models.rs        # Data structures
    ├── scheduler.rs     # Task scheduling
    ├── scraper.rs       # Coupon scraping
    └── validator.rs     # Coupon validation
```

## 🔧 Troubleshooting

### Common Issues

1. **Bot not connecting to Discord**:
   - Check that your `DISCORD_TOKEN` or `DISCORD_WEBHOOK_URL` is correct
   - Ensure the bot has proper permissions in your Discord server
   - Check the logs for any connection errors

2. **No coupons being found**:
   - Check your internet connection
   - Verify that the scraping sources are accessible
   - Check the logs for any scraping errors
   - Try adjusting the user agent in `config.toml`

3. **Database errors**:
   - Ensure the `data` directory exists and is writable
   - Check disk space
   - Check logs for specific SQL errors

4. **Container not starting**:
   - Check Docker logs: `docker-compose logs`
   - Verify that ports are not already in use
   - Ensure your `.env` file exists and is properly configured

### Logging

The bot uses structured logging with different verbosity levels:
- Set `RUST_LOG=debug` in your `.env` file for more detailed logs
- Set `RUST_LOG=info` for standard operational logs
- Set `RUST_LOG=warn` or `RUST_LOG=error` for less verbose logs

## 📊 Versioning

RinKokonoe follows [Semantic Versioning](https://semver.org/). The version numbers follow the format `MAJOR.MINOR.PATCH`:

- `MAJOR` version increments for incompatible API changes
- `MINOR` version increments for new functionality in a backward-compatible manner
- `PATCH` version increments for backward-compatible bug fixes

## 📝 Changelog

### v1.0.0 (May 29, 2025) - Initial Release

#### Added
- Automated scraping of AI tools, LLMs, and development coupons
- Multi-source support for Cursor AI, GitHub, Replit, Warp, Tabnine
- Coupon validation engine to verify coupon codes before posting
- Discord integration for posting coupon notifications
- SQLite database for storing coupon information
- Scheduled tasks for periodic scraping and cleanup
- Docker support for easy deployment
- Comprehensive documentation and setup scripts

For a detailed list of all changes between versions, please see the [CHANGELOG.md](CHANGELOG.md) file.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgements

- Thanks to all the coupon sources that make these deals available
- Built with Rust and various awesome crates from the Rust ecosystem

---

Created by NullMeDevNow
