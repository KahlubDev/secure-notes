use sqlx::{migrate::Migrator, sqlite::SqlitePoolOptions, SqlitePool};

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn connect() -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://notes.db")
        .await?;

    MIGRATOR.run(&pool).await?;

    Ok(pool)
}