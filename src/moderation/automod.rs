use fluoride::{keywords, Config};
use poise::serenity_prelude as serenity;
use serenity::{Context as SerenityContext, Message};

use crate::moderation::essential::announcer;

use super::essential::logger;

pub async fn init(ctx: &SerenityContext, msg: &Message) {
    let cfg = Config::construct();
    let keywords = keywords();
    if !keywords
        .iter()
        .any(|keyword| msg.content.to_lowercase().contains(keyword))
    {
        return;
    }
    //
    if let Err(why) = msg.delete(&ctx.http).await {
        println!("Error deleting message: {:?}", why);
    }

    if let Err(e) = announcer::enable(&msg, &ctx, &cfg).await {
        eprintln!("{:?}", e);
    }
    logger::enable(&msg, &ctx, &cfg).await;
}
