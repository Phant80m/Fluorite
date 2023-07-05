use crate::moderation::automod::time_now;
use fluorite::Config;

use owo_colors::OwoColorize;
use serenity::all::ChannelId;
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::model::channel::Message;
use serenity::model::Timestamp;
use serenity::prelude::*;

use std::fs::OpenOptions;
use std::io::Write;

pub async fn enable(msg: &Message, ctx: &Context, cfg: &Config) {
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("./logs.txt")
        .expect("Failed to open log file");

    // let time_string = local_time.format("%I:%M:%S %p").to_string();
    if cfg.do_logs {
        if let Some(guild_id) = msg.guild_id {
            match guild_id.member(ctx.http(), msg.author.id).await {
                Ok(member) => {
                    println!(
                        "[ {} ]: {} -> {}",
                        time_now().bold().yellow(),
                        member.display_name().to_string().bold().red(),
                        &msg.content.to_string().bold().white()
                    );
                    writeln!(
                        &mut log_file,
                        "[ {} ]: {} -> {}",
                        time_now(),
                        member.display_name().to_string(),
                        &msg.content.to_string()
                    )
                    .expect("Failed to write to log file");
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
    }
    // put your channel ID here
    if cfg.do_logs {
        if let Err(why) = ChannelId::new(cfg.logging_channel.unwrap_or_default())
            .send_message(
                &ctx.http,
                CreateMessage::new().embed(
                    CreateEmbed::new()
                        .title(format!("Language alert:"))
                        .description(format!("{} said: {}", &msg.author, &msg.content))
                        .timestamp(Timestamp::now()),
                ),
            )
            .await
        {
            println!("Error sending message: {:?}", why);
        }
    }
}
