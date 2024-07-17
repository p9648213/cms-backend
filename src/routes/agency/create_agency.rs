use axum::extract::State;
use deadpool_diesel::postgres::Pool;

use crate::utilities::{app_error::AppError, db_connection::get_connection};

pub async fn create_agency(
  State(pool): State<Pool>
) -> Result<(), AppError> {
  let db = get_connection(pool).await?;

  Ok(())
}