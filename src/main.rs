mod commands;
mod moderation;
use fluorite::check_for_guild;
use poise::serenity_prelude;
use owo_colors::OwoColorize;
use serenity::all::OnlineStatus;
use serenity::async_trait;
use std::fs::File;
use std::io::{self, Write};
use tokio::fs;

use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};

use serenity::gateway::ActivityData;
use serenity::model::application::{Command, Interaction};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;

use serenity::prelude::*;
use std::env;
struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type ContextPoise<'a> = poise::Context<'a, Data, Error>;
struct Handler;

async fn age(
    ctx: ContextPoise<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        moderation::automod::init(&ctx, &msg).await;
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        //
        if let Err(e) = check_for_config().await {
            println!("Error {:?}", e);
        }
        if let Err(e) = check_for_guild() {
            println!("{:?}", e);
        }
        //
        println!("{} is connected!", ready.user.name);
        let _guild_command =
            Command::create_global_command(&ctx.http, commands::ping::register()).await;
        ctx.set_presence(Some(ActivityData::listening("/ping")), OnlineStatus::Online);
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
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age()],
            ..Default::default()
        })

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

async fn check_for_config() -> io::Result<()> {
    if let Err(_) = fs::metadata("./config.fcl").await {
        println!(
            "{}",
            "[ core ]: Config file not found creating one for you at ./config.fcl ..."
                .bold()
                .green()
        );
        let path = "./config.fcl";
        let mut output = File::create(path)?;

        let config = r#"
// fluorite config

public_shame = true
dm_warning = true
do_logs = true
do_mutes = true

logging_channel = <channelID here>
"#;

        write!(output, "{}", config)?;
        println!(
            "{}",
            "[ Core ]: Config file should be created!".bold().yellow()
        );
        println!(
            "{}",
            "[ Core ]: Make sure to edit the 'logging_channel' in the config.fcl"
                .bold()
                .red()
        );
        std::process::exit(0);
    }
    Ok(())
}
