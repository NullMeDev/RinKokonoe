use anyhow::{Context as AnyhowContext, Result};
use chrono::Utc;
use serenity::{
    async_trait,
    builder::{CreateEmbed, CreateMessage},
    http::Http,
    model::{
        channel::Message,
        gateway::Ready,
        id::ChannelId,
        webhook::Webhook,
    },
    prelude::*,
};
use std::env;
use std::sync::Arc;
use tracing::{debug, error, info, warn};

use crate::config;
use crate::models::{Config, Coupon};

/// Discord client wrapper that supports both bot token and webhook
pub struct DiscordClient {
    token_client: Option<Arc<Http>>,
    webhook_url: Option<String>,
    channel_id: Option<ChannelId>,
    config: Arc<Config>,
}

impl DiscordClient {
    /// Create a new Discord client
    pub fn new(token_client: Option<Arc<Http>>, webhook_url: Option<String>, channel_id: Option<String>, config: Arc<Config>) -> Self {
        let channel_id = channel_id.and_then(|id| id.parse::<u64>().ok()).map(ChannelId);
        
        Self {
            token_client,
            webhook_url,
            channel_id,
            config,
        }
    }
    
    /// Send a coupon notification to Discord
    pub async fn send_coupon_notification(&self, coupon: &Coupon) -> Result<()> {
        info!("Sending coupon notification to Discord: {}", coupon.name);
        
        let embed = self.create_coupon_embed(coupon);
        
        // Try webhook first if available
        if let Some(webhook_url) = &self.webhook_url {
            debug!("Using webhook to send notification");
            return self.send_webhook_message(webhook_url, &coupon.name, embed).await;
        }
        
        // Fall back to bot token if available
        if let Some(client) = &self.token_client {
            if let Some(channel_id) = self.channel_id {
                debug!("Using bot token to send notification to channel {}", channel_id);
                return self.send_channel_message(client, channel_id, &coupon.name, embed).await;
            } else {
                return Err(anyhow::anyhow!("Channel ID not set for bot token client"));
            }
        }
        
        // Neither method available
        Err(anyhow::anyhow!("No Discord notification method available (neither webhook nor bot token)"))
    }
    
    /// Create a rich embed for a coupon
    fn create_coupon_embed(&self, coupon: &Coupon) -> CreateEmbed {
        let mut embed = CreateEmbed::default();
        
        // Set the title and URL
        embed.title(format!("✅ {} AI Coupon", coupon.name));
        embed.url(&coupon.url);
        
        // Set the description
        embed.description(&coupon.description);
        
        // Add fields for discount, code, etc.
        if let Some(discount) = coupon.discount_percentage {
            embed.field("Discount", format!("{}%", discount), true);
        }
        
        embed.field("Code", &coupon.code, true);
        embed.field("Source", &coupon.source, true);
        
        // Add expiry if available
        if let Some(expiry) = coupon.expiry {
            let now = Utc::now();
            let days_left = (expiry - now).num_days();
            
            if days_left > 0 {
                embed.field("Expires", format!("In {} days", days_left), true);
            } else {
                embed.field("Expires", "Today", true);
            }
        }
        
        // Set the color and timestamp
        embed.color(0x00_c8_ff); // Light blue color
        embed.timestamp(Utc::now());
        
        // Set footer
        embed.footer(|f| {
            f.text("RinKokonoe Coupon Bot")
        });
        
        embed
    }
    
    /// Send a message via webhook
    async fn send_webhook_message(&self, webhook_url: &str, content: &str, embed: CreateEmbed) -> Result<()> {
        let http = Http::new("");
        let webhook = Webhook::from_url(&http, webhook_url).await?;
        
        webhook.execute(&http, false, |w| {
            w.content(content)
                .username("RinKokonoe Coupon Bot")
                .embeds(vec![embed])
        }).await?;
        
        Ok(())
    }
    
    /// Send a message to a channel using bot token
    async fn send_channel_message(&self, http: &Http, channel_id: ChannelId, content: &str, embed: CreateEmbed) -> Result<()> {
        channel_id.send_message(http, |m| {
            m.content(content)
                .embed(|e| {
                    e.0 = embed.0;
                    e
                })
        }).await?;
        
        Ok(())
    }
}

/// Handler for Discord events
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected to Discord as {}", ready.user.name);
    }
    
    async fn message(&self, ctx: Context, msg: Message) {
        // Handle commands if needed in the future
        if msg.content.starts_with('!') {
            debug!("Received command: {}", msg.content);
            // Command handling can be added here
        }
    }
}

/// Initialize Discord client
pub async fn initialize_discord(config: &Config) -> Result<DiscordClient> {
    info!("Initializing Discord client");
    
    let token_result = config::get_discord_token();
    let webhook_url = config.discord.webhook_url.clone();
    let channel_id = config.discord.channel_id.clone();
    
    let token_client = match token_result {
        Ok(token) => {
            info!("Using Discord bot token for authentication");
            let http = Http::new(&token);
            
            // Validate the token by fetching the current user
            match http.get_current_user().await {
                Ok(user) => {
                    info!("Authenticated as Discord user: {}", user.name);
                    Some(Arc::new(http))
                }
                Err(e) => {
                    warn!("Failed to validate Discord token: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            if webhook_url.is_some() {
                info!("Discord token not found, using webhook URL instead");
                None
            } else {
                return Err(anyhow::anyhow!("No Discord authentication method available: {}", e));
            }
        }
    };
    
    // Create and return the client
    let client = DiscordClient::new(
        token_client,
        webhook_url,
        channel_id,
        Arc::new(config.clone()),
    );
    
    Ok(client)
}

/// Start a full Discord bot (optional for future expansion)
pub async fn start_discord_bot(config: &Config) -> Result<Client> {
    let token = config::get_discord_token()?;
    
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES;
    
    let client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .context("Error creating Discord client")?;
    
    Ok(client)
}

/// Format a coupon notification message
pub fn format_coupon_message(coupon: &Coupon) -> String {
    let mut message = format!("✅ **{}**\n", coupon.name);
    
    if let Some(discount) = coupon.discount_percentage {
        message.push_str(&format!("> **Discount:** {}%\n", discount));
    }
    
    message.push_str(&format!("> **Code:** {}\n", coupon.code));
    message.push_str(&format!("> 🔗 [Apply Here]({})\n", coupon.url));
    
    message
}

