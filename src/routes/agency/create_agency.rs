use axum::Json;
use axum::{extract::State, http::StatusCode};
use deadpool_diesel::postgres::Pool;
use diesel::{RunQueryDsl, SelectableHelper};
use crate::models::agencies::{Agency, NewAgency};
use crate::schema::agencies;
use crate::utilities::{app_error::AppError, db_connection::get_connection};

use super::RequestAgency;

pub async fn create_agency(
  State(pool): State<Pool>,
  Json(request_agency): Json<RequestAgency>
) -> Result<Json<Agency>, AppError> {
  let db = get_connection(pool).await?;

  let new_agency = NewAgency {
    name: request_agency.name,
    email: request_agency.email,
    phone_number: request_agency.phone_number,
    address: request_agency.address,
    tax_code: request_agency.tax_code,
    identifier_id: request_agency.identifier_id,
    api_key: request_agency.api_key,
    invoice_address: request_agency.invoice_address,
    invoice_email: request_agency.invoice_email,
    invoice_company_name: request_agency.invoice_company_name
  };

  let created_agency = db.interact(|conn| {
    diesel::insert_into(agencies::table).values(new_agency).returning(Agency::as_returning()).get_result(conn)
  }).await.map_err(|error| {
    eprintln!("Error connecting to database {:?}", error);
    AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "There was an error, please try again later")
  })?.map_err(|error| {
    eprintln!("Error creating agencies {:?}", error);
    if error.to_string().contains("duplicate") {
      AppError::new(StatusCode::BAD_REQUEST, "Duplication error")
    }else{
      AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "There was an error, please try again later")
    }
  })?;

  Ok(Json(created_agency))
}