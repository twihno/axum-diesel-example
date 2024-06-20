use diesel::prelude::*;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tracing::info;

use crate::util::{config::DbConfig, exit::exit_critical};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub type DbPool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

/// Applies the pending migrations
/// and creates a connection pool to connect to the database
pub async fn init_db_connection(config: &DbConfig) -> DbPool {
    run_db_migrations(config);

    let config =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(config.url.clone());
    match bb8::Pool::builder().build(config).await {
        Err(err) => exit_critical(
            &format!("Couldn't initialize db connection pool: \"{:?}\"", err),
            true,
        ),
        Ok(val) => val,
    }
}

/// Connects to the database and applies all pending migrations
fn run_db_migrations(config: &DbConfig) {
    let mut db_connection = PgConnection::establish(&config.url)
        .unwrap_or_else(|_| exit_critical("Couldn't establish database connection", true));

    let result = match db_connection.run_pending_migrations(MIGRATIONS) {
        Err(_) => exit_critical("Database migrations failed", true),
        Ok(val) => val,
    };

    for migration in result {
        info!("Applied database migration: {}", migration)
    }
}
