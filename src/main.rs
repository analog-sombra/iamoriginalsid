use ::serenity::all::CreateEmbed;
use poise::serenity_prelude as serenity;
mod config;
use chrono::{DateTime, TimeZone, Utc};
use std::time::UNIX_EPOCH;
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    if u.name == "bugzorc" {
        ctx.say("windows user ew").await?;
    }
    if u.name == "asahi_87" {
        ctx.say("scam(uses arch btw)").await?; //I will change this to user id's later i swear
    }
    let time_str = u.created_at().to_string();
    let dt = DateTime::parse_from_rfc3339(&time_str).expect("Failed to parse datetime");
    let epoch_time = dt.timestamp();
    let discord_timestamp = format!("<t:{}:R>", epoch_time);
    let response = format!("{}'s account was created at {}", u.name, discord_timestamp);
    ctx.say(response).await?;
    Ok(())
}
#[poise::command(slash_command, prefix_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let response = format!("pong!");
    ctx.say(response).await?;
    Ok(())
}
#[poise::command(slash_command, prefix_command)]
async fn help(ctx: Context<'_>) -> Result<(), Error> {
    let embed = CreateEmbed::default().description("Helping");

    Ok(())
}
#[tokio::main]
async fn main() {
    let config = (*config::config::CONFIG).clone();
    let token = config.token;
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), ping(), help()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
