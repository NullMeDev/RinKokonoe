use anyhow::{Context as AnyhowContext, Result};
use chrono::{DateTime, Duration, Utc};
use sqlx::SqlitePool;
use std::sync::Arc;
use std::time::Duration as StdDuration;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

use crate::db;
use crate::discord::DiscordClient;
use crate::models::{AppState, Config, Coupon};
use crate::scraper::Scraper;
use crate::validator::Validator;

/// Scheduler for periodic tasks
pub struct TaskScheduler {
    state: Arc<Mutex<AppState>>,
    scrapers: Vec<Box<dyn Scraper>>,
    validator: Validator,
    discord_client: DiscordClient,
    config: Arc<Config>,
}

impl TaskScheduler {
    /// Create a new task scheduler
    pub fn new(
        state: Arc<Mutex<AppState>>,
        scrapers: Vec<Box<dyn Scraper>>,
        validator: Validator,
        discord_client: DiscordClient,
        config: Arc<Config>,
    ) -> Self {
        Self {
            state,
            scrapers,
            validator,
            discord_client,
            config,
        }
    }
    
    /// Start the scheduler
    pub async fn start(&self) -> Result<JoinHandle<()>> {
        info!("Starting task scheduler");
        
        // Clone the values needed for the async task
        let state = self.state.clone();
        let scrapers = self.scrapers.clone();
        let validator = self.validator.clone();
        let discord_client = self.discord_client.clone();
        let config = self.config.clone();
        
        // Start the main scheduler loop in a separate task
        let handle = tokio::spawn(async move {
            info!("Task scheduler started");
            
            // Run initial scrape immediately
            if let Err(e) = run_scrape_task(&state, &scrapers, &validator, &discord_client, &config).await {
                error!("Initial scrape failed: {}", e);
            }
            
            // Schedule periodic tasks
            let scrape_interval = StdDuration::from_secs(config.scraping.default_interval * 60);
            let cleanup_interval = StdDuration::from_secs(24 * 60 * 60); // Daily cleanup
            
            let mut last_cleanup = Utc::now();
            
            loop {
                // Wait for the next scrape interval
                sleep(scrape_interval).await;
                
                // Run the scrape task
                if let Err(e) = run_scrape_task(&state, &scrapers, &validator, &discord_client, &config).await {
                    error!("Scheduled scrape failed: {}", e);
                }
                
                // Check if we need to run cleanup (daily)
                let now = Utc::now();
                if (now - last_cleanup).num_seconds() >= (cleanup_interval.as_secs() as i64) {
                    if let Err(e) = run_cleanup_task(&state).await {
                        error!("Cleanup task failed: {}", e);
                    }
                    last_cleanup = now;
                }
            }
        });
        
        Ok(handle)
    }
}

/// Run a scrape task
async fn run_scrape_task(
    state: &Arc<Mutex<AppState>>,
    scrapers: &[Box<dyn Scraper>],
    validator: &Validator,
    discord_client: &DiscordClient,
    config: &Config,
) -> Result<()> {
    info!("Running scrape task");
    
    // Create HTTP client for scraping
    let client = reqwest::Client::builder()
        .timeout(StdDuration::from_secs(30))
        .user_agent(&config.scraping.user_agent)
        .build()
        .context("Failed to build HTTP client")?;
    
    let mut state_guard = state.lock().await;
    let db_pool = state_guard.db_pool.clone();
    state_guard.last_scrape = Some(Utc::now());
    drop(state_guard); // Release the lock
    
    // Scrape coupons from all sources
    let mut all_coupons = Vec::new();
    
    for scraper in scrapers {
        info!("Scraping coupons from {}", scraper.name());
        
        match scraper.scrape(&client).await {
            Ok(coupons) => {
                info!("Found {} coupons from {}", coupons.len(), scraper.name());
                all_coupons.extend(coupons);
            }
            Err(e) => {
                error!("Failed to scrape coupons from {}: {}", scraper.name(), e);
            }
        }
    }
    
    info!("Found {} coupons in total", all_coupons.len());
    
    // Process each coupon
    for coupon in all_coupons {
        process_coupon(&db_pool, &coupon, validator, discord_client).await?;
    }
    
    Ok(())
}

/// Process a single coupon
async fn process_coupon(
    db_pool: &SqlitePool,
    coupon: &Coupon,
    validator: &Validator,
    discord_client: &DiscordClient,
) -> Result<()> {
    // Check if coupon already exists in the database
    if db::coupon_exists(db_pool, &coupon.hash).await? {
        debug!("Coupon already exists: {}", coupon.name);
        return Ok(());
    }
    
    // Insert coupon into database
    let coupon_id = db::insert_coupon(db_pool, coupon).await?;
    debug!("Inserted coupon with ID {}: {}", coupon_id, coupon.name);
    
    // Validate the coupon
    info!("Validating coupon: {}", coupon.name);
    match validator.validate_coupon(coupon).await {
        Ok(validation_result) => {
            // Update validation status in database
            db::update_validation_status(db_pool, coupon_id, validation_result.is_valid).await?;
            
            if validation_result.is_valid {
                info!("Coupon is valid: {}", coupon.name);
                
                // Post validated coupon to Discord
                let mut validated_coupon = coupon.clone();
                validated_coupon.id = Some(coupon_id);
                validated_coupon.is_valid = true;
                validated_coupon.validated_at = Some(validation_result.validated_at);
                
                if let Err(e) = discord_client.send_coupon_notification(&validated_coupon).await {
                    error!("Failed to send coupon notification: {}", e);
                } else {
                    // Mark coupon as posted
                    db::mark_as_posted(db_pool, coupon_id).await?;
                    info!("Coupon posted to Discord: {}", coupon.name);
                }
            } else {
                info!("Coupon is invalid: {}", coupon.name);
                if let Some(message) = validation_result.message {
                    debug!("Validation message: {}", message);
                }
            }
        }
        Err(e) => {
            error!("Failed to validate coupon {}: {}", coupon.name, e);
        }
    }
    
    Ok(())
}

/// Run a cleanup task to remove expired coupons
async fn run_cleanup_task(state: &Arc<Mutex<AppState>>) -> Result<()> {
    info!("Running cleanup task");
    
    let state_guard = state.lock().await;
    let db_pool = state_guard.db_pool.clone();
    drop(state_guard); // Release the lock
    
    // Delete expired coupons
    let deleted_count = db::delete_expired_coupons(&db_pool).await?;
    info!("Deleted {} expired coupons", deleted_count);
    
    Ok(())
}

/// Start the scheduler
pub async fn start_scheduler(
    state: Arc<Mutex<AppState>>,
    scrapers: Vec<Box<dyn Scraper>>,
    validator: Validator,
    discord_client: DiscordClient,
    config: &Config,
) -> Result<JoinHandle<()>> {
    let scheduler = TaskScheduler::new(
        state,
        scrapers,
        validator,
        discord_client,
        Arc::new(config.clone()),
    );
    
    scheduler.start().await
}

