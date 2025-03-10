use poise::CreateReply;

use crate::commands::{Context, Error};
use std::fmt::Display;

pub async fn reply_error(ctx: Context<'_>, error: impl Display) -> Result<(), Error> {
    ctx.send(
        ctx.reply_builder(CreateReply::default())
            .content(format!("```Error: {}```", error))
            .reply(true)
            .ephemeral(true),
    )
    .await?;

    Err(error.to_string().into())
}
