use axum::{extract::{Path, State}, http::StatusCode, Json};
use deadpool_diesel::postgres::Pool;
use diesel::{associations::HasTable, prelude::*};
use crate::{models::website::Website, schema::websites::dsl, utilities::{app_error::AppError, db_connection::get_db_connection}};

pub async fn get_websites(
  State(pool): State<Pool>
) -> Result<Json<Vec<Website>>, AppError>{
  let db = get_db_connection(pool).await?;

  let response_website = db.interact(|conn| {
    dsl::websites.select(Website::as_select()).load(conn)
  }).await.map_err(|error| {
    eprintln!("Error connecting to database {:?}", error);
    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "There was an error, please try again later")
  })?.map_err(|error| {
    eprintln!("Error loading agencies {:?}", error);
    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "There was an error, please try again later")
  })?;

  Ok(Json(response_website))
}

pub async fn get_websites_by_agency_id(
  State(pool): State<Pool>,
  Path(id): Path<i32>
) -> Result<Json<Vec<Website>>, AppError>{
  let db = get_db_connection(pool).await?;

  let response_websites = db.interact(move |conn| {
    dsl::websites::table().filter(dsl::agency_id.eq(id)).select(Website::as_select()).load(conn)
  }).await.map_err(|error| {
    eprintln!("Error connecting to database {:?}", error);
    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "There was an error, please try again later")
  })?.map_err(|error| {
    eprintln!("Error loading agencies {:?}", error);
    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "There was an error, please try again later")
  })?;

  Ok(Json(response_websites))
}