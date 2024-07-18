use serde::Deserialize;

pub mod create_agency;
pub mod get_agencies;

#[derive(Deserialize)]
pub struct RequestAgency {
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
