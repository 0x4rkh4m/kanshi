use crate::{Context, Error};
use url::Url;

#[poise::command(prefix_command, hide_in_help, required_permissions = "MANAGE_GUILD")]
pub async fn set_server_url(
    ctx: Context<'_>,
    #[description = "URL to set"] url: String,
) -> Result<(), Error> {
    if Url::parse(&url).is_err() {
        ctx.say("Invalid URL provided").await?;
        return Ok(());
    }

    let data = ctx.data();
    data.fivem_service.lock().await.set_server_url(&url).await;

    ctx.say("Server URL set successfully").await?;
    Ok(())
}

#[poise::command(prefix_command, hide_in_help, required_permissions = "MANAGE_GUILD")]
pub async fn show_server_url(ctx: Context<'_>) -> Result<(), Error> {
    let data = ctx.data();
    let url = data.fivem_service.lock().await.get_server_url().await;

    ctx.say(format!("Current server URL: {}", url)).await?;
    Ok(())
}