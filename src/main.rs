#![warn(
    clippy::str_to_string,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

use poise::serenity_prelude::{self as serenity, GatewayIntents};

// Type aliases for poise.
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
// This is global and easier to use than Serenity's implementation.

struct Data {}

/// Needed to register/remove slash commands, idk how this works,
/// but I shall have to learn. Need to make this command owner only.
#[poise::command(prefix_command, owners_only)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or(ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Show this help menu
#[poise::command(prefix_command, track_edits, slash_command)]
async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about (or type nothing for a list of all commands!)"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "\
This bot is developed in the Poise framework in Rust.\
To contribute, see the code, see: github.com/nullndvoid/a.bot",
            show_context_menu_commands: true,
            ephemeral: true,
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // Loads environment variables from .env
    dotenv::dotenv().ok();

    // MESSAGE_CONTENT is needed for basic ~prefix commands,
    // However we will move quickly over to slash commands.
    let intents_bits =
        GatewayIntents::MESSAGE_CONTENT.bits() | GatewayIntents::non_privileged().bits();
    // Unwrapping this will always succeed so long as the correct bits are set.
    let intents = GatewayIntents::from_bits(intents_bits).unwrap();

    let framework = poise::Framework::build()
        .options(poise::FrameworkOptions {
            commands: vec![help(), age(), register()], // rust_analyzer can't handle this but its okay.
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
        // unwrapping things like the TOKEN are okay, because we need it anyway, so if it
        // fails you get a clue what you are missing.
        .token(std::env::var("BOT_TOKEN").unwrap())
        .intents(intents)
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await.unwrap();
}
