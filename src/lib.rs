use deadpool_diesel::postgres::{Manager, Pool};
use dotenvy::dotenv;
use std::env;

pub mod handlers;
pub mod models;
pub mod routes;
pub mod state;

pub type DBPool = Pool;
pub async fn connect_to_db() -> Pool {
    dotenv().ok();
    let url_db = env::var("DATABASE_URL").expect("database url not found");
    let manager = Manager::new(url_db, deadpool_diesel::Runtime::Tokio1);
    let pool = Pool::builder(manager)
        .build()
        .expect("fail to create db pool ");
    println!("create db pool");
    pool
}
