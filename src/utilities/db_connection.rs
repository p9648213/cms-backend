use axum::http::StatusCode;
use deadpool_diesel::postgres::{Object, Pool};
use super::app_error::AppError;

pub async fn get_db_connection(pool:Pool) -> Result<Object, AppError> {
  pool.get().await.map_err(|error| {
    eprintln!("Error connecting to pool: {:?}", error);
    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "There was an error, please try again later")
  })
} 