use crate::moderation::automod::{Config, Time};

use serenity::model::channel::Message;
use serenity::model::Timestamp;
use serenity::prelude::*;

pub async fn enable(msg: &Message, ctx: &Context, time: &Time, cfg: &Config) {
    if cfg.do_mutes {
        if msg.author.id.to_string() == "744784263932674079" {
            // makes riot evr4 exempt to mutes for testing purposes
        } else if let Some(guild_id) = msg.guild_id {
            match guild_id.member(ctx.http(), msg.author.id).await {
                Ok(mut member) => {
                    match member
                        .disable_communication_until_datetime(
                            ctx.http(),
                            Timestamp::parse(&time.mute_duration).unwrap(),
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
            println!("Muted Them until {}", &time.format_mute_duration)
        } else {
            println!("something did not go right");
        }
    }
}
