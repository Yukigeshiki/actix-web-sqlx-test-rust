use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[allow(clippy::async_yields_async)]
#[tracing::instrument(
name = "Adding a new user",
skip(form, pool),
fields(
subscriber_email = % form.email,
subscriber_name = % form.name
)
)]
pub async fn add_user(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_user(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Saving new user details in the database", skip(form, pool))]
pub async fn insert_user(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO users (id, name, email)
    VALUES ($1, $2, $3)
            "#,
        Uuid::new_v4(),
        form.name,
        form.email
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
