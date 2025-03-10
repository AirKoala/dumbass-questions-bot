use super::utils::reply_error;
use crate::{Context, Data, Error};
use poise::serenity_prelude as serenity;

#[poise::command(
    slash_command,
    subcommands(
        "pingrole",
        "modrole",
        "blacklistrole",
        "queuechannel",
        "qotdchannel",
        "all",
        "autothread",
        "autoapprove",
        "schedule",
    )
)]
async fn conf(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Set the role to ping when a QOTD is posted.
#[poise::command(slash_command)]
async fn pingrole(
    ctx: Context<'_>,
    #[description = "Omit this if you want to disable pinging a role."] role: Option<
        serenity::Role,
    >,
) -> Result<(), Error> {
    reply_error(ctx, "Not Implemented.").await
}

/// Users belonging to this role can approve QOTDs.
#[poise::command(slash_command, default_member_permissions = "ADMINISTRATOR")]
async fn modrole(
    ctx: Context<'_>,
    #[description = "Omit this if you want to remove the mod role."] role: Option<serenity::Role>,
) -> Result<(), Error> {
    reply_error(ctx, "Not Implemented.").await
}

/// Blacklist a role from submitting QOTDs.
#[poise::command(slash_command)]
async fn blacklistrole(ctx: Context<'_>) -> Result<(), Error> {
    reply_error(ctx, "Not Implemented.").await
}

/// Set the channel to use as a submission queue.
#[poise::command(slash_command)]
async fn queuechannel(
    ctx: Context<'_>,
    #[description = "The channel to send the queue to"]
    #[channel_types("Text")]
    channel: serenity::GuildChannel,
) -> Result<(), Error> {
    println!("channel: {:?}", channel);
    reply_error(ctx, "Not Implemented.").await
}

/// Set the channel to send the QOTD to.
#[poise::command(slash_command)]
async fn qotdchannel(
    ctx: Context<'_>,
    #[description = "The channel to send the queue to"]
    #[channel_types("Text")]
    channel: serenity::GuildChannel,
) -> Result<(), Error> {
    println!("channel: {:?}", channel);
    reply_error(ctx, "Not Implemented.").await
}

/// Automatically create a thread for each QOTD.
#[poise::command(slash_command)]
async fn autothread(
    ctx: Context<'_>,
    #[description = "Whether to automatically create a thread for each QOTD."] autothread: bool,
) -> Result<(), Error> {
    reply_error(ctx, "Not Implemented.").await
}

/// Automatically approve every submission.
#[poise::command(slash_command)]
async fn autoapprove(
    ctx: Context<'_>,
    #[description = "Whether to automatically approve every submission."] autoapprove: bool,
) -> Result<(), Error> {
    reply_error(ctx, "Not Implemented.").await
}

/// Set the frequency of QOTDs.
#[poise::command(slash_command)]
async fn schedule(
    ctx: Context<'_>,

    #[description = "Number of hours between posts."]
    #[min = 1]
    frequency: u32,

    #[description = "Timezone as offset hours from UTC. (eg. Dhaka is +6)"]
    #[min = -12]
    #[max = 12]
    timezone: i32,

    #[min_length = 5]
    #[max_length = 5]
    #[description = "Time of day to post first QOTD. Format: HH:MM (24-hour time)"]
    time: String,
) -> Result<(), Error> {
    reply_error(ctx, "Not Implemented.").await
}

#[poise::command(slash_command)]
async fn all(ctx: Context<'_>) -> Result<(), Error> {
    reply_error(ctx, "Not Implemented.").await
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![conf()]
}
