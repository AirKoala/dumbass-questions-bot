mod commands;
mod data;

pub use crate::data::Data;

use poise::serenity_prelude as serenity;
use std::{
    collections::HashMap,
    env::var,
    sync::Mutex,
};

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;



async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().unwrap_or_else(|e| panic!("Failed to load .env file: {}", e));

    // FrameworkOptions contains all of poise's configuration option in one struct
    // Every option can be omitted to use its default value
    let options = poise::FrameworkOptions {
        // commands: vec![commands::help(), commands::vote(), commands::getvotes()],
        commands: commands::commands(),
        // prefix_options: poise::PrefixFrameworkOptions {
        //     prefix: Some("~".into()),
        //     edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
        //         Duration::from_secs(3600),
        //     ))),
        //     additional_prefixes: vec![
        //         poise::Prefix::Literal("hey bot,"),
        //         poise::Prefix::Literal("hey bot"),
        //     ],
        //     ..Default::default()
        // },
        // The global error handler for all error cases that may occur
        on_error: |error| Box::pin(on_error(error)),
        // This code is run before every command
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name);
            })
        },
        // This code is run after a command if it was successful (returned Ok)
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name);
            })
        },
        // Every command invocation must pass this check to continue execution
        command_check: Some(|ctx| {
            Box::pin(async move {
                if ctx.author().id == 123456789 {
                    return Ok(false);
                }
                Ok(true)
            })
        }),
        // Enforce command checks even for owners (enforced by default)
        // Set to true to bypass checks, which is useful for testing
        skip_checks_for_owners: false,
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                println!(
                    "Got an event in event handler: {:?}",
                    event.snake_case_name()
                );
                Ok(())
            })
        },
        ..Default::default()
    };

    // TODO: Move this to a config file
    let testing_guilds = vec![
        593359698800017418, // Test server
    ];

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                // TODO: Check if in debug mode and set registration mode accordingly

                // poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                for gid in testing_guilds {
                    // Register commands in specified guilds. This propagates faster than global registration.

                    if let Err(error) = poise::builtins::register_in_guild(
                        ctx,
                        &framework.options().commands,
                        gid.into(),
                    )
                    .await
                    {
                        println!("Error registering commands in guild {}: {:?}", gid, error);
                    }
                }

                Ok(Data::new(&var("DATABASE_URL").expect("Missing `DATABASE_URL` env var")).await)
            })
        })
        .options(options)
        .build();

    let token = var("DISCORD_TOKEN")
        .expect("Missing `DISCORD_TOKEN` env var, see README for more information.");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap()
}
