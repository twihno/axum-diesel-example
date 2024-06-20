//! Functionality related to the creation and
//! attachment of api routes

use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use demo::{handle_create_demo, handle_get_demo};
use healthcheck::ping;

use crate::util::app_state::AppState;

pub mod demo;
pub mod healthcheck;

/// Attach routes to the provided router.
/// Creates a new router and returns the modified version
pub fn attach_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router
        .route("/ping", get(ping))
        .route("/demo", get(handle_get_demo))
        .route("/demo/create", post(handle_create_demo))
}
