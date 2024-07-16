use axum::extract::FromRef;
use deadpool_diesel::postgres::Pool;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub pool: Pool,
}
