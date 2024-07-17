use axum::{response::Html, routing::{get, post}, Router};

use crate::{
    app_state::AppState,
    routes::{agency::{create_agency::create_agency, get_agencies::get_agencies}, check_health::ping::ping},
};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/ping", get(ping))
        .route("/agencies", get(get_agencies))
        .route("/agencies", post(create_agency))
        .with_state(app_state)
}

async fn hello_world() -> Html<&'static str> {
    println!("Request received\nSending response.");
    Html("<h1>Hello World!</h1>")
}
