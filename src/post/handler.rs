use crate::{
    AppState,
    post::{
        dto::{CreatePostDto, FilterOptions, UpdatePostDto},
        entity::Post
    },
    errors::BaseError,
    operations::OperationsDto
};
use actix_web::{
    delete, get, patch, post,
    web,
};
use chrono::prelude::*;
use uuid::Uuid;

/// List all post items
///
/// List all post posts from in-memory storage.
#[utoipa::path(
    context_path = "/api/v1/posts",
    tag = "Post",
    params(FilterOptions),
    responses(
        (
            status = 200,
            description = "List all posts successfully",
            body = [Post]
        )
    )
)]
#[get("")]
pub async fn get_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> Result<String, BaseError> {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        Post,
        "SELECT * FROM post ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
        .fetch_all(&data.db)
        .await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all post items";
        return Err(BaseError::InternalServerError {msg: message});
    }

    let posts = query_result.unwrap();

    return Ok(serde_json::to_string(&(*posts)).unwrap());
}

/// Create new post
///
/// Tries to create a new post item to in-memory storage or fails with 400 conflict if already exists.
#[utoipa::path(
    context_path = "/api/v1/posts",
    tag = "Post",
    request_body = CreatePostDto,
    responses(
        (status = 201, description = "Post item created successfully", body = Post),
        (status = 409, description = "Post already exists", body = BaseError)
    )
)]
#[post("")]
pub async fn create_handler(
    body: web::Json<CreatePostDto>,
    data: web::Data<AppState>,
) -> Result<Post, BaseError> {
    let query_result = sqlx::query_as!(
        Post,
        "INSERT INTO post (title, author, category, content, published) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        body.title.to_string(),
        body.author.to_string(),
        body.category.to_owned().unwrap_or("".to_string()),
        body.content.to_string(),
        body.published.unwrap_or(false)
    )
        .fetch_one(&data.db)
        .await;

    return match query_result {
        Ok(posts) => Ok(posts),
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return Err(BaseError::BadRequest {msg: "Post with that title already exists"});
            }
            return Err(BaseError::InternalServerError {msg: "Something bad happened while creating post"});
        }
    }
}

/// Get post item
///
/// Get post item from in-memory storage.
#[utoipa::path(
    context_path = "/api/v1/posts",
    tag = "Post",
    params(
        ("id" = string, Path, description = "Post database id")
    ),
    responses(
        (status = 200, description = "Post item created successfully", body = Post),
        (status = 404, description = "Post not found")
    )
)]
#[get("/{id}")]
pub async fn get_handler(
    path: web::Path<Uuid>,
    data: web::Data<AppState>,
) -> Result<Post, BaseError> {
    let post_id = path.into_inner();
    let query_result = sqlx::query_as!(Post, "SELECT * FROM post WHERE id = $1", post_id)
        .fetch_one(&data.db)
        .await;

    return match query_result {
        Ok(posts) => Ok(posts),
        Err(_) => {
            return Err(BaseError::NotFoundError {id: post_id, entity_name: "Post"});
        }
    }
}

/// Update post item by id
///
/// Update post item by given id. Return only status 200 on success or 404 if post is not found.
#[utoipa::path(
    context_path = "/api/v1/posts",
    tag = "Post",
    request_body = UpdatePostDto,
    params(
        ("id" = string, Path, description = "Post database id")
    ),
    responses(
        (status = 200, description = "Post updated successfully"),
        (status = 404, description = "Post not found")
    ),
)]
#[patch("/{id}")]
pub async fn update_handler(
    path: web::Path<Uuid>,
    body: web::Json<UpdatePostDto>,
    data: web::Data<AppState>,
) -> Result<Post, BaseError> {
    let post_id = path.into_inner();
    let query_result = sqlx::query_as!(Post, "SELECT * FROM post WHERE id = $1", post_id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        return Err(BaseError::NotFoundError {id: post_id, entity_name: "Post"});
    }

    let now = Utc::now();
    let posts = query_result.unwrap();

    let query_result = sqlx::query_as!(
        Post,
        "UPDATE post SET title = $1, author = $2, category = $3, content = $4, published = $5, updated_at = $6 WHERE id = $7 RETURNING *",
        body.title.to_owned().unwrap_or(posts.title),
        body.author.to_owned().unwrap_or(posts.author),
        body.category.to_owned().unwrap_or(posts.category.unwrap()),
        body.content.to_owned().unwrap_or(posts.content),
        body.published.unwrap_or(posts.published.unwrap()),
        now,
        post_id
    )
        .fetch_one(&data.db)
        .await;

    return match query_result {
        Ok(posts) => Ok(posts),
        Err(_) => {
            return Err(BaseError::InternalServerError {msg: "Something bad happened while updating post"});
        }
    }
}

/// Delete post item by id
///
/// Delete post item from in-memory storage by id. Returns either 200 success of 404 with postError if post is not found.
#[utoipa::path(
    context_path = "/api/v1/posts",
    tag = "Post",
    params(
        ("id" = string, Path, description = "Post database id")
    ),
    responses(
        (status = 200, description = "Post marked done successfully", body = OperationsDto),
        (status = 404, description = "Post not found", body = BaseError)
    )
)]
#[delete("/{id}")]
pub async fn delete_handler(
    path: web::Path<Uuid>,
    data: web::Data<AppState>,
) -> Result<OperationsDto, BaseError> {
    let post_id = path.into_inner();
    let rows_affected = sqlx::query!("DELETE FROM post WHERE id = $1", post_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        return Err(BaseError::NotFoundError {id: post_id, entity_name: "Post"});
    }

    Ok(OperationsDto{msg: "Successfully deleted"})
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/posts")
        .service(get_list_handler)
        .service(get_handler)
        .service(create_handler)
        .service(update_handler)
        .service(delete_handler);

    conf.service(scope);
}