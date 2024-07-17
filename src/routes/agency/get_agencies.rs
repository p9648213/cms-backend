use axum::response::Json;
use axum::{extract::State, http::StatusCode};
use deadpool_diesel::postgres::Pool;
use crate::utilities::db_connection::get_connection;
use crate::{models::agencies::Agency, utilities::app_error::AppError};
use crate::schema::agencies::dsl::*;
use diesel::prelude::*;

pub async fn get_agencies(
  State(pool): State<Pool>
) -> Result<Json<Vec<Agency>>, AppError> {
  let db = get_connection(pool).await?;

  let result = db.interact(|conn| {
     agencies.select(Agency::as_select()).load(conn)
  }).await.map_err(|error| {
    eprintln!("Error connecting to db {:?}", error);
    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "There was an error, please try again later")
  })?.map_err(|error| {
    eprintln!("Error loading agencies {:?}", error);
    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "There was an error, please try again later")
  })?;

  Ok(Json(result))
}