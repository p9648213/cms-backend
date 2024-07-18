use backend::app_state::AppState;
use backend::run;
use deadpool_diesel::postgres::{Manager, Pool};
use dotenvy::dotenv;
use dotenvy_macro::dotenv;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let database_url = dotenv!("DATABASE_URL").to_owned();

    let manager = Manager::new(
        database_url,
        deadpool_diesel::Runtime::Tokio1,
    );
    
    let pool = Pool::builder(manager).build().expect("Error creating pool");

    let app_state = AppState {
        pool
    };

    run(app_state).await;
}

