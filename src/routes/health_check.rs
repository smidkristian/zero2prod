use actix_web::{get, HttpResponse};

#[get("/health_check")]
async fn healthcheck() -> HttpResponse {
    HttpResponse::Ok().finish()
}
