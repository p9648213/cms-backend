use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::agencies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Agency {
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub tax_code: Option<String>,
    pub invoice_address: Option<String>,
    pub invoice_email: Option<String>,
    pub invoice_company_name: Option<String>,
}
