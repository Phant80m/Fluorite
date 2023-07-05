use crate::moderation::functions::*;
use chrono::{DateTime, Duration, Local, Utc};
use fluorite::get_keywords;
use owo_colors::OwoColorize;
use serenity::model::channel::Message;

use serenity::prelude::*;
use std::fs;
pub async fn init(ctx: &Context, msg: &Message) {
    // generate key structs

    let cfg = Config::construct();
    let time = Time::construct();

    // keywords for auto mod, can be edited in crate root ./keywords (no capitals!)
    let keywords = get_keywords();
    if keywords
        .iter()
        .any(|keyword| msg.content.to_lowercase().contains(keyword))
    {
        if let Err(why) = msg.delete(&ctx.http).await {
            println!("Error sending message: {:?}", why);
        }

        // dm
        direct_message::enable(&msg, &time, &ctx, &cfg).await;
        //public shame
        announce::enable(&msg, &ctx, &cfg).await;
        // log
        logger::enable(&msg, &ctx, &cfg).await;
        // mute members
        mute::enable(&msg, &ctx, &time, &cfg).await;
    }
}
pub struct Config {
    pub public_shame: bool,
    pub dm_warning: bool,
    pub do_logs: bool,
    pub do_mutes: bool,
}
pub struct Time {
    pub mute_duration: String,
    pub format_mute_duration: String,
}
impl Time {
    pub fn construct() -> Time {
        let duration = Duration::hours(2) + Duration::minutes(30) + Duration::seconds(15);
        let result_datetime: DateTime<Utc> = Utc::now() + duration;
        let mute_duration = result_datetime.to_rfc3339();
        let local_datetime: DateTime<Local> = result_datetime.into();
        let format_mute_duration = local_datetime.format("%Y-%m-%d :: %I:%M:%S %p").to_string();

        return Time {
            mute_duration,
            format_mute_duration,
        };
    }
}
impl Config {
    pub fn construct() -> Config {
        // fluorite config language very simple true or false bool orientatied language.
        let config_file = fs::read_to_string("./config.fcl").expect("Failed to read config file.");
        //
        let public_shame = config_gen(&config_file, "public_shame = true", "public_shame = false");
        let dm_warning = config_gen(&config_file, "dm_warning = true", "dm_warning = false");
        let do_logs = config_gen(&config_file, "do_logs = true", "do_logs = false");
        let do_mutes = config_gen(&config_file, "do_mutes = true", "do_mutes = false");

        //
        return Config {
            public_shame,
            dm_warning,
            do_logs,
            do_mutes,
        };
    }
}
fn config_gen(config_file: &str, true_: &str, false_: &str) -> bool {
    if config_file.contains(true_) {
        true
    } else if config_file.contains(false_) {
        false
    } else {
        println!(
            "{} {}",
            "[ Fluorite Config Language]".bold().red(),
            "Panicked at Invalid configuration file. Has everything been included?"
                .bold()
                .yellow()
        );
        false
    }
}

pub fn time_now() -> String {
    let local_time = Local::now();
    return local_time.format("%Y-%m-%d :: %I:%M:%S %p").to_string();
}
