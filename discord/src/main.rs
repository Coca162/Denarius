#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use dotenvy::dotenv;
use shared::AekosiaAPI;
use reqwest::Client;
use std::env;
use env_logger;

use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, AekosiaAPI, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Gets the balance of a individual
#[poise::command(slash_command, prefix_command, track_edits)]
async fn balance(
    ctx: Context<'_>,
    #[description = "Selected person"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author()).id.as_u64();
    let id = ctx.data().get_id_from_discord(user).await?;
    let balance = ctx.data().get_balance(&id).await?;
    ctx.say(balance.to_string()).await?;
    Ok(())
}

/// Registers you as a new person
#[poise::command(slash_command, prefix_command)]
async fn register(
    ctx: Context<'_>
) -> Result<(), Error> {
    let user = ctx.author().id.as_u64();
    ctx.data().register_person(user).await?;
    ctx.say("Registered!").await?;
    Ok(())
}

#[poise::command(prefix_command, hide_in_help)]
async fn register_commands(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

/// Show this menu
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: "\
Type `;help command` for more info on a command.
You can edit most command messages and the bot will edit its response.",
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().expect("A .env file does not exist!");
    env_logger::init();

    let api = {
        let client = Client::new();

        let website_url = format!("http://{}", env::var("WEBSITE_URL").expect("Could not find discord token from environment variables!"));

        AekosiaAPI::new(client, &website_url)
    };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(";".into()),
                edit_tracker: Some(poise::EditTracker::for_timespan(std::time::Duration::from_secs(120))),
                ..Default::default()
            },
            commands: vec![age(), register(), balance(), register_commands(), help()],
            ..Default::default()
        })
        .token(env::var("DISCORD_TOKEN").expect("Could not find discord token from environment variables!"))
        .intents(serenity::GatewayIntents::non_privileged().union(serenity::GatewayIntents::MESSAGE_CONTENT))
        .setup(|_ctx, _ready, _framework| Box::pin(async { Ok(api) }));

    framework.run().await.unwrap();
}