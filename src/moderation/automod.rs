use crate::moderation::functions::*;
use chrono::{DateTime, Duration, Local, Utc};
use fluorite::{get_keywords, Config};
use serenity::model::channel::Message;

use serenity::prelude::*;
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
pub fn time_now() -> String {
    let local_time = Local::now();
    return local_time.format("%Y-%m-%d :: %I:%M:%S %p").to_string();
}
