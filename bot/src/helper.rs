pub struct Data {} // User data, which is stored and accessible in all command invocations
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
