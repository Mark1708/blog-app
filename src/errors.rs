use actix_web::{
    HttpResponse, ResponseError,
    body::BoxBody,
    http::{
        header::ContentType,
        StatusCode
    }
};
use uuid::Uuid;

use derive_more::{Display, Error};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, ToSchema, Display, Error, Serialize)]
pub enum BaseError {
    #[display(fmt = "{} with ID: {} not found", entity_name, id)]
    NotFoundError { id: Uuid, entity_name: &'static str},
    #[display(fmt = "Internal server error: {}", msg)]
    InternalServerError {msg: &'static str},
    #[display(fmt = "Bad request: {}", msg)]
    BadRequest{msg: &'static str}
}

impl ResponseError for BaseError {
    fn status_code(&self) -> StatusCode {
        match *self {
            BaseError::NotFoundError { .. } => StatusCode::BAD_REQUEST,
            BaseError::InternalServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            BaseError::BadRequest { .. } => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .content_type(ContentType::json())
            .json(serde_json::json!(self))
    }
}
