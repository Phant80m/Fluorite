use chrono::{DateTime, Duration, Local, Utc};
use discord::get_keywords;
use owo_colors::OwoColorize;
use serenity::all::{ChannelFlags, ChannelId};
use serenity::builder::{CreateEmbed, CreateEmbedFooter, CreateMessage};
use serenity::model::channel::Message;
use serenity::model::Timestamp;
use serenity::prelude::*;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

struct Config {
    public_shame: bool,
    dm_warning: bool,
    do_logs: bool,
}
impl Config {
    fn construct() -> Config {
        let config_file = fs::read_to_string("./config.fcl").expect("Failed to read config file.");
        //
        let public_shame =
            parse_boolean_value(&config_file, "public_shame = true", "public_shame = false");
        let dm_warning =
            parse_boolean_value(&config_file, "dm_warning = true", "dm_warning = false");
        let do_logs = parse_boolean_value(&config_file, "do_logs = true", "do_logs = false");
        //
        return Config {
            public_shame,
            dm_warning,
            do_logs,
        };
    }
}

fn time_now() -> String {
    let local_time = Local::now();
    return local_time.format("%Y-%m-%d :: %I:%M:%S %p").to_string();
}
pub async fn language(ctx: &Context, msg: &Message) {
    // config parser
    let cfg = Config::construct();

    // time duratiom for mutes
    let duration = Duration::hours(2) + Duration::minutes(30) + Duration::seconds(15);
    let result_datetime: DateTime<Utc> = Utc::now() + duration;
    let formatted_datetime = result_datetime.to_rfc3339();
    let local_datetime: DateTime<Local> = result_datetime.into();
    let twenty_four_datetime = local_datetime.format("%Y-%m-%d :: %I:%M:%S %p").to_string();

    // keywords for auto mod, can be edited in crate root ./keywords
    let keywords = get_keywords();
    if keywords
        .iter()
        .any(|keyword| msg.content.to_lowercase().contains(keyword))
    {
        if let Err(why) = msg.delete(&ctx.http).await {
            println!("Error sending message: {:?}", why);
        }

        // dm
        let footer =
            CreateEmbedFooter::new(format!("Because you said: *{}*", &msg.content.to_string()));
        let builder = CreateMessage::new().embed(
            CreateEmbed::new()
                .title("Watch your lanuage")
                .color(16753920)
                .description(format!(
                    "You have been muted until {}",
                    twenty_four_datetime
                ))
                .footer(footer),
        );
        if cfg.dm_warning {
            if let Err(why) = msg.author.direct_message(&ctx, builder).await {
                println!("Error sending message: {:?}", why);
            }
        }
        //public shame
        if cfg.public_shame {
            if let Err(why) = msg
                .channel_id
                .send_message(
                    &ctx.http,
                    CreateMessage::new().embed(
                        CreateEmbed::new()
                            .title(format!("thought you could get away?"))
                            .description(format!(
                                "{} tried to say said: {}",
                                msg.author, msg.content
                            )),
                    ),
                )
                .await
            {
                println!("Error sending message: {why:?}");
            }
        }
        // log
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
            if let Err(why) = ChannelId::new(1125358725423706112)
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
        if msg.author.id.to_string() == "744784263932674079" {
            // makes riot evr4 exempt to mutes for testing purposes
        } else if let Some(guild_id) = msg.guild_id {
            match guild_id.member(ctx.http(), msg.author.id).await {
                Ok(mut member) => {
                    match member
                        .disable_communication_until_datetime(
                            ctx.http(),
                            Timestamp::parse(&formatted_datetime).unwrap(),
                        )
                        .await
                    {
                        Ok(comms) => comms,
                        Err(e) => {
                            println!("{:?}", e)
                        }
                    }
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                }
            }
            println!("Muted Them until {}", twenty_four_datetime)
        } else {
            println!("something did not go right");
        }
    }
}
fn parse_boolean_value(config_file: &str, true_value: &str, false_value: &str) -> bool {
    if config_file.contains(true_value) {
        true
    } else if config_file.contains(false_value) {
        false
    } else {
        panic!("Invalid config file format.");
    }
}
