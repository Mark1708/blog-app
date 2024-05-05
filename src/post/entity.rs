use actix_web::{
    body::{BoxBody},
    HttpRequest, HttpResponse, Responder,
    http::header::ContentType
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(ToSchema, FromRow, Deserialize, Serialize)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub category: Option<String>,
    pub content: String,
    pub published: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Responder for Post {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(res_body)
    }
}