use database::service::DbService;
use poise::serenity_prelude::{self as serenity, CreateEmbed};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
pub struct Data {
    // User data, which is stored and accessible in all command invocations
    pub cooldowns: Arc<Mutex<HashMap<u64, Instant>>>,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;

use std::env;
//check if user has set role
pub async fn is_admin(ctx: ApplicationContext<'_>) -> Result<bool, Error> {
    let required_role_id: u64 = env::var("REQUIRED_ROLE_ID")
        .unwrap_or_else(|_| "1372996752900000000".to_string())
        .parse()
        .unwrap_or(1372996752900000000); // Fetch from environment variable

    if let Some(guild_id) = ctx.guild_id() {
        let member = ctx
            .serenity_context()
            .http
            .get_member(guild_id, ctx.author().id)
            .await?;
        if let Some(guild) = ctx.guild() {
            if member.roles.iter().any(|role_id| {
                guild
                    .roles
                    .get(role_id)
                    .map(|r| r.id.get() == required_role_id)
                    .unwrap_or(false)
            }) {
                return Ok(true);
            }
        }
    }
    // If the user does not have the required role, return false
    Ok(false)
}

//check if command is sent in age gated channel
pub async fn is_age_gated(ctx: ApplicationContext<'_>) -> Result<bool, Error> {
    let channel_id = ctx.channel_id().to_string();
    let channel = ctx
        .serenity_context()
        .http
        .get_channel(channel_id.parse::<u64>().unwrap().into())
        .await
        .map_err(|e| format!("Error fetching channel: {}", e))?;
    Ok(channel.guild().map(|c| c.nsfw).unwrap_or(false)) // fail to sfw to prevent nsfw in safe channels
}
pub fn paginate<T>(vec: &[T], page: usize, per_page: usize) -> &[T] {
    let start = page.saturating_sub(1) * per_page;
    let end = start + per_page;
    &vec[start.min(vec.len())..end.min(vec.len())]
}

pub async fn check_and_update_cooldown(ctx: &ApplicationContext<'_>) -> Result<bool, Error> {
    let is_admin = crate::helper::is_admin(*ctx).await?;
    if is_admin {
        return Ok(false); // No cooldown for admins
    }
    // get cooldown from env
    let cooldown_secs: u64 = env::var("COOLDOWN_SECONDS")
        .unwrap_or_else(|_| "10".to_string())
        .parse()
        .unwrap_or(10); // Default to 10 seconds if not set
    let data = ctx.data();
    let user_id = ctx.author().id.get();
    let mut cooldowns = data.cooldowns.lock().await;
    let now = Instant::now();
    if let Some(&last_used) = cooldowns.get(&user_id) {
        if now.duration_since(last_used) < Duration::from_secs(cooldown_secs) {
            return Ok(true); // Cooldown active
        }
    }
    cooldowns.insert(user_id, now);
    Ok(false)
}
//error function
pub async fn error_happened(e: Error, ctx: Option<ApplicationContext<'_>>) -> Result<(), Error> {
    println!("some kind of error happened: {}", e);
    //save error to database
    let mut db = DbService::new();
    let error_message = e.to_string();
    let error_code = format!("{:?}", e);

    db.new_error_log(error_message, error_code, None)?;
    match ctx {
        None => {}
        Some(ctx) => {
            ctx.send(
                poise::CreateReply::default()
                    .ephemeral(true)
                    .embed(create_embed(
                        "Some kind of error happened",
                        "Ask a server admin to look at the logs",
                        "",
                        serenity::Colour::RED,
                    )),
            )
            .await?;
        }
    };
    Ok(())
}
//creates an embed
pub fn create_embed(
    title: &str,
    description: &str,
    footer: &str,
    color: serenity::Color,
) -> CreateEmbed {
    serenity::CreateEmbed::new()
        .title(title)
        .description(description)
        .footer(serenity::CreateEmbedFooter::new(footer))
        .color(color)
}
//discord user id string to username
pub async fn get_username_from_id(id: &str, ctx: Option<ApplicationContext<'_>>) -> String {
    match ctx {
        None => id.to_string(),
        Some(ctx) => {
            let user = ctx
                .serenity_context()
                .http
                .get_user(serenity::UserId::new(id.parse::<u64>().unwrap()))
                .await;
            match user {
                Ok(user) => user.name,
                Err(_) => id.to_string(),
            }
        }
    }
}
