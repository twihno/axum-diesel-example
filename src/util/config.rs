//! Functions to provide the base config for the entire program
//! (e.g. logging conf, server conf)

use super::env::get_typed_env_or_panic;

/// The base config ("the config necessary for starting the program")
/// for the entire program (http server config, log config)
#[derive(Debug, Clone)]
pub struct Config {
    pub server_config: ServerConfig,
    pub db_config: DbConfig,
}

/// The config related to the webserver
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub base_url: String,
    pub only_localhost: bool,
}

/// The config related to the database
#[derive(Debug, Clone)]
pub struct DbConfig {
    pub url: String,
}

/// Loads the base config from the values stored in the environment variables.
/// Also uses available `.env` files to populate the environment variables.
///
/// # Panics
/// Invalid types/values for config.
pub fn load_config() -> Config {
    let mut server_config = ServerConfig {
        port: 8000,
        base_url: "/".to_string(),
        only_localhost: false,
    };

    server_config.port = get_typed_env_or_panic::<u16>("PORT", server_config.port);
    server_config.base_url = get_typed_env_or_panic::<String>("BASE_URL", server_config.base_url);
    server_config.only_localhost =
        get_typed_env_or_panic::<bool>("ONLY_LOCALHOST", server_config.only_localhost);

    let mut db_config = DbConfig {
        url: String::from(""),
    };

    db_config.url = get_typed_env_or_panic::<String>("DATABASE_URL", db_config.url);

    Config {
        server_config,
        db_config,
    }
}
