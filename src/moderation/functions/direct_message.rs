use crate::moderation::automod::Time;
use fluorite::Config;

use serenity::builder::{CreateEmbed, CreateEmbedFooter, CreateMessage};
use serenity::model::channel::Message;

use serenity::prelude::*;

pub async fn enable(msg: &Message, time: &Time, ctx: &Context, cfg: &Config) {
    let footer =
        CreateEmbedFooter::new(format!("Because you said: *{}*", &msg.content.to_string()));
    let builder = CreateMessage::new().embed(
        CreateEmbed::new()
            .title("Watch your lanuage")
            .color(16753920)
            .description(format!(
                "You have been muted until {}",
                &time.format_mute_duration
            ))
            .footer(footer),
    );
    if cfg.dm_warning {
        if let Err(why) = msg.author.direct_message(&ctx, builder).await {
            println!("Error sending message: {:?}", why);
        }
    }
}
