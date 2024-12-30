use mongodb::{options::ClientOptions, Client};
use std::net::TcpListener;
use zero2prod::{
    config::get_config,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_config().expect("Failed to read config");

    let listener = TcpListener::bind(format!(
        "{}:{}",
        config.application.host, config.application.port
    ))
    .expect("Failed to bind address");

    let db_options = ClientOptions::parse(&config.database.uri)
        .await
        .expect("Failed to parse database client options");
    let db_client = Client::with_options(db_options).expect("Failed to create database client");
    let database = db_client.database(&config.database.database_name);

    run(listener, database)?.await
}
