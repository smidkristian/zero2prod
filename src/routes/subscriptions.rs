use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, db_connection_pool)
)]
async fn insert_subscriber(form: &FormData, db_connection_pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // we use `get_ref` to get an immutable reference to the `PgPool` wrapped by `web::Data`
    .execute(db_connection_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}

#[post("/subscriptions")]
#[tracing::instrument(
    name ="Adding a new subscriber",
    skip(form, db_connection_pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
async fn subscribe(
    form: web::Form<FormData>,
    db_connection_pool: web::Data<PgPool>,
) -> HttpResponse {
    match insert_subscriber(&form, &db_connection_pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_e) => HttpResponse::InternalServerError().finish()
    }
}
