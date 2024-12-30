use std::net::TcpListener;

use actix_web::rt::spawn;
use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use once_cell::sync::Lazy;
use serde::Deserialize;
use uuid::Uuid;
use zero2prod::{
    config::{self, DatabaseSettings},
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::test]
async fn health_check() {
    let test_app = spawn_app().await;
    // we need to bring in `reqwest` to perform HTTP requests against our application
    let client = reqwest::Client::new();

    // act
    let response = client
        .get(format!("{}/health_check", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request");

    // assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[derive(Deserialize)]
struct Subscriber {
    email: String,
    name: String,
}

#[actix_web::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // arrange
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    // act
    let body = "name=mc%20kenzie&email=katie_mc_kenzie%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", &test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // assert
    assert_eq!(200, response.status().as_u16());

    let filter = doc! { "email": "katie_mc_kenzie@gmail.com" };
    let saved: Subscriber = test_app
        .db
        .collection("subscriptions")
        .find_one(filter, None)
        .await
        .expect("Failed to fetch saved subscription")
        .expect("No document found");

    assert_eq!(saved.email, "katie_mc_kenzie@gmail.com");
    assert_eq!(saved.name, "mc kenzie");
}

#[actix_web::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // arrange
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=mc%20kenzie", "missing the email"),
        ("email=katie_mc_kenzie%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // act
        let response = client
            .post(format!("{}/subscriptions", &test_app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}

// ensuring that the tracing is initialized only once
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    // we cannot assign the output of `get_subscriber` to a variable based on the value of `TEST_LOG`
    // because the sink is part of the type returned by `get_subscriber`, therefore they are not the
    // same type.. we could work around it, but this is the most straight-forward way of moving forward
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestApp {
    address: String,
    db: Database,
}

// launch our application in the background
async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let mut config = config::get_config().expect("Failed to get config");
    // randomize database name so that each test has its own database
    config.database.database_name = Uuid::new_v4().to_string();

    let database = configure_database(&config.database).await;

    let server =
        zero2prod::startup::run(listener, database.clone()).expect("Failed to bind address");

    spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db: database,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> Database {
    let client_options = ClientOptions::parse(&config.uri)
        .await
        .expect("Failed to parse client options");

    let client = Client::with_options(client_options).expect("Failed to create database client");

    client.database(&config.database_name)
}
