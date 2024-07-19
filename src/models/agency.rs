use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::agencies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Agency {
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub tax_code: Option<String>,
    pub identifier_id: Option<i32>,
    pub api_key: Option<String>,
    pub invoice_address: Option<String>,
    pub invoice_email: Option<String>,
    pub invoice_company_name: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::agencies)]
pub struct NewAgency {
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub address: String,
    pub tax_code: String,
    pub identifier_id: i32,
    pub api_key: String,
    pub invoice_address: String,
    pub invoice_email: String,
    pub invoice_company_name: String,
}
