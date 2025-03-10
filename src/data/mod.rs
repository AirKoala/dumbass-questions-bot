pub mod config;
pub mod question;
pub mod database;

use poise::serenity_prelude as serenity;
use std::{collections::HashMap, sync::Mutex};

// Custom user data passed to all command functions
pub struct Data {
    config_cache: Mutex<HashMap<serenity::GuildId, config::Config>>,
    questions_cache: Mutex<HashMap<serenity::GuildId, Vec<question::Question>>>,
}
impl Data {
    pub fn new() -> Self {
        Self {
            config_cache: Mutex::new(HashMap::new()),
            questions_cache: Mutex::new(HashMap::new()),
        }
    }

    pub async fn get_config_for(
        &self,
        guild_id: serenity::GuildId,
    ) -> Result<config::Config, crate::Error> {
        Ok(self
            .config_cache
            .lock()
            .unwrap()
            .entry(guild_id)
            .or_insert_with(|| config::Config::default()) // TODO: fetch from db
            .clone())
    }

    pub async fn set_config_for(
        &self,
        guild_id: serenity::GuildId,
        config: &config::Config,
    ) -> Result<(), crate::Error> {
        println!("Set config for guild {}", guild_id);

        // TODO: save to db
        self.config_cache.lock().unwrap().insert(guild_id, config.clone());
        Ok(())
    }

    pub async fn get_questions_for(
        &self,
        guild_id: serenity::GuildId,
    ) -> Result<Vec<question::Question>, crate::Error> {
        Ok(self
            .questions_cache
            .lock()
            .unwrap()
            .entry(guild_id)
            .or_insert_with(|| vec![]) // TODO: fetch from db
            .clone())
    }
}
