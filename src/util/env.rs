//! Helper functions for working with environment variables (or at least get their values)

use dotenvy::dotenv;
use std::{
    any::type_name,
    env::{self, VarError},
    str::FromStr,
};
use tracing::{debug, warn};

use super::exit::exit_critical;

/// Errors that can occur during the reading and parsing of
/// an environment variable
#[derive(thiserror::Error, Debug)]
pub enum EnvError {
    #[error("Environment variable \"{variable_name}\" not present")]
    /// Environment variable not present
    NotPresent { variable_name: String },
    #[error("The value of the environment variable \"{variable_name}\" is not valid unicode")]
    /// Environment variable not valid unicode
    NotUnicode { variable_name: String },
    #[error("Error while trying to parse the environment variable \"{variable_name}\" as the requested type \"{r#type}\"")]
    /// Error while trying to parse the environment variable as the requested type
    ParseError {
        variable_name: String,
        r#type: String,
    },
}

/// Tries to retrieve the environment variable `key` and convert it to
/// the desired type.
pub fn get_typed_env<T: FromStr>(key: &str) -> Result<T, EnvError> {
    let env_value = match env::var(key) {
        Err(err) => match err {
            VarError::NotPresent => {
                return Err(EnvError::NotPresent {
                    variable_name: key.to_string(),
                })
            }
            VarError::NotUnicode(_) => {
                return Err(EnvError::NotUnicode {
                    variable_name: key.to_string(),
                })
            }
        },
        Ok(val) => val,
    };

    match env_value.parse::<T>() {
        Err(_) => Err(EnvError::ParseError {
            variable_name: key.to_string(),
            r#type: type_name::<T>().to_string(),
        }),
        Ok(val) => Ok(val),
    }
}

/// Tries to retrieve the environment variable `key` and convert it to
/// the desired type.
///
/// If the environment variable isn't set, it returns the default value.
pub fn get_typed_env_with_default<T: FromStr>(key: &str, default: T) -> Result<T, EnvError> {
    let err = match get_typed_env::<T>(key) {
        Ok(val) => return Ok(val),
        Err(err) => err,
    };

    match err {
        EnvError::NotPresent { .. } => Ok(default),
        EnvError::NotUnicode { .. } => Err(err),
        EnvError::ParseError { .. } => Err(err),
    }
}

/// Tries to retrieve the environment variable `key` and convert it to
/// the desired type. If the variable isn't set, the default value
/// will be returned.
///
/// # Panics
/// - `NotUnicode`: Unicode error. Inherited from `std::env::var`
/// - `ParseError`: Parsing error:
///     Couldn't convert the environment variable to the desired type
pub fn get_typed_env_or_panic<T: FromStr>(key: &str, default: T) -> T {
    let env_val = get_typed_env(key);

    match env_val {
        Ok(val) => val,
        Err(err) => match err {
            EnvError::NotPresent { .. } => default,
            EnvError::NotUnicode { .. } => exit_critical(
                &format!(
                    "Value for environment variable \"{}\" is not valid unicode.",
                    key
                ),
                true,
            ),
            EnvError::ParseError { .. } => exit_critical(
                &format!("Couldn't parse environment variable \"{}\".", key),
                true,
            ),
        },
    }
}

/// Result after trying to load the .env file
pub enum DotenvLoadResult {
    /// .env present and loaded successfully
    Ok(String),
    /// no .env available
    NoFile,
    /// .env present and loaded with error
    FileWithInvalidPath(String),
}

/// Try to load the .env file and add it to the local environment
pub fn load_dotenv_file() -> DotenvLoadResult {
    if let Ok(pb) = dotenv() {
        return match pb.to_str() {
            None => DotenvLoadResult::FileWithInvalidPath(
                "Using .env file, but couldn't determine its path.".into(),
            ),
            Some(path_str) => DotenvLoadResult::Ok(format!("Using .env file \"{}\"", path_str)),
        };
    };

    DotenvLoadResult::NoFile
}

impl DotenvLoadResult {
    /// Add the `DotenvLoadResult` to the tracing log
    pub fn log_dotenv_load_result(&self) {
        match self {
            DotenvLoadResult::Ok(msg) => debug!("{msg}"),
            DotenvLoadResult::NoFile => (),
            DotenvLoadResult::FileWithInvalidPath(msg) => warn!("{msg}"),
        }
    }
}
