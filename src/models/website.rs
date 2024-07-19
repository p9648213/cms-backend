use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::websites)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Website {
    pub id: i32,
    pub agency_id: Option<i32>,
    pub name: Option<String>,
    pub domain: Option<String>,
    pub status: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::websites)]
pub struct NewWebsite {
    pub name: String,
    pub agency_id: i32,
    pub domain: String,
    pub status: String,
}
