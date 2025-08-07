use deadpool_diesel::postgres::{Manager, Pool};
use dotenvy::dotenv;
use std::env;

pub mod handlers;
pub mod models;
pub mod routes;
pub mod schema;
pub mod state;

pub type DBPool = Pool;
pub async fn connect_to_db() -> Pool {
    dotenv().ok();
    let url_db = env::var("DATABASE_URL").expect("database url not found");
    let manager = Manager::new(url_db, deadpool_diesel::Runtime::Tokio1);
    let pool = Pool::builder(manager).build().expect("fail to create db pool ");

    match pool.get().await {
        Ok(_) => {
            println!("✅ Successfully connected to the database!");
        }
        Err(e) => {
            panic!(
                "❌ Failed to get a connection from the pool. Is the database running and is the DATABASE_URL correct?\n   Error details: {:?}",
                e
            );
        }
    }
    pool
}
