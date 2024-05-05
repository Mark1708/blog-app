use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

/// Check app health
///
/// Check application status.
#[utoipa::path(
    tag = "App",
    responses(
        (status = 200, description = "Application is running"),
    )
)]
#[get("/health")]
async fn health_handler() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "HEALTHY"}))
}