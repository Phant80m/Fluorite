use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption]) -> String {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const _: &str = env!("CARGO_PKG_NAME");
    String::from(format!("**Dinosaur:**\n Version: {}", VERSION))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}
