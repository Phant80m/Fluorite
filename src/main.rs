mod moderation;
use poise::serenity_prelude as serenity;
use serenity::{
    Activity, Context as SerenityContext, GatewayIntents, Message, OnlineStatus, Ready,
};
struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
struct Handler;

#[serenity::async_trait]
impl serenity::EventHandler for Handler {
    async fn message(&self, ctx: SerenityContext, msg: Message, pctx: &Context) {
        moderation::automod::init(&ctx, &msg, &pctx).await
    }
    async fn ready(&self, ctx: SerenityContext, ready: Ready) {
        println!("{} is connected", ready.user.name);
        ctx.set_presence(Some(Activity::listening("/ping")), OnlineStatus::Online)
            .await;
    }
}
/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!(
        "{}'s account was created on {} at {}",
        u.name,
        u.created_at().format("%Y-%m-%d").to_string(),
        u.created_at().format("%I:%M:%S %p").to_string()
    );
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(intents)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .client_settings(|x| x.event_handler(Handler));

    framework.run().await.unwrap();
    println!("bot started");
}
