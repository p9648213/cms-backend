use backend::app_state::AppState;
use backend::run;
use deadpool_diesel::postgres::{Manager, Pool};
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = Manager::new(
        database_url,
        deadpool_diesel::Runtime::Tokio1,
    );
    
    let pool = Pool::builder(manager).build().expect("Error connecting to database");

    let app_state = AppState {
        pool
    };

    run(app_state).await;
}

