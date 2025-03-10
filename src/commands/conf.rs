use super::utils::reply_error;
use crate::{data::config::Schedule, Context, Data, Error};
use chrono::{offset::LocalResult, TimeZone};
use poise::serenity_prelude as serenity;

use eyre::{eyre, Result};

#[poise::command(
    slash_command,
    subcommands(
        "pingrole",
        "modrole",
        "whitelistrole",
        "blacklistrole",
        "queuechannel",
        "qotdchannel",
        "show",
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
        serenity::RoleId,
    >,
) -> Result<(), Error> {
    let guild_id = get_guild_id(ctx).await?;
    let mut config = ctx.data().get_config_for(guild_id).await?;

    config.ping_roles = match role {
        Some(role) => vec![role],
        None => vec![],
    };

    ctx.data().set_config_for(guild_id, &config).await?;

    ctx.send(embed_reply(
        config.as_embed()?.title("Config successfully changed."),
    ))
    .await?;

    Ok(())
}

/// Users belonging to this role can approve QOTDs.
#[poise::command(slash_command, default_member_permissions = "ADMINISTRATOR")]
async fn modrole(
    ctx: Context<'_>,
    #[description = "Omit this if you want to remove the mod role."] role: Option<serenity::RoleId>,
) -> Result<(), Error> {
    let guild_id = get_guild_id(ctx).await?;
    let mut config = ctx.data().get_config_for(guild_id).await?;

    config.mod_roles = match role {
        Some(role) => vec![role],
        None => vec![],
    };

    ctx.data().set_config_for(guild_id, &config).await?;

    ctx.send(embed_reply(
        config.as_embed()?.title("Config successfully changed."),
    ))
    .await?;

    Ok(())
}

/// Whitelist a role from submitting QOTDs.
#[poise::command(slash_command)]
async fn whitelistrole(
    ctx: Context<'_>,
    #[description = "Omit this if you want to disable whitelisting."] role: Option<
        serenity::RoleId,
    >,
) -> Result<(), Error> {
    let guild_id = get_guild_id(ctx).await?;
    let mut config = ctx.data().get_config_for(guild_id).await?;

    config.whitelist_roles = match role {
        Some(role) => vec![role],
        None => vec![],
    };

    ctx.data().set_config_for(guild_id, &config).await?;

    ctx.send(embed_reply(
        config.as_embed()?.title("Config successfully changed."),
    ))
    .await?;

    Ok(())
}

/// Blacklist a role from submitting QOTDs.
#[poise::command(slash_command)]
async fn blacklistrole(
    ctx: Context<'_>,
    #[description = "Omit this if you want to disable blacklisting."] role: Option<
        serenity::RoleId,
    >,
) -> Result<(), Error> {
    let guild_id = get_guild_id(ctx).await?;
    let mut config = ctx.data().get_config_for(guild_id).await?;

    config.blacklist_roles = match role {
        Some(role) => vec![role],
        None => vec![],
    };

    ctx.data().set_config_for(guild_id, &config).await?;

    ctx.send(embed_reply(
        config.as_embed()?.title("Config successfully changed."),
    ))
    .await?;

    Ok(())
}

/// Set the channel to use as a submission queue.
#[poise::command(slash_command)]
async fn queuechannel(
    ctx: Context<'_>,
    #[description = "The channel to send the queue to"]
    #[channel_types("Text")]
    channel: serenity::GuildChannel,
) -> Result<(), Error> {
    let guild_id = get_guild_id(ctx).await?;
    let mut config = ctx.data().get_config_for(guild_id).await?;

    config.queue_channel = Some(channel.id);

    ctx.data().set_config_for(guild_id, &config).await?;

    ctx.send(embed_reply(
        config.as_embed()?.title("Config successfully changed."),
    ))
    .await?;

    Ok(())
}

/// Set the channel to send the QOTD to.
#[poise::command(slash_command)]
async fn qotdchannel(
    ctx: Context<'_>,
    #[description = "The channel to send the queue to"]
    #[channel_types("Text")]
    channel: serenity::GuildChannel,
) -> Result<(), Error> {
    let guild_id = get_guild_id(ctx).await?;
    let mut config = ctx.data().get_config_for(guild_id).await?;

    config.qotd_channel = Some(channel.id);

    ctx.data().set_config_for(guild_id, &config).await?;

    ctx.send(embed_reply(
        config.as_embed()?.title("Config successfully changed."),
    ))
    .await?;

    Ok(())
}

/// Automatically create a thread for each QOTD.
#[poise::command(slash_command)]
async fn autothread(
    ctx: Context<'_>,
    #[description = "Whether to automatically create a thread for each QOTD."] autothread: bool,
) -> Result<(), Error> {
    let guild_id = get_guild_id(ctx).await?;
    let mut config = ctx.data().get_config_for(guild_id).await?;

    config.autothread = autothread;

    ctx.data().set_config_for(guild_id, &config).await?;
    Ok(())
}

/// Automatically approve every submission.
#[poise::command(slash_command)]
async fn autoapprove(
    ctx: Context<'_>,
    #[description = "Whether to automatically approve every submission."] autoapprove: bool,
) -> Result<(), Error> {
    let guild_id = get_guild_id(ctx).await?;
    let mut config = ctx.data().get_config_for(guild_id).await?;

    config.autoapprove = autoapprove;

    ctx.data().set_config_for(guild_id, &config).await?;
    Ok(())
}

/// Set the frequency of QOTDs.
#[poise::command(slash_command)]
async fn schedule(
    ctx: Context<'_>,

    #[description = "Number of hours between posts."]
    #[min = 1]
    frequency: u32,

    #[description = "Timezone as offset hours from UTC. (eg. Dhaka is +6)"]
    #[max = 12]
    #[min = -12]
    timezone: i32,

    #[min_length = 5]
    #[max_length = 5]
    #[description = "Time of day to post first QOTD. Format: HH:MM (24-hour time)"]
    time: String,
) -> Result<(), Error> {
    let guild_id = get_guild_id(ctx).await?;
    let mut config = ctx.data().get_config_for(guild_id).await?;

    let parsed_time = chrono::NaiveTime::parse_from_str(&time, "%H:%M")?;
    let parsed_tz = chrono::FixedOffset::east_opt(timezone * 3600);
    if parsed_tz.is_none() {
        return reply_error(ctx, "Invalid timezone.").await;
    }
    let parsed_tz = parsed_tz.unwrap();

    let first_post = parsed_tz
        .from_utc_datetime(&chrono::Utc::now().naive_utc())
        .with_time(parsed_time)
        .unwrap() // unwrap is fine here because FixedOffset never fails
        .with_timezone(&chrono::Utc);

    config.schedule = Some(Schedule {
        frequency_hours: frequency,
        first_post,
    });

    ctx.data().set_config_for(guild_id, &config).await?;
    ctx.send(embed_reply(
        config.as_embed()?.title("Config successfully changed."),
    ))
    .await?;

    Ok(())
}

/// Show the current configuration.
#[poise::command(slash_command)]
async fn show(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = get_guild_id(ctx).await?;
    let config = ctx.data().get_config_for(guild_id).await?;

    ctx.send(embed_reply(config.as_embed()?)).await?;

    Ok(())
}

async fn get_guild_id(ctx: Context<'_>) -> Result<serenity::GuildId> {
    match ctx.guild_id() {
        Some(guild_id) => Ok(guild_id),
        None => {
            let _ = reply_error(ctx, "This command must be run in a server.").await;
            Err(eyre!("Not in a server."))
        }
    }
}

fn embed_reply(embed: serenity::CreateEmbed) -> poise::CreateReply {
    poise::CreateReply::default().embed(embed)
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![conf()]
}
