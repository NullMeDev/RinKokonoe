use anyhow::{Context as AnyhowContext, Result};
use async_trait::async_trait;
use chrono::Utc;
use reqwest::{Client, StatusCode};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, error, info, warn};

use crate::models::{Config, Coupon, CouponSource, ValidationResult};

/// Trait for coupon validators
#[async_trait]
pub trait CouponValidator: Send + Sync {
    /// Returns the name of the validator
    fn name(&self) -> &'static str;
    
    /// Checks if this validator can validate coupons from the given source
    fn can_validate(&self, source: &str) -> bool;
    
    /// Validates a coupon
    async fn validate(&self, coupon: &Coupon, client: &Client) -> Result<ValidationResult>;
}

/// Main validator that manages all validation strategies
pub struct Validator {
    validators: Vec<Box<dyn CouponValidator>>,
    config: Arc<Config>,
    client: Client,
}

impl Validator {
    pub fn new(config: Arc<Config>, client: Client) -> Self {
        let mut validators: Vec<Box<dyn CouponValidator>> = Vec::new();
        
        // Add validators for different sources
        validators.push(Box::new(CursorAIValidator::new()));
        validators.push(Box::new(GitHubValidator::new()));
        validators.push(Box::new(ReplitValidator::new()));
        validators.push(Box::new(WarpValidator::new()));
        validators.push(Box::new(TabnineValidator::new()));
        validators.push(Box::new(GenericValidator::new()));
        
        Self {
            validators,
            config,
            client,
        }
    }
    
    /// Validate a coupon
    pub async fn validate_coupon(&self, coupon: &Coupon) -> Result<ValidationResult> {
        // First check if the coupon is expired
        if coupon.is_expired() {
            return Ok(ValidationResult {
                is_valid: false,
                message: Some("Coupon has expired".to_string()),
                validated_at: Utc::now(),
            });
        }
        
        // Find a validator for this coupon's source
        for validator in &self.validators {
            if validator.can_validate(&coupon.source) {
                debug!("Using {} validator for coupon: {}", validator.name(), coupon.name);
                return validator.validate(coupon, &self.client).await;
            }
        }
        
        // If no specific validator is found, use a fallback approach
        warn!("No validator found for source: {}", coupon.source);
        Ok(ValidationResult {
            is_valid: true, // Assume valid if we can't validate
            message: Some(format!("No validator available for source: {}", coupon.source)),
            validated_at: Utc::now(),
        })
    }
}

/// Cursor AI validator
pub struct CursorAIValidator;

impl CursorAIValidator {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CouponValidator for CursorAIValidator {
    fn name(&self) -> &'static str {
        "Cursor AI Validator"
    }
    
    fn can_validate(&self, source: &str) -> bool {
        source == CouponSource::CursorAI.to_string()
    }
    
    async fn validate(&self, coupon: &Coupon, client: &Client) -> Result<ValidationResult> {
        // For student offers, we just verify the student page exists
        if coupon.code == "STUDENT" && coupon.url.contains("/student") {
            let response = client
                .get(&coupon.url)
                .send()
                .await
                .context("Failed to fetch Cursor AI student page")?;
            
            if response.status().is_success() {
                return Ok(ValidationResult {
                    is_valid: true,
                    message: Some("Student program verified as active".to_string()),
                    validated_at: Utc::now(),
                });
            } else {
                return Ok(ValidationResult {
                    is_valid: false,
                    message: Some(format!(
                        "Student program page returned status: {}",
                        response.status()
                    )),
                    validated_at: Utc::now(),
                });
            }
        }
        
        // For promo codes, we'd need to check them against the API
        // This is a simplified example - in a real implementation, we might:
        // 1. Simulate adding a product to cart
        // 2. Apply the coupon code
        // 3. Check if the discount is applied
        
        // For this example, we'll just validate the code format
        if coupon.code.len() >= 4 && coupon.code.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            Ok(ValidationResult {
                is_valid: true,
                message: Some("Coupon code format is valid".to_string()),
                validated_at: Utc::now(),
            })
        } else {
            Ok(ValidationResult {
                is_valid: false,
                message: Some("Invalid coupon code format".to_string()),
                validated_at: Utc::now(),
            })
        }
    }
}

/// GitHub validator
pub struct GitHubValidator;

impl GitHubValidator {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CouponValidator for GitHubValidator {
    fn name(&self) -> &'static str {
        "GitHub Validator"
    }
    
    fn can_validate(&self, source: &str) -> bool {
        source == CouponSource::GitHub.to_string()
    }
    
    async fn validate(&self, coupon: &Coupon, client: &Client) -> Result<ValidationResult> {
        // For GitHub Student Developer Pack, we mainly verify the offer still exists
        let response = client
            .get(&coupon.url)
            .send()
            .await
            .context("Failed to fetch GitHub offer page")?;
        
        if response.status().is_success() {
            // Check if the page contains the offer name
            let html = response.text().await.context("Failed to get response text")?;
            
            if html.contains(&coupon.name) {
                return Ok(ValidationResult {
                    is_valid: true,
                    message: Some("Offer found on GitHub Education page".to_string()),
                    validated_at: Utc::now(),
                });
            } else {
                return Ok(ValidationResult {
                    is_valid: false,
                    message: Some("Offer not found on GitHub Education page".to_string()),
                    validated_at: Utc::now(),
                });
            }
        } else {
            return Ok(ValidationResult {
                is_valid: false,
                message: Some(format!(
                    "GitHub Education page returned status: {}",
                    response.status()
                )),
                validated_at: Utc::now(),
            });
        }
    }
}

/// Replit validator
pub struct ReplitValidator;

impl ReplitValidator {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CouponValidator for ReplitValidator {
    fn name(&self) -> &'static str {
        "Replit Validator"
    }
    
    fn can_validate(&self, source: &str) -> bool {
        source == CouponSource::Replit.to_string()
    }
    
    async fn validate(&self, coupon: &Coupon, client: &Client) -> Result<ValidationResult> {
        // For Replit, verify the education program page exists
        let response = client
            .get(&coupon.url)
            .send()
            .await
            .context("Failed to fetch Replit education page")?;
        
        if response.status().is_success() {
            return Ok(ValidationResult {
                is_valid: true,
                message: Some("Education program verified as active".to_string()),
                validated_at: Utc::now(),
            });
        } else {
            return Ok(ValidationResult {
                is_valid: false,
                message: Some(format!(
                    "Education program page returned status: {}",
                    response.status()
                )),
                validated_at: Utc::now(),
            });
        }
    }
}

/// Warp validator
pub struct WarpValidator;

impl WarpValidator {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CouponValidator for WarpValidator {
    fn name(&self) -> &'static str {
        "Warp Validator"
    }
    
    fn can_validate(&self, source: &str) -> bool {
        source == CouponSource::Warp.to_string()
    }
    
    async fn validate(&self, coupon: &Coupon, client: &Client) -> Result<ValidationResult> {
        // For Warp, verify the student program page exists
        let response = client
            .get(&coupon.url)
            .send()
            .await
            .context("Failed to fetch Warp student page")?;
        
        if response.status().is_success() {
            return Ok(ValidationResult {
                is_valid: true,
                message: Some("Student program verified as active".to_string()),
                validated_at: Utc::now(),
            });
        } else {
            return Ok(ValidationResult {
                is_valid: false,
                message: Some(format!(
                    "Student program page returned status: {}",
                    response.status()
                )),
                validated_at: Utc::now(),
            });
        }
    }
}

/// Tabnine validator
pub struct TabnineValidator;

impl TabnineValidator {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CouponValidator for TabnineValidator {
    fn name(&self) -> &'static str {
        "Tabnine Validator"
    }
    
    fn can_validate(&self, source: &str) -> bool {
        source == CouponSource::Tabnine.to_string()
    }
    
    async fn validate(&self, coupon: &Coupon, client: &Client) -> Result<ValidationResult> {
        // For Tabnine, verify the student program page exists
        let response = client
            .get(&coupon.url)
            .send()
            .await
            .context("Failed to fetch Tabnine student page")?;
        
        if response.status().is_success() {
            return Ok(ValidationResult {
                is_valid: true,
                message: Some("Student program verified as active".to_string()),
                validated_at: Utc::now(),
            });
        } else {
            return Ok(ValidationResult {
                is_valid: false,
                message: Some(format!(
                    "Student program page returned status: {}",
                    response.status()
                )),
                validated_at: Utc::now(),
            });
        }
    }
}

/// Generic validator for other sources
pub struct GenericValidator;

impl GenericValidator {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CouponValidator for GenericValidator {
    fn name(&self) -> &'static str {
        "Generic Validator"
    }
    
    fn can_validate(&self, source: &str) -> bool {
        source == CouponSource::Generic.to_string()
    }
    
    async fn validate(&self, coupon: &Coupon, client: &Client) -> Result<ValidationResult> {
        // For generic coupons, we:
        // 1. Verify the source page is still accessible
        // 2. Check if the coupon code is still mentioned on the page
        
        let response = client
            .get(&coupon.url)
            .send()
            .await
            .context("Failed to fetch coupon source page")?;
        
        if !response.status().is_success() {
            return Ok(ValidationResult {
                is_valid: false,
                message: Some(format!(
                    "Source page returned status: {}",
                    response.status()
                )),
                validated_at: Utc::now(),
            });
        }
        
        let html = response.text().await.context("Failed to get response text")?;
        
        // Check if the coupon code is still mentioned on the page
        if html.contains(&coupon.code) {
            return Ok(ValidationResult {
                is_valid: true,
                message: Some("Coupon code found on source page".to_string()),
                validated_at: Utc::now(),
            });
        } else {
            return Ok(ValidationResult {
                is_valid: false,
                message: Some("Coupon code not found on source page".to_string()),
                validated_at: Utc::now(),
            });
        }
    }
}

/// Initialize the validator
pub fn initialize_validator(config: &Config) -> Result<Validator> {
    info!("Initializing coupon validator");
    
    // Create HTTP client for validation
    let client = Client::builder()
        .timeout(Duration::from_secs(config.validation.timeout))
        .user_agent(&config.scraping.user_agent)
        .build()
        .context("Failed to build HTTP client for validator")?;
    
    Ok(Validator::new(Arc::new(config.clone()), client))
}

