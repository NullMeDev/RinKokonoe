use anyhow::{Context as AnyhowContext, Result};
use chrono::Utc;
use sqlx::{
    migrate::MigrateDatabase, pool::PoolOptions, sqlite::SqlitePoolOptions, Pool, Sqlite, SqlitePool,
};
use std::path::Path;
use std::time::Duration;
use tracing::{debug, error, info, warn};

use crate::config;
use crate::models::{Config, Coupon};

/// Initialize the database, creating it if it doesn't exist
pub async fn initialize_database(config: &Config) -> Result<Pool<Sqlite>> {
    let database_url = config::get_database_url();
    info!("Initializing database with URL: {}", database_url);

    // Extract the database file path from the URL
    let db_path = database_url
        .strip_prefix("sqlite:")
        .unwrap_or(&database_url)
        .to_string();

    // Create the directory for the database file if it doesn't exist
    if let Some(parent) = Path::new(&db_path).parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .context("Failed to create database directory")?;
            info!("Created database directory: {:?}", parent);
        }
    }

    // Create the database if it doesn't exist
    if !Sqlite::database_exists(&database_url).await.unwrap_or(false) {
        info!("Database does not exist, creating it...");
        Sqlite::create_database(&database_url)
            .await
            .context("Failed to create SQLite database")?;
        info!("Database created successfully");
    }

    // Connect to the database
    let pool = SqlitePoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(30))
        .connect(&database_url)
        .await
        .context("Failed to connect to SQLite database")?;

    // Run migrations if they exist
    apply_migrations(&pool).await?;

    // Create tables if they don't exist
    create_tables(&pool).await?;

    Ok(pool)
}

/// Apply database migrations if available
async fn apply_migrations(pool: &SqlitePool) -> Result<()> {
    let migrations_path = std::env::var("MIGRATIONS_DIR").unwrap_or_else(|_| "migrations".to_string());
    
    if Path::new(&migrations_path).exists() {
        info!("Applying database migrations from {}", migrations_path);
        sqlx::migrate::Migrator::new(Path::new(&migrations_path))
            .await
            .context("Failed to create migrator")?
            .run(pool)
            .await
            .context("Failed to run migrations")?;
        info!("Database migrations applied successfully");
    } else {
        warn!("Migrations directory not found, skipping migrations");
    }
    
    Ok(())
}

/// Create database tables if they don't exist
async fn create_tables(pool: &SqlitePool) -> Result<()> {
    info!("Creating database tables if they don't exist");
    
    // Create coupons table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS coupons (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            discount_percentage REAL,
            code TEXT NOT NULL,
            url TEXT NOT NULL,
            source TEXT NOT NULL,
            expiry TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            validated_at TEXT,
            is_valid INTEGER NOT NULL DEFAULT 0,
            is_posted INTEGER NOT NULL DEFAULT 0,
            hash TEXT NOT NULL UNIQUE
        )
        "#,
    )
    .execute(pool)
    .await
    .context("Failed to create coupons table")?;
    
    info!("Database tables created successfully");
    Ok(())
}

/// Insert a new coupon into the database
pub async fn insert_coupon(pool: &SqlitePool, coupon: &Coupon) -> Result<i64> {
    debug!("Inserting coupon: {:?}", coupon);
    
    let result = sqlx::query!(
        r#"
        INSERT INTO coupons
        (name, description, discount_percentage, code, url, source, expiry, created_at, is_valid, hash)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
        coupon.name,
        coupon.description,
        coupon.discount_percentage,
        coupon.code,
        coupon.url,
        coupon.source,
        coupon.expiry.map(|dt| dt.to_rfc3339()),
        coupon.created_at.unwrap_or_else(Utc::now).to_rfc3339(),
        coupon.is_valid,
        coupon.hash
    )
    .execute(pool)
    .await
    .context("Failed to insert coupon")?;
    
    Ok(result.last_insert_rowid())
}

/// Check if a coupon already exists in the database by its hash
pub async fn coupon_exists(pool: &SqlitePool, hash: &str) -> Result<bool> {
    let result = sqlx::query!("SELECT COUNT(*) as count FROM coupons WHERE hash = ?", hash)
        .fetch_one(pool)
        .await
        .context("Failed to check if coupon exists")?;
    
    Ok(result.count > 0)
}

/// Update a coupon's validation status
pub async fn update_validation_status(
    pool: &SqlitePool,
    coupon_id: i64,
    is_valid: bool,
) -> Result<()> {
    let now = Utc::now().to_rfc3339();
    
    sqlx::query!(
        r#"
        UPDATE coupons
        SET is_valid = ?,
            validated_at = ?
        WHERE id = ?
        "#,
        is_valid,
        now,
        coupon_id
    )
    .execute(pool)
    .await
    .context("Failed to update coupon validation status")?;
    
    Ok(())
}

/// Mark a coupon as posted
pub async fn mark_as_posted(pool: &SqlitePool, coupon_id: i64) -> Result<()> {
    sqlx::query!("UPDATE coupons SET is_posted = 1 WHERE id = ?", coupon_id)
        .execute(pool)
        .await
        .context("Failed to mark coupon as posted")?;
    
    Ok(())
}

/// Get all coupons from the database
pub async fn get_all_coupons(pool: &SqlitePool) -> Result<Vec<Coupon>> {
    let coupons = sqlx::query_as!(
        Coupon,
        r#"
        SELECT 
            id,
            name,
            description,
            discount_percentage,
            code,
            url,
            source,
            expiry as "expiry: Option<DateTime<Utc>>",
            created_at as "created_at: Option<DateTime<Utc>>",
            validated_at as "validated_at: Option<DateTime<Utc>>",
            is_valid,
            is_posted,
            hash
        FROM coupons
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
    .context("Failed to get all coupons")?;
    
    Ok(coupons)
}

/// Get valid coupons that haven't been posted yet
pub async fn get_valid_unposted_coupons(pool: &SqlitePool) -> Result<Vec<Coupon>> {
    let coupons = sqlx::query_as!(
        Coupon,
        r#"
        SELECT 
            id,
            name,
            description,
            discount_percentage,
            code,
            url,
            source,
            expiry as "expiry: Option<DateTime<Utc>>",
            created_at as "created_at: Option<DateTime<Utc>>",
            validated_at as "validated_at: Option<DateTime<Utc>>",
            is_valid,
            is_posted,
            hash
        FROM coupons
        WHERE is_valid = 1 AND is_posted = 0
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
    .context("Failed to get valid unposted coupons")?;
    
    Ok(coupons)
}

/// Get coupon by id
pub async fn get_coupon_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Coupon>> {
    let coupon = sqlx::query_as!(
        Coupon,
        r#"
        SELECT 
            id,
            name,
            description,
            discount_percentage,
            code,
            url,
            source,
            expiry as "expiry: Option<DateTime<Utc>>",
            created_at as "created_at: Option<DateTime<Utc>>",
            validated_at as "validated_at: Option<DateTime<Utc>>",
            is_valid,
            is_posted,
            hash
        FROM coupons
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(pool)
    .await
    .context("Failed to get coupon by id")?;
    
    Ok(coupon)
}

/// Delete expired coupons
pub async fn delete_expired_coupons(pool: &SqlitePool) -> Result<u64> {
    let now = Utc::now().to_rfc3339();
    
    let result = sqlx::query!(
        r#"
        DELETE FROM coupons
        WHERE expiry IS NOT NULL AND expiry < ?
        "#,
        now
    )
    .execute(pool)
    .await
    .context("Failed to delete expired coupons")?;
    
    Ok(result.rows_affected())
}

/// Get coupons by source
pub async fn get_coupons_by_source(pool: &SqlitePool, source: &str) -> Result<Vec<Coupon>> {
    let coupons = sqlx::query_as!(
        Coupon,
        r#"
        SELECT 
            id,
            name,
            description,
            discount_percentage,
            code,
            url,
            source,
            expiry as "expiry: Option<DateTime<Utc>>",
            created_at as "created_at: Option<DateTime<Utc>>",
            validated_at as "validated_at: Option<DateTime<Utc>>",
            is_valid,
            is_posted,
            hash
        FROM coupons
        WHERE source = ?
        ORDER BY created_at DESC
        "#,
        source
    )
    .fetch_all(pool)
    .await
    .context("Failed to get coupons by source")?;
    
    Ok(coupons)
}

