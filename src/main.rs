use std::sync::Arc;

use axum_template::api::start_server;
use axum_template::db::init_db_connection;
use axum_template::util::config::load_config;
use axum_template::util::env::load_dotenv_file;
use axum_template::util::tracing::init_tracing;

#[tokio::main]
async fn main() {
    let dotenv_result = load_dotenv_file();
    init_tracing();
    dotenv_result.log_dotenv_load_result();

    let config = Arc::new(load_config());
    let db_pool = init_db_connection(&config.db_config).await;
    tokio::join!(start_server(config.clone(), db_pool));
}
