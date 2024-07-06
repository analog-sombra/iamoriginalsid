mod config;
use std::process::exit;

use log::{error, info};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
//Initial commit
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                info!("Error sending message: {why:?}");
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();
    // let logger = SimpleLogger::new().init();

    // match logger {
    //     Ok(_) => {
    //         info!("Logger init!")
    //     }
    //     Err(e) => {
    //         println!("{e}");
    //         exit(1);
    //     }
    // }

    let config = (*config::config::CONFIG).clone();

    let token = config.token;
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await;

    match client {
        Ok(mut bot) => match bot.start().await {
            Ok(_) => {
                info!("UwU I am Online");
            }
            Err(why) => {
                error!("Client error: {why:?}");
                exit(1);
            }
        },
        Err(e) => {
            println!("{e}");
            exit(1);
        }
    }
}
