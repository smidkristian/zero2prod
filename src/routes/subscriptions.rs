use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use mongodb::{bson::doc, results::InsertOneResult, Database};

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(name = "Saving new subscriber details in the database", skip(form, db))]
async fn insert_subscriber(
    form: &FormData,
    db: &Database,
) -> Result<InsertOneResult, mongodb::error::Error> {
    let collection = db.collection("subscriptions");

    let new_subscriber = doc! {
        "email": &form.email,
        "name": &form.name,
        "timestamp": Utc::now().to_string()
    };

    collection.insert_one(new_subscriber, None).await
}

#[post("/subscriptions")]
#[tracing::instrument(
    name ="Adding a new subscriber",
    skip(form, db),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
async fn subscribe(form: web::Form<FormData>, db: web::Data<Database>) -> HttpResponse {
    match insert_subscriber(&form, &db).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_e) => HttpResponse::InternalServerError().finish(),
    }
}
