pub mod config;
pub mod database;
pub mod question;

use poise::serenity_prelude as serenity;
use std::{collections::HashMap, sync::Mutex};

// Custom user data passed to all command functions
pub struct Data {
    config_cache: Mutex<HashMap<serenity::GuildId, config::Config>>,
    questions_cache: Mutex<HashMap<serenity::GuildId, Vec<question::Question>>>,
    dbpool: sqlx::SqlitePool,
}
impl Data {
    pub async fn new(dburl: &str) -> Self {
        Self {
            config_cache: Mutex::new(HashMap::new()),
            questions_cache: Mutex::new(HashMap::new()),
            dbpool: database::connect(dburl, true).await,
        }
    }

    pub async fn get_config_for(
        &self,
        guild_id: serenity::GuildId,
    ) -> Result<config::Config, crate::Error> {
        let conf = {
            let cache = self.config_cache.lock().unwrap();
            cache.get(&guild_id).cloned()
        };

        let conf = match conf {
            Some(c) => c.clone(),
            None => {
                let conf = database::fetch_config_for(&self.dbpool, guild_id)
                    .await?
                    .unwrap_or_default();
                self.config_cache
                    .lock()
                    .unwrap()
                    .insert(guild_id, conf.clone());
                conf
            }
        };

        Ok(conf)
    }

    pub async fn set_config_for(
        &self,
        guild_id: serenity::GuildId,
        config: &config::Config,
    ) -> Result<(), crate::Error> {
        println!("Set config for guild {}", guild_id);

        self.config_cache
            .lock()
            .unwrap()
            .insert(guild_id, config.clone());

        database::set_config_for(&self.dbpool, guild_id, config.clone()).await?;
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
