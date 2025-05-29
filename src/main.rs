use anyhow::{Context, Result};
use dotenv::dotenv;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info};

// These will be our modules
mod config;
mod db;
mod discord;
mod models;
mod scheduler;
mod scraper;
mod validator;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize .env file
    dotenv().ok();

    // Setup logging
    tracing_subscriber::fmt::init();
    info!("Starting RinKokonoe coupon bot...");

    // Load configuration
    let config = config::load_config()
        .context("Failed to load configuration")?;
    info!("Configuration loaded successfully");

    // Initialize database connection
    let db_pool = db::initialize_database(&config)
        .await
        .context("Failed to initialize database")?;
    info!("Database connection established");

    // Initialize coupon scrapers
    let scrapers = scraper::initialize_scrapers(&config)
        .context("Failed to initialize scrapers")?;
    info!("Scrapers initialized successfully");

    // Initialize coupon validator
    let validator = validator::initialize_validator(&config)
        .context("Failed to initialize validator")?;
    info!("Validator initialized successfully");

    // Initialize Discord client
    let discord_client = discord::initialize_discord(&config)
        .await
        .context("Failed to initialize Discord client")?;
    info!("Discord client initialized successfully");

    // Initialize shared state
    let state = Arc::new(Mutex::new(models::AppState {
        config: config.clone(),
        db_pool: db_pool.clone(),
        last_scrape: None,
    }));

    // Start the scheduler for periodic scraping
    let scheduler_handle = scheduler::start_scheduler(
        state.clone(),
        scrapers,
        validator,
        discord_client.clone(),
        &config,
    )
    .await
    .context("Failed to start scheduler")?;
    info!("Scheduler started successfully");

    // Keep the main thread alive
    tokio::signal::ctrl_c().await?;
    info!("Shutdown signal received, cleaning up...");

    // Cleanup
    scheduler_handle.abort();
    info!("RinKokonoe bot shutting down");

    Ok(())
}

