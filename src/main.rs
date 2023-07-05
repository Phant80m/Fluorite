mod commands;
mod moderation;

use serenity::all::OnlineStatus;
use serenity::async_trait;

use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::cache::Cache;
use serenity::gateway::ActivityData;
use serenity::model::application::{Command, Interaction};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;

use serenity::prelude::*;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        moderation::moderation::language(&ctx, &msg).await;
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        //
        println!("{} is connected!", ready.user.name);
        let _guild_command =
            Command::create_global_command(&ctx.http, commands::ping::register()).await;
        ctx.set_presence(
            Some(ActivityData::competing("Against users.")),
            OnlineStatus::Online,
        );
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),

                _ => Some("not implemented yet".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
