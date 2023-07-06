use fluoride::Config;
use poise::serenity_prelude::{self as serenity, Result, Timestamp};
use serenity::{Context as SerenityContext, Message};

pub async fn enable(msg: &Message, ctx: &SerenityContext, cfg: &Config) -> Result<()> {
    if !cfg.announcer {
        return Ok(());
    }

    if let Some(guild_id) = msg.guild_id {
        if let Ok(member) = guild_id.member(&ctx.http, msg.author.id).await {
            let user = &member.user;
            msg.channel_id
                .send_message(&ctx, |m| {
                    m.embed(|e| {
                        e.title("Thought you could get away?")
                            .description(format!("{} said {}!", msg.author, msg.content))
                            .color(16753920)
                            .author(|a| a.icon_url(&user.face()).name(&user.name))
                            .timestamp(Timestamp::now())
                    })
                })
                .await?;
        } else if let Err(e) = guild_id.member(&ctx.http, msg.author.id).await {
            eprintln!("{}", e);
        }
    }

    Ok(())
}
