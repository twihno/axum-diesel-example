use std::sync::Arc;

use bb8::PooledConnection;
use diesel_async::{pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection};

use crate::{
    api::error::{replace_with_500_page, ErrorPage},
    db::DbPool,
};

#[derive(Debug)]
pub struct AppState {
    pub db_pool: DbPool,
    // pub name: &'static str,
}

pub type AppStateFull = Arc<AppState>;

pub async fn get_db_connection_from_app_state(
    app_state: &AppState,
) -> Result<PooledConnection<AsyncDieselConnectionManager<AsyncPgConnection>>, ErrorPage> {
    app_state.db_pool.get().await.map_err(replace_with_500_page)
}
