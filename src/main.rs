use actix_cors::Cors;
use actix_web::{
    http::header,
    App, HttpServer, web,
    middleware::Logger
};
use dotenv::dotenv;
use sqlx::{
    Pool, Postgres,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub mod post;
mod health;
mod errors;
mod operations;
mod db;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Blog API definition",
        description = "Simple Rust API",
        contact(
            name = "Mark",
            email = "mark1708.work@gmail.com",
            url = "https://mark1708.github.io/"
        ),
        version = "1.0.0"
    ),
    paths(
        post::handler::get_list_handler,
        post::handler::get_handler,
        post::handler::create_handler,
        post::handler::update_handler,
        post::handler::delete_handler,
        health::health_handler,
    ),
    components(
        schemas(
            post::entity::Post, 
            post::dto::CreatePostDto, 
            post::dto::UpdatePostDto, 
            errors::BaseError,
            operations::OperationsDto
        ),
    ),
    tags(
        (name = "Post", description = "Post management API"),
        (name = "App", description = "Application management API")
    )
)]
pub struct ApiDocs;

impl ApiDocs {
    pub fn generate() -> String {
        ApiDocs::openapi().to_json().unwrap()
    }
}

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
        std::env::set_var("RUST_BACKTRACE", "1");
    }
    dotenv().ok();
    env_logger::init();

    let pool = db::create_pool().await;
    let openapi = ApiDocs::openapi();

    log::info!("Starting HTTP server: go to http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(create_app_state(pool.clone())))
            .service(
                web::scope("/api/v1")
                    .configure(post::handler::config)
            )
            .service(health::health_handler)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
            .wrap(get_cors())
            .wrap(Logger::default())
    })
        .workers(4)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

fn get_cors() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
        .allowed_headers(vec![
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
        ])
        .supports_credentials()
}

fn create_app_state(pool: Pool<Postgres>) -> AppState {
    AppState { db: pool }
}
