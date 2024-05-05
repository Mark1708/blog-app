use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(IntoParams, Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(IntoParams, Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(ToSchema, Serialize, Deserialize, Debug)]
pub struct CreatePostDto {
    pub title: String,
    pub author: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
    pub content: String,
}

#[derive(ToSchema, Serialize, Deserialize, Debug)]
pub struct UpdatePostDto {
    pub title: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    pub content: Option<String>,
    pub published: Option<bool>,
}