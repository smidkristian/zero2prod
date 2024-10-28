use std::net::TcpListener;
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use zero2prod::{config::get_config, startup::run, telemetry::{get_subscriber, init_subscriber}};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_config().expect("Failed to read config");

    let listener = TcpListener::bind(format!("{}:{}", config.application.host, config.application.port))
        .expect("Failed to bind address");

    let db_connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect_lazy(config.database.get_connection_string().expose_secret())
        .expect("Failed to connect to Postgres");

    run(listener, db_connection_pool)?.await
}
