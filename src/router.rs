use axum::{routing::get, Router};

use crate::{
    app_state::AppState,
    routes::{agency::get_agencies::get_agencies, check_health::ping::ping},
};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/ping", get(ping))
        .route("/agencies", get(get_agencies))
        .with_state(app_state)
}
