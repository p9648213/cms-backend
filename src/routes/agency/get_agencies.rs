use axum::{extract::State, http::StatusCode, Json};
use deadpool_diesel::postgres::Pool;

use crate::{models::agencies::Agency, utilities::app_error::AppError};

pub async fn get_agencies(
  pool: State<Pool>
) -> Result<Json<Agency>, AppError> {
  let connection = pool.get().await.map_err(|error| {
    eprintln!("Error coonecting to database: {:?}", error);
    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "There was an error, please try again later")
  })?;

  // let res = connection.interact(|conn| {

  // });

  todo!()
}