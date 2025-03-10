use super::utils::reply_error;
use crate::{Context, Data, Error};

/// Add a qotd to the queue. Warning: any previous item in the queue (approved or not) will be replaced.
#[poise::command(slash_command)]
async fn qotd(
    ctx: Context<'_>,
    #[description = "The question to ask"] question: String,
) -> Result<(), Error> {
    println!("author: {}, qotd: {:?}", ctx.author().name, question);
    reply_error(ctx, "Not Implemented.").await
}

pub fn commands() -> Vec<poise::Command<Data, Error>> {
    vec![qotd()]
}
