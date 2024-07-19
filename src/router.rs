use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    app_state::AppState,
    routes::{
        agency::{create_agency::create_agency, get_agencies::get_agencies},
        check_health::ping::ping,
        website::{
            create_website::create_website,
            get_websites::{get_websites, get_websites_by_agency_id},
        },
    },
};

pub fn create_router(app_state: AppState) -> Router {
    let check_health_route = Router::new().route("/ping", get(ping));

    let agency_route = Router::new()
        .route("/agencies", get(get_agencies))
        .route("/agencies", post(create_agency));

    let website_routes = Router::new()
        .route("/websites", get(get_websites))
        .route("/websites", post(create_website))
        .route("/websites/agency/:id", get(get_websites_by_agency_id));

    Router::new()
        .merge(check_health_route)
        .merge(agency_route)
        .merge(website_routes)
        .with_state(app_state)
}
