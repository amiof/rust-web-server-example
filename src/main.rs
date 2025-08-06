mod run_app;

use axume_project::connect_to_db;
use run_app::run;

#[tokio::main]

async fn main() {
    run().await
}
