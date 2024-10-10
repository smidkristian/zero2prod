use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{config::get_config, startup::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_config().expect("Failed to read config");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.application_port))
        .expect("Failed to bind address");

    let db_connection_pool = PgPool::connect(&config.database.get_connection_string())
        .await
        .expect("Failed to connect to Postgres");

    run(listener, db_connection_pool)?.await
}
