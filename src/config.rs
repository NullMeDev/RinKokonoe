use anyhow::{Context as AnyhowContext, Result};
use config::{Config as ConfigCrate, ConfigBuilder, Environment, File};
use std::env;
use std::path::Path;
use std::sync::Arc;
use tracing::{debug, info, warn};

use crate::models::{
    ApiConfig, Config, DiscordConfig, ProxyConfig, RssConfig, ScrapingConfig, ValidationConfig,
};

/// Load configuration from files and environment variables
pub fn load_config() -> Result<Arc<Config>> {
    // Determine the configuration file path
    let config_path = env::var("CONFIG_FILE").unwrap_or_else(|_| "config.toml".to_string());
    info!("Loading configuration from {}", config_path);

    // Build configuration with defaults, file, and environment variables
    let config_builder = ConfigCrate::builder()
        .set_default("discord.command_prefix", "!")?
        .set_default("discord.status_message", "Scraping coupons")?
        .set_default("scraping.default_interval", 60)?
        .set_default("scraping.max_concurrent", 10)?
        .set_default("scraping.user_agent", "RinKokonoe Coupon Bot/1.0")?
        .set_default("rss.items_per_feed", 30)?
        .set_default("rss.refresh_interval", 60)?
        .set_default("api.enable", true)?
        .set_default("api.port", 8080)?
        .set_default("api.rate_limit", 60)?
        .set_default("proxy.enable", false)?
        .set_default("proxy.proxies", "")?
        .set_default("proxy.rotate_after", 100)?
        .set_default("validation.enable", true)?
        .set_default("validation.timeout", 30)?;

    // Load config file if it exists
    let config_builder = if Path::new(&config_path).exists() {
        config_builder.add_source(File::with_name(&config_path))
    } else {
        warn!("Configuration file {} not found, using defaults", config_path);
        config_builder
    };

    // Add environment variables with prefix RIN_ (e.g., RIN_DISCORD_TOKEN)
    let config_builder = config_builder.add_source(
        Environment::with_prefix("RIN")
            .separator("_")
            .try_parsing(true),
    );

    // Build the configuration
    let config = config_builder.build()?;

    // Convert to our Config struct
    let discord_config = DiscordConfig {
        command_prefix: config.get_string("discord.command_prefix")?,
        status_message: config.get_string("discord.status_message")?,
        webhook_url: config
            .get_string("discord.webhook_url")
            .ok(),
        channel_id: config
            .get_string("discord.channel_id")
            .ok(),
    };

    let scraping_config = ScrapingConfig {
        default_interval: config.get_int("scraping.default_interval")? as u64,
        max_concurrent: config.get_int("scraping.max_concurrent")? as u64,
        user_agent: config.get_string("scraping.user_agent")?,
    };

    let rss_config = RssConfig {
        items_per_feed: config.get_int("rss.items_per_feed")? as u64,
        refresh_interval: config.get_int("rss.refresh_interval")? as u64,
    };

    let api_config = ApiConfig {
        enable: config.get_bool("api.enable")?,
        port: config.get_int("api.port")? as u16,
        rate_limit: config.get_int("api.rate_limit")? as u64,
    };

    let proxy_config = ProxyConfig {
        enable: config.get_bool("proxy.enable")?,
        proxies: config.get_string("proxy.proxies")?,
        rotate_after: config.get_int("proxy.rotate_after")? as u64,
    };

    let validation_config = ValidationConfig {
        enable: config.get_bool("validation.enable")?,
        timeout: config.get_int("validation.timeout")? as u64,
    };

    let app_config = Config {
        discord: discord_config,
        scraping: scraping_config,
        rss: rss_config,
        api: api_config,
        proxy: proxy_config,
        validation: validation_config,
    };

    // Validate configuration
    validate_config(&app_config)?;

    debug!("Configuration loaded: {:?}", app_config);
    Ok(Arc::new(app_config))
}

/// Validate the configuration to ensure required values are present and valid
fn validate_config(config: &Config) -> Result<()> {
    // Validate Discord token from environment
    if env::var("DISCORD_TOKEN").is_err() && 
       env::var("RIN_DISCORD_TOKEN").is_err() && 
       config.discord.webhook_url.is_none() {
        return Err(anyhow::anyhow!(
            "DISCORD_TOKEN environment variable or discord.webhook_url must be set"
        ));
    }

    // Validate scraping interval
    if config.scraping.default_interval < 1 {
        return Err(anyhow::anyhow!(
            "scraping.default_interval must be at least 1 minute"
        ));
    }

    // Validate API port if API is enabled
    if config.api.enable && (config.api.port < 1024 || config.api.port > 65535) {
        return Err(anyhow::anyhow!("api.port must be between 1024 and 65535"));
    }

    Ok(())
}

/// Get the Discord token from environment variables
pub fn get_discord_token() -> Result<String> {
    // Try RIN_DISCORD_TOKEN first, then fall back to DISCORD_TOKEN
    env::var("RIN_DISCORD_TOKEN")
        .or_else(|_| env::var("DISCORD_TOKEN"))
        .context("DISCORD_TOKEN environment variable must be set")
}

/// Get the database URL from environment variables or use default
pub fn get_database_url() -> String {
    env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:data/rin_kokonoe.db".to_string())
}

/// Get the RSS output directory from environment variables or use default
pub fn get_rss_output_dir() -> String {
    env::var("RSS_OUTPUT_DIR").unwrap_or_else(|_| "rss".to_string())
}

/// Get the base URL for the API
pub fn get_base_url() -> String {
    env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string())
}

