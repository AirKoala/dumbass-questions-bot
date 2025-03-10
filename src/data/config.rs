use derivative::Derivative;
use std::fmt::{Display, Formatter};

use poise::serenity_prelude as serenity;
use serenity::Mentionable;

#[derive(Derivative)]
#[derivative(Debug, Default, Clone)]
pub struct Schedule {
    pub frequency_hours: u32,
    pub first_post: chrono::DateTime<chrono::Utc>,
}
impl Display for Schedule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Every {} hours, starting from <t:{}:t>",
            self.frequency_hours,
            self.first_post.timestamp()
        )
    }
}

#[derive(Derivative)]
#[derivative(Debug, Default, Clone)]
pub struct Config {
    pub qotd_channel: Option<serenity::ChannelId>,
    pub queue_channel: Option<serenity::ChannelId>,
    pub schedule: Option<Schedule>,
    pub ping_role: Option<serenity::RoleId>,
    pub mod_role: Option<serenity::RoleId>,
    pub blacklist_role: Option<serenity::RoleId>,
    pub whitelist_role: Option<serenity::RoleId>,
    pub autoapprove: bool,
    pub autothread: bool,
}
impl Config {
    pub fn as_embed(&self) -> Result<serenity::CreateEmbed, crate::Error> {
        Ok(serenity::CreateEmbed::default()
            .title("Configuration")
            .field(
                "QOTD Channel",
                match &self.qotd_channel {
                    Some(c) => c.mention().to_string(),
                    None => "Not set.".to_string(),
                },
                true,
            )
            .field(
                "Queue Channel",
                match &self.queue_channel {
                    Some(c) => c.mention().to_string(),
                    None => "Not set.".to_string(),
                },
                true,
            )
            .field(
                "Schedule",
                match &self.schedule {
                    Some(s) => s.to_string(),
                    None => "Not set.".to_string(),
                },
                false,
            )
            .field("Auto Approve", self.autoapprove.to_string(), true)
            .field("Auto Thread", self.autothread.to_string(), true)
            .field(
                "Ping Roles",
                match &self.ping_role {
                    Some(r) => r.mention().to_string(),
                    None => "Not set.".to_string(),
                },
                false,
            )
            .field(
                "Mod Roles",
                match &self.mod_role {
                    Some(r) => r.mention().to_string(),
                    None => "Not set.".to_string(),
                },
                false,
            )
            .field(
                "Blacklist Roles",
                match &self.blacklist_role {
                    Some(r) => r.mention().to_string(),
                    None => "Not set.".to_string(),
                },
                false,
            )
            .field(
                "Whitelist Roles",
                match &self.whitelist_role {
                    Some(r) => r.mention().to_string(),
                    None => "Not set.".to_string(),
                },
                false,
            ))
    }
}
