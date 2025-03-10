use super::config;
use poise::serenity_prelude as serenity;
use sqlx::SqlitePool;

pub async fn connect(url: &str, run_migration: bool) -> SqlitePool {
    let pool = SqlitePool::connect(url).await.unwrap_or_else(|e| {
        panic!("Failed to connect to database: {}", e);
    });

    if run_migration {
        migrate(&pool).await;
    }

    pool
}

async fn migrate(pool: &SqlitePool) {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .unwrap_or_else(|e| {
            panic!("Failed to migrate database: {}", e);
        });
}

pub async fn disconnect(pool: &SqlitePool) {
    pool.close().await;
}

pub async fn fetch_config_for(
    pool: &SqlitePool,
    guild_id: serenity::GuildId,
) -> Result<Option<config::Config>, crate::Error> {
    let gid = guild_id.get() as i64;

    sqlx::query!(
        "SELECT * FROM config
         WHERE guild_id = ?
         LIMIT 1",
        gid
    )
    .map(|row| config::Config {
        qotd_channel: row
            .qotd_channel_id
            .map(|id| serenity::ChannelId::new(id as u64)),
        queue_channel: row
            .queue_channel_id
            .map(|id| serenity::ChannelId::new(id as u64)),

        // Schedule is Some(...) if both fields are Some(...)
        schedule: match (
            row.schedule_frequency_hours,
            row.schedule_first_post_timestamp,
        ) {
            (Some(freq), Some(ts)) => Some(config::Schedule {
                frequency_hours: freq as u32,
                first_post: chrono::DateTime::from_timestamp_millis(ts).unwrap_or_default(),
            }),
            _ => None,
        },

        ping_role: row.ping_role_id.map(|id| serenity::RoleId::new(id as u64)),
        mod_role: row.mod_role_id.map(|id| serenity::RoleId::new(id as u64)),
        blacklist_role: row
            .blacklist_role_id
            .map(|id| serenity::RoleId::new(id as u64)),
        whitelist_role: row
            .whitelist_role_id
            .map(|id| serenity::RoleId::new(id as u64)),
        autoapprove: row.autoapprove == 1,
        autothread: row.autothread == 1,
    })
    .fetch_optional(pool)
    .await
    .map_err(|e| crate::Error::from(e))
}

pub async fn set_config_for(
    pool: &SqlitePool,
    guild_id: serenity::GuildId,
    config: config::Config,
) -> Result<(), crate::Error> {
    let gid = guild_id.get() as i64;
    let qotd_channel_id = config.qotd_channel.map(|c| c.get() as i64);
    let queue_channel_id = config.queue_channel.map(|c| c.get() as i64);
    let (schedule_frequency_hours, schedule_first_post_timestamp) =
        config.schedule.map_or((None, None), |s| {
            (
                Some(s.frequency_hours as i64),
                Some(s.first_post.timestamp_millis() as i64),
            )
        });
    let ping_role_id = config.ping_role.map(|r| r.get() as i64);
    let mod_role_id = config.mod_role.map(|r| r.get() as i64);
    let blacklist_role_id = config.blacklist_role.map(|r| r.get() as i64);
    let whitelist_role_id = config.whitelist_role.map(|r| r.get() as i64);
    let autoapprove = config.autoapprove as i64;
    let autothread = config.autothread as i64;

    sqlx::query!(
        "INSERT OR REPLACE INTO config (
            guild_id,
            qotd_channel_id,
            queue_channel_id,
            schedule_frequency_hours,
            schedule_first_post_timestamp,
            ping_role_id,
            mod_role_id,
            blacklist_role_id,
            whitelist_role_id,
            autoapprove,
            autothread
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        gid,
        qotd_channel_id,
        queue_channel_id,
        schedule_frequency_hours,
        schedule_first_post_timestamp,
        ping_role_id,
        mod_role_id,
        blacklist_role_id,
        whitelist_role_id,
        autoapprove,
        autothread,
    )
    .execute(pool)
    .await
    .map_err(|e| crate::Error::from(e))?;
    Ok(())
}
