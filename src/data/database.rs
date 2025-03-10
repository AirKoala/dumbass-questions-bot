use sqlx::SqlitePool;

pub async fn connect(url: &str) -> SqlitePool {
    SqlitePool::connect(url).await.unwrap_or_else(|e| {
        panic!("Failed to connect to database: {}", e);
    })
}

pub async fn migrate(pool: &SqlitePool) {
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

