package config

import (
    "os"
    "path/filepath"

    "github.com/joho/godotenv"
)

// Config holds all configuration values
type Config struct {
    Environment      string
    DatabasePath    string
    LogDirectory    string
    DiscordToken    string
    DiscordGuild    string
    OpenRouterKey   string
    ScraperUserAgent string
    ProxyList       []string
    RcloneDrivePath string
    ValidationFrequency string
}

// Load reads configuration from environment variables or .env file
func Load(path string) (*Config, error) {
    // Load .env file if it exists
    if _, err := os.Stat(path); err == nil {
        if err := godotenv.Load(path); err != nil {
            return nil, err
        }
    }

    cfg := &Config{
        Environment:         getEnv("ENVIRONMENT", "development"),
        DatabasePath:        getEnv("DATABASE_PATH", "./data/opsbot.db"),
        LogDirectory:        getEnv("LOG_DIR", "./logs"),
        DiscordToken:        getEnv("DISCORD_TOKEN", ""),
        DiscordGuild:        getEnv("DISCORD_GUILD", ""),
        OpenRouterKey:       getEnv("OPENROUTER_KEY", ""),
        ScraperUserAgent:    getEnv("SCRAPER_USER_AGENT", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"),
        RcloneDrivePath:     getEnv("RCLONE_DRIVE_PATH", "gdrive:opsbot"),
        ValidationFrequency: getEnv("VALIDATION_FREQUENCY", "0 */6 * * *"),
    }

    // Create required directories
    os.MkdirAll(cfg.LogDirectory, 0755)
    os.MkdirAll(filepath.Dir(cfg.DatabasePath), 0755)

    // Parse proxy list if provided
    if proxyList := getEnv("PROXY_LIST", ""); proxyList != "" {
        cfg.ProxyList = filepath.SplitList(proxyList)
    }

    return cfg, nil
}

// getEnv gets environment variable with fallback
func getEnv(key, fallback string) string {
    if value, exists := os.LookupEnv(key); exists {
        return value
    }
    return fallback
}
