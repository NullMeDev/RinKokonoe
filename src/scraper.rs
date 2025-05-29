use anyhow::{Context as AnyhowContext, Result};
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use std::sync::Arc;
use std::time::Duration as StdDuration;
use tracing::{debug, error, info, warn};

use crate::models::{Config, Coupon, CouponSource};

/// Trait defining the interface for all scrapers
#[async_trait]
pub trait Scraper: Send + Sync {
    /// Returns the name of the scraper
    fn name(&self) -> &'static str;
    
    /// Returns the source of the scraper
    fn source(&self) -> String;
    
    /// Scrapes coupons from the source
    async fn scrape(&self, client: &Client) -> Result<Vec<Coupon>>;
}

/// Cursor AI scraper
pub struct CursorAIScraper;

#[async_trait]
impl Scraper for CursorAIScraper {
    fn name(&self) -> &'static str {
        "Cursor AI"
    }
    
    fn source(&self) -> String {
        CouponSource::CursorAI.to_string()
    }
    
    async fn scrape(&self, client: &Client) -> Result<Vec<Coupon>> {
        info!("Scraping coupons from Cursor AI");
        let mut coupons = Vec::new();
        
        // First check the student page
        let student_url = "https://cursor.sh/student";
        let response = client
            .get(student_url)
            .send()
            .await
            .context("Failed to fetch Cursor AI student page")?;
        
        if !response.status().is_success() {
            warn!("Failed to fetch Cursor AI student page: HTTP {}", response.status());
            return Ok(coupons);
        }
        
        let html = response.text().await.context("Failed to get response text")?;
        let document = Html::parse_document(&html);
        
        // Try to find student discount information
        if let Some(student_coupon) = extract_cursor_student_coupon(&document, student_url) {
            coupons.push(student_coupon);
        }
        
        // Also check the pricing page for other promotions
        let pricing_url = "https://cursor.sh/pricing";
        let response = client
            .get(pricing_url)
            .send()
            .await
            .context("Failed to fetch Cursor AI pricing page")?;
        
        if !response.status().is_success() {
            warn!("Failed to fetch Cursor AI pricing page: HTTP {}", response.status());
            return Ok(coupons);
        }
        
        let html = response.text().await.context("Failed to get response text")?;
        let document = Html::parse_document(&html);
        
        // Try to find promotion codes
        if let Some(promo_coupons) = extract_cursor_promo_coupons(&document, pricing_url) {
            coupons.extend(promo_coupons);
        }
        
        info!("Found {} coupons from Cursor AI", coupons.len());
        Ok(coupons)
    }
}

/// Helper function to extract student coupon from Cursor AI
fn extract_cursor_student_coupon(document: &Html, url: &str) -> Option<Coupon> {
    // This is a simplified example - in reality, we'd use more complex selectors
    // to find the student discount information
    let selector = Selector::parse("div.student-discount").ok()?;
    
    if let Some(element) = document.select(&selector).next() {
        // Example: found student discount information
        Some(Coupon::new(
            "Cursor AI Student Plan".to_string(),
            "Free Pro features for verified students".to_string(),
            Some(100.0), // 100% discount
            "STUDENT".to_string(),
            url.to_string(),
            CouponSource::CursorAI.to_string(),
            Some(Utc::now() + Duration::days(365)), // Assume 1 year validity
        ))
    } else {
        None
    }
}

/// Helper function to extract promotion coupons from Cursor AI
fn extract_cursor_promo_coupons(document: &Html, url: &str) -> Option<Vec<Coupon>> {
    // Again, this is simplified - real implementation would parse the page
    let selector = Selector::parse("div.promotion-code").ok()?;
    let mut coupons = Vec::new();
    
    for element in document.select(&selector) {
        // Extract code, discount, etc.
        let code = element.value().attr("data-code").unwrap_or("PROMO");
        let discount = element.value().attr("data-discount").unwrap_or("10");
        let discount_pct = discount.parse::<f64>().unwrap_or(10.0);
        
        coupons.push(Coupon::new(
            format!("Cursor AI Promotion: {}% Off", discount),
            "Limited time promotion for Cursor AI Pro".to_string(),
            Some(discount_pct),
            code.to_string(),
            url.to_string(),
            CouponSource::CursorAI.to_string(),
            Some(Utc::now() + Duration::days(30)), // Assume 30 days validity
        ));
    }
    
    if coupons.is_empty() {
        None
    } else {
        Some(coupons)
    }
}

/// GitHub education/student scraper
pub struct GitHubScraper;

#[async_trait]
impl Scraper for GitHubScraper {
    fn name(&self) -> &'static str {
        "GitHub"
    }
    
    fn source(&self) -> String {
        CouponSource::GitHub.to_string()
    }
    
    async fn scrape(&self, client: &Client) -> Result<Vec<Coupon>> {
        info!("Scraping coupons from GitHub Education");
        let mut coupons = Vec::new();
        
        let url = "https://education.github.com/pack";
        let response = client
            .get(url)
            .send()
            .await
            .context("Failed to fetch GitHub Education page")?;
        
        if !response.status().is_success() {
            warn!("Failed to fetch GitHub Education page: HTTP {}", response.status());
            return Ok(coupons);
        }
        
        let html = response.text().await.context("Failed to get response text")?;
        let document = Html::parse_document(&html);
        
        // Extract GitHub Student Developer Pack offers
        let offers_selector = Selector::parse("div.d-flex.flex-wrap.gutter").ok();
        if let Some(selector) = offers_selector {
            for offer_element in document.select(&selector) {
                if let Some(coupon) = extract_github_offer(&offer_element, url) {
                    coupons.push(coupon);
                }
            }
        }
        
        info!("Found {} coupons from GitHub", coupons.len());
        Ok(coupons)
    }
}

/// Helper function to extract offers from GitHub Education
fn extract_github_offer(element: &scraper::ElementRef, url: &str) -> Option<Coupon> {
    // This is a simplified example - real implementation would parse each offer
    let title_selector = Selector::parse("h3").ok()?;
    let desc_selector = Selector::parse("p").ok()?;
    
    let title = element
        .select(&title_selector)
        .next()
        .map(|e| e.text().collect::<String>())
        .unwrap_or_default();
    
    let description = element
        .select(&desc_selector)
        .next()
        .map(|e| e.text().collect::<String>())
        .unwrap_or_default();
    
    if title.is_empty() || !title.to_lowercase().contains("ai") {
        return None; // Not an AI tool offer
    }
    
    // Create a coupon for this offer
    Some(Coupon::new(
        format!("GitHub Student Pack: {}", title),
        description,
        None, // Discount percentage often not explicitly stated
        "GITHUB-STUDENT".to_string(),
        format!("{}#{}", url, title.to_lowercase().replace(' ', "-")),
        CouponSource::GitHub.to_string(),
        None, // Expiry date often not specified
    ))
}

/// Replit scraper
pub struct ReplitScraper;

#[async_trait]
impl Scraper for ReplitScraper {
    fn name(&self) -> &'static str {
        "Replit"
    }
    
    fn source(&self) -> String {
        CouponSource::Replit.to_string()
    }
    
    async fn scrape(&self, client: &Client) -> Result<Vec<Coupon>> {
        info!("Scraping coupons from Replit");
        let mut coupons = Vec::new();
        
        // Check education page
        let edu_url = "https://replit.com/site/teams-for-education";
        let response = client
            .get(edu_url)
            .send()
            .await
            .context("Failed to fetch Replit education page")?;
        
        if !response.status().is_success() {
            warn!("Failed to fetch Replit education page: HTTP {}", response.status());
            return Ok(coupons);
        }
        
        let html = response.text().await.context("Failed to get response text")?;
        let document = Html::parse_document(&html);
        
        // Extract education offers
        // Simplified - real implementation would be more complex
        let selector = Selector::parse("div.education-discount").ok();
        if let Some(sel) = selector {
            if let Some(element) = document.select(&sel).next() {
                coupons.push(Coupon::new(
                    "Replit Teams for Education".to_string(),
                    "Special pricing for educational institutions".to_string(),
                    Some(50.0), // Assumed discount
                    "EDUCATION".to_string(),
                    edu_url.to_string(),
                    CouponSource::Replit.to_string(),
                    None,
                ));
            }
        }
        
        info!("Found {} coupons from Replit", coupons.len());
        Ok(coupons)
    }
}

/// Warp terminal scraper
pub struct WarpScraper;

#[async_trait]
impl Scraper for WarpScraper {
    fn name(&self) -> &'static str {
        "Warp"
    }
    
    fn source(&self) -> String {
        CouponSource::Warp.to_string()
    }
    
    async fn scrape(&self, client: &Client) -> Result<Vec<Coupon>> {
        info!("Scraping coupons from Warp terminal");
        let mut coupons = Vec::new();
        
        // Check student page
        let student_url = "https://www.warp.dev/students";
        let response = client
            .get(student_url)
            .send()
            .await
            .context("Failed to fetch Warp student page")?;
        
        if !response.status().is_success() {
            warn!("Failed to fetch Warp student page: HTTP {}", response.status());
            return Ok(coupons);
        }
        
        // Create a coupon for Warp student program
        coupons.push(Coupon::new(
            "Warp Terminal Student Plan".to_string(),
            "Free Warp Premium subscription for verified students".to_string(),
            Some(100.0), // 100% discount
            "AUTO-APPLIED".to_string(),
            student_url.to_string(),
            CouponSource::Warp.to_string(),
            Some(Utc::now() + Duration::days(365)), // Assume 1 year validity
        ));
        
        info!("Found {} coupons from Warp", coupons.len());
        Ok(coupons)
    }
}

/// Tabnine scraper
pub struct TabnineScraper;

#[async_trait]
impl Scraper for TabnineScraper {
    fn name(&self) -> &'static str {
        "Tabnine"
    }
    
    fn source(&self) -> String {
        CouponSource::Tabnine.to_string()
    }
    
    async fn scrape(&self, client: &Client) -> Result<Vec<Coupon>> {
        info!("Scraping coupons from Tabnine");
        let mut coupons = Vec::new();
        
        // Check student page
        let student_url = "https://www.tabnine.com/students";
        let response = client
            .get(student_url)
            .send()
            .await
            .context("Failed to fetch Tabnine student page")?;
        
        if !response.status().is_success() {
            warn!("Failed to fetch Tabnine student page: HTTP {}", response.status());
            return Ok(coupons);
        }
        
        // Create a coupon for Tabnine student program
        coupons.push(Coupon::new(
            "Tabnine Pro Student Plan".to_string(),
            "Free Tabnine Pro for verified students".to_string(),
            Some(100.0), // 100% discount
            "STUDENT".to_string(),
            student_url.to_string(),
            CouponSource::Tabnine.to_string(),
            Some(Utc::now() + Duration::days(365)), // Assume 1 year validity
        ));
        
        info!("Found {} coupons from Tabnine", coupons.len());
        Ok(coupons)
    }
}

/// Generic AI tools scraper
pub struct GenericAIScraper {
    urls: Vec<String>,
}

impl GenericAIScraper {
    pub fn new(urls: Vec<String>) -> Self {
        Self { urls }
    }
}

#[async_trait]
impl Scraper for GenericAIScraper {
    fn name(&self) -> &'static str {
        "Generic AI Tools"
    }
    
    fn source(&self) -> String {
        CouponSource::Generic.to_string()
    }
    
    async fn scrape(&self, client: &Client) -> Result<Vec<Coupon>> {
        info!("Scraping coupons from generic AI tool sources");
        let mut coupons = Vec::new();
        
        for url in &self.urls {
            info!("Scraping from URL: {}", url);
            
            match client.get(url).send().await {
                Ok(response) => {
                    if !response.status().is_success() {
                        warn!("Failed to fetch {}: HTTP {}", url, response.status());
                        continue;
                    }
                    
                    match response.text().await {
                        Ok(html) => {
                            let document = Html::parse_document(&html);
                            
                            // Look for coupon code patterns
                            if let Some(new_coupons) = extract_generic_coupons(&document, url) {
                                coupons.extend(new_coupons);
                            }
                        }
                        Err(e) => {
                            warn!("Failed to get text from {}: {}", url, e);
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to fetch {}: {}", url, e);
                }
            }
        }
        
        info!("Found {} coupons from generic sources", coupons.len());
        Ok(coupons)
    }
}

/// Helper function to extract coupons from generic pages
fn extract_generic_coupons(document: &Html, url: &str) -> Option<Vec<Coupon>> {
    // Look for common coupon patterns using regex
    let code_regex = Regex::new(r"(?i)code[:\s]+([A-Z0-9-]+)").ok()?;
    let discount_regex = Regex::new(r"(\d+)%\s+(?:off|discount)").ok()?;
    
    let text = document.root_element().text().collect::<String>();
    let mut coupons = Vec::new();
    
    // Find coupon codes
    for code_cap in code_regex.captures_iter(&text) {
        if let Some(code_match) = code_cap.get(1) {
            let code = code_match.as_str().to_string();
            
            // Try to find a discount percentage nearby
            let discount = discount_regex
                .captures(&text)
                .and_then(|cap| cap.get(1))
                .and_then(|m| m.as_str().parse::<f64>().ok())
                .unwrap_or(10.0); // Default to 10% if not specified
            
            // Create a coupon with the found information
            coupons.push(Coupon::new(
                format!("AI Tool Discount: {}% Off", discount),
                format!("Use code {} for {}% off", code, discount),
                Some(discount),
                code,
                url.to_string(),
                CouponSource::Generic.to_string(),
                Some(Utc::now() + Duration::days(30)), // Assume 30 days validity
            ));
        }
    }
    
    if coupons.is_empty() {
        None
    } else {
        Some(coupons)
    }
}

/// Initialize all scrapers based on configuration
pub fn initialize_scrapers(config: &Config) -> Result<Vec<Box<dyn Scraper>>> {
    info!("Initializing scrapers");
    let mut scrapers: Vec<Box<dyn Scraper>> = Vec::new();
    
    // Add built-in scrapers
    scrapers.push(Box::new(CursorAIScraper));
    scrapers.push(Box::new(GitHubScraper));
    scrapers.push(Box::new(ReplitScraper));
    scrapers.push(Box::new(WarpScraper));
    scrapers.push(Box::new(TabnineScraper));
    
    // Add generic scraper with configurable URLs
    // In a real implementation, these URLs would come from config
    let generic_urls = vec![
        "https://aidevtools.com/deals".to_string(),
        "https://llmdeals.net".to_string(),
        "https://devsoftwaredeals.com".to_string(),
    ];
    scrapers.push(Box::new(GenericAIScraper::new(generic_urls)));
    
    info!("Initialized {} scrapers", scrapers.len());
    Ok(scrapers)
}

/// Create an HTTP client for scraping
pub fn create_http_client(config: &Config) -> Result<Client> {
    let user_agent = &config.scraping.user_agent;
    
    let client = Client::builder()
        .timeout(StdDuration::from_secs(30))
        .user_agent(user_agent)
        .build()
        .context("Failed to build HTTP client")?;
    
    Ok(client)
}

