use app_state::AppState;
use router::create_router;
mod router;
mod routes;
mod schema;
mod models;
pub mod app_state;
pub mod utilities;

pub async fn run(app_state: AppState) {
  let app = create_router(app_state);
  let listener = tokio::net::TcpListener::bind("0.0.0.0:8979").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}