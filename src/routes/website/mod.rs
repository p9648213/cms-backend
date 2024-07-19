use serde::Deserialize;

pub mod create_website;
pub mod get_websites;

#[derive(Deserialize)]
pub struct RequestWebsite {
    pub name: String,
    pub agency_id: i32,
    pub domain: String,
    pub status: String,
}
