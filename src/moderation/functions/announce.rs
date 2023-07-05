use crate::moderation::automod::Config;

use serenity::builder::{CreateEmbed, CreateMessage};
use serenity::model::channel::Message;

use serenity::prelude::*;

pub async fn enable(msg: &Message, ctx: &Context, cfg: &Config) {
    if cfg.public_shame {
        if let Err(why) = msg
            .channel_id
            .send_message(
                &ctx.http,
                CreateMessage::new().embed(
                    CreateEmbed::new()
                        .title(format!("thought you could get away?"))
                        .description(format!("{} tried to say said: {}", msg.author, msg.content)),
                ),
            )
            .await
        {
            println!("Error sending message: {why:?}");
        }
    }
}
