use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use mongodb::Database;
use tracing_actix_web::TracingLogger;

use crate::routes::{healthcheck, subscribe};

pub fn run(listener: TcpListener, db: Database) -> Result<Server, std::io::Error> {
    let db = web::Data::new(db);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(healthcheck)
            .service(subscribe)
            .app_data(db.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
