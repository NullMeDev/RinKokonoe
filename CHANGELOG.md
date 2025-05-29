# Changelog

All notable changes to the RinKokonoe project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Future features will be listed here

### Changed
- Future changes will be listed here

### Deprecated
- Future deprecations will be listed here

### Removed
- Future removals will be listed here

### Fixed
- Future fixes will be listed here

### Security
- Future security fixes will be listed here

## [1.0.0] - 2025-05-29

### Added
- Initial project structure and module organization
- Configuration management with environment variables and config.toml support
- SQLite database integration with migration support
- Web scraping functionality for multiple coupon sources:
  - Cursor AI IDE scraper
  - GitHub Student Developer Pack scraper
  - Replit education offers scraper
  - Warp Terminal student offers scraper
  - Tabnine offers scraper
  - Generic AI tools scraper
- Coupon validation engine to verify codes before posting
- Discord integration using both bot token and webhook approaches
- Rich embed formatting for coupon notifications
- Task scheduler for periodic scraping and database cleanup
- Setup scripts for Windows and Unix-based systems
- Comprehensive documentation with setup and usage instructions
- Docker and Docker Compose support for easy deployment
- Logging infrastructure with configurable verbosity levels
- Error handling throughout the application

### Changed
- N/A (Initial release)

### Deprecated
- N/A (Initial release)

### Removed
- N/A (Initial release)

### Fixed
- N/A (Initial release)

### Security
- Input validation for all web requests
- Safe handling of environment variables
- Proper error handling to prevent information leakage

[Unreleased]: https://github.com/yourusername/RinKokonoe/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/yourusername/RinKokonoe/releases/tag/v1.0.0

