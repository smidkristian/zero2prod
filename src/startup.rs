use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::routes::{healthcheck, subscribe};

pub fn run(listener: TcpListener, db_connection_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_connection_pool = web::Data::new(db_connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(healthcheck)
            .service(subscribe)
            .app_data(db_connection_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
