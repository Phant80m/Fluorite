use fluoride::{time_now, Config};
use owo_colors::OwoColorize;
use poise::serenity_prelude::{self as serenity, Timestamp};
use serenity::model::id::ChannelId;
use serenity::{Context as SerenityContext, Message};
use std::{fs::OpenOptions, io::Write};

pub async fn enable(msg: &Message, ctx: &SerenityContext, cfg: &Config) {
    if !cfg.do_logs {
        return;
    }
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("./logs.txt")
        .expect("Failed to open log file");
    if let Some(guild_id) = msg.guild_id {
        match guild_id.member(&ctx.http, msg.author.id).await {
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
    if let Some(guild_id) = msg.guild_id {
        if let Ok(member) = guild_id.member(&ctx.http, msg.author.id).await {
            let user = &member.user;
            let warn = ChannelId(cfg.logging_channel.unwrap_or(0)).send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("language log")
                        .description(format!("said: {}", msg.content))
                        .color(16753920)
                        .author(|a| a.icon_url(&user.face()).name(&user.name))
                        .timestamp(Timestamp::now())
                })
            });
            if let Err(e) = warn.await {
                eprintln!("{:?}", e);
            }
        } else if let Err(e) = guild_id.member(&ctx.http, msg.author.id).await {
            eprintln!("{}", e);
        }
    }
}
