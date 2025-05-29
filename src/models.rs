use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

/// Application state shared between components
#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db_pool: Pool<Sqlite>,
    pub last_scrape: Option<DateTime<Utc>>,
}

/// Configuration structure matching config.toml
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub discord: DiscordConfig,
    pub scraping: ScrapingConfig,
    pub rss: RssConfig,
    pub api: ApiConfig,
    pub proxy: ProxyConfig,
    pub validation: ValidationConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DiscordConfig {
    pub command_prefix: String,
    pub status_message: String,
    #[serde(default)]
    pub webhook_url: Option<String>,
    #[serde(default)]
    pub channel_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ScrapingConfig {
    pub default_interval: u64,
    pub max_concurrent: u64,
    pub user_agent: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RssConfig {
    pub items_per_feed: u64,
    pub refresh_interval: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiConfig {
    pub enable: bool,
    pub port: u16,
    pub rate_limit: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProxyConfig {
    pub enable: bool,
    pub proxies: String,
    pub rotate_after: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ValidationConfig {
    pub enable: bool,
    pub timeout: u64,
}

/// Represents a coupon with all metadata
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Coupon {
    #[sqlx(default)]
    pub id: Option<i64>,
    pub name: String,
    pub description: String,
    pub discount_percentage: Option<f64>,
    pub code: String,
    pub url: String,
    pub source: String,
    pub expiry: Option<DateTime<Utc>>,
    #[sqlx(default)]
    pub created_at: Option<DateTime<Utc>>,
    #[sqlx(default)]
    pub validated_at: Option<DateTime<Utc>>,
    #[sqlx(default)]
    pub is_valid: bool,
    #[sqlx(default)]
    pub is_posted: bool,
    #[sqlx(default)]
    pub hash: String,
}

impl Coupon {
    /// Create a new coupon
    pub fn new(
        name: String,
        description: String,
        discount_percentage: Option<f64>,
        code: String,
        url: String,
        source: String,
        expiry: Option<DateTime<Utc>>,
    ) -> Self {
        let now = Utc::now();
        let hash = Self::generate_hash(&name, &code, &url);
        
        Self {
            id: None,
            name,
            description,
            discount_percentage,
            code,
            url,
            source,
            expiry,
            created_at: Some(now),
            validated_at: None,
            is_valid: false,
            is_posted: false,
            hash,
        }
    }
    
    /// Generate a unique hash for the coupon to help with deduplication
    fn generate_hash(name: &str, code: &str, url: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        code.hash(&mut hasher);
        url.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    
    /// Check if the coupon is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expiry) = self.expiry {
            return expiry < Utc::now();
        }
        false
    }
}

/// Enum representing different coupon sources/types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CouponSource {
    CursorAI,
    GitHub,
    Replit,
    Warp,
    Tabnine,
    Generic,
}

impl ToString for CouponSource {
    fn to_string(&self) -> String {
        match self {
            CouponSource::CursorAI => "Cursor AI".to_string(),
            CouponSource::GitHub => "GitHub".to_string(),
            CouponSource::Replit => "Replit".to_string(),
            CouponSource::Warp => "Warp".to_string(),
            CouponSource::Tabnine => "Tabnine".to_string(),
            CouponSource::Generic => "Generic".to_string(),
        }
    }
}

/// Validation result for a coupon
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub message: Option<String>,
    pub validated_at: DateTime<Utc>,
}

