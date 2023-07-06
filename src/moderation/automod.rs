use fluoride::keywords;
use poise::serenity_prelude as serenity;
use serenity::{Context as SerenityContext, Message};
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
struct Data {} // User data, which is stored and accessible in all command invocations

pub async fn init(ctx: &SerenityContext, msg: &Message, pctx: Context) {
    println!("automod initialized");
    let keywords = keywords();
    if keywords
        .iter()
        .any(|keyword| msg.content.to_lowercase().contains(keyword))
    {
        println!("keyword detected");
        if let Err(why) = msg.delete(&ctx.http).await {
            println!("Error sending message: {:?}", why);
        }
    }
}
