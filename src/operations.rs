use actix_web::{
    body::BoxBody,
    http::header::ContentType,
    HttpRequest, HttpResponse, Responder
};
use serde::{Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]
pub struct OperationsDto {
    pub msg: &'static str
}

impl Responder for OperationsDto {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(res_body)
    }
}