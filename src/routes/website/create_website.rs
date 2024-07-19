use axum::{extract::State, http::StatusCode, Json};
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use crate::{models::website::{NewWebsite, Website}, schema::websites, utilities::{app_error::AppError, db_connection::get_db_connection}};

use super::RequestWebsite;

pub async fn create_website(
  State(pool): State<Pool>,
  Json(request_website): Json<RequestWebsite>
) -> Result<Json<Website>, AppError> {
  let db = get_db_connection(pool).await?;

  let new_website = NewWebsite {
    name: request_website.name,
    agency_id: request_website.agency_id,
    domain: request_website.domain,
    status: request_website.status
  };

  let created_website = db.interact(move |conn| {
    diesel::insert_into(websites::table).values(new_website).returning(Website::as_returning()).get_result(conn)
  }).await.map_err(|error| {
    eprintln!("Error connecting to database {:?}", error);
    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "There was an error, please try again later")
  })?.map_err(|error| {
    eprintln!("Error loading agencies {:?}", error);
    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "There was an error, please try again later")
  })?;

  Ok(Json(created_website))
}