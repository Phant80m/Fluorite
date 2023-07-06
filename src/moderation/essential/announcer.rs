use fluoride::Config;
use poise::serenity_prelude::{self as serenity, Result};
use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::{Context as SerenityContext, Message};

pub async fn enable(msg: &Message, ctx: &SerenityContext, cfg: &Config) -> Result<()> {
    if !cfg.announcer {
        return Ok(());
    }
    // let response = CreateEmbed::default();
    // response
    //     .title("Thought you could get away?")
    //     .description(format!("{} said {}!", msg.author, msg.content))
    //     .color(16753920);
    // let message = CreateMessage::default();
    // message.embed(response);
    // msg.channel_id.send_message(&ctx.http, message);
    Ok(())
}
