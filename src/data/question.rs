use poise::serenity_prelude as serenity;

#[derive(Debug, Clone)]
pub struct Question {
    pub datetime: chrono::DateTime<chrono::Utc>,
    pub content: String,
    pub approved: bool,
    pub author: serenity::UserId,
    pub exhausted: bool,
    pub deleted: bool,
}
