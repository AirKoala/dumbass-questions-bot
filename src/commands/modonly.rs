use super::utils::reply_error;
use crate::{Context, Data, Error};
use poise::serenity_prelude as serenity;

/// Add a qotd to the queue. Warning: any previous item in the queue (approved or not) will be replaced.
#[poise::command(
    slash_command,
    rename = "mod",
    subcommands("approve", "reject", "next",)
)]
async fn modonly(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(slash_command)]
async fn approve(
    ctx: Context<'_>,
    #[description = "Link of the message to approve"] message: serenity::Message,
) -> Result<(), Error> {
    println!("message: {:?}", message);
    reply_error(ctx, "Not Implemented.").await
}

#[poise::command(context_menu_command = "Approve")]
async fn context_menu_approve(
    ctx: Context<'_>,
    #[description = "Link of the message to approve"] message: serenity::Message,
) -> Result<(), Error> {
    println!("message: {:?}", message);
    reply_error(ctx, "Not Implemented.").await
}

#[poise::command(slash_command)]
async fn reject(
    ctx: Context<'_>,
    #[description = "Link of the message to reject"] message: serenity::Message,
) -> Result<(), Error> {
    reply_error(ctx, "Not Implemented.").await
}

#[poise::command(slash_command)]
async fn next(ctx: Context<'_>) -> Result<(), Error> {
    reply_error(ctx, "Not Implemented.").await
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![modonly(), context_menu_approve()]
}
