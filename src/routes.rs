use std::error::Error;
use std::env;
use axum::{
    http::{header, HeaderValue, Method}, routing::{delete, get, post, put}, Router
};
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;

use crate::db;
use crate::handlers;

pub async fn make_app() -> Result<Router, Box<dyn Error>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::create_pool(&database_url).await;

    sqlx::migrate!("./migrations")
    .run(&pool)
    .await
    .expect("Failed to run migrations");

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
            header::ORIGIN,
        ])
        .expose_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
        ])
        .allow_credentials(true);

    let app = Router::new()
        .route("/api/auth/register", post(handlers::auth::register))
        .route("/api/auth/login", post(handlers::auth::login))
        .route("/api/auth/logout", post(handlers::auth::logout))
        .route("/api/user/profile", get(handlers::user::profile))
        .route("/api/user/profile/edit", post(handlers::user::edit_profile))
        .route("/api/user", get(handlers::user::get_user))
        .route("/api/user/classes", get(handlers::user::list_classes))
        .route("/api/class/create", post(handlers::classroom::create_class))
        .route("/api/class/join", post(handlers::classroom::join_class))
        .route("/api/class/{id}", get(handlers::classroom::get_class))
        .route("/api/class/{id}/role", get(handlers::classroom::get_class_role))
        .route("/api/class/{id}/create-assignment", post(handlers::classroom::create_assignment))
        .route("/api/class/{c_id}/assignment/{a_id}", get(handlers::classroom::get_assignment))
        .route("/api/class/{c_id}/assignment/{a_id}/add-materials", post(handlers::classroom::add_assignment_materials))
        .route("/api/class/{c_id}/assignment/{a_id}/submit", post(handlers::classroom::save_submission_multipart_files))
        .route("/api/class/{c_id}/assignment/{a_id}/delete-file/{f_id}", delete(handlers::classroom::delete_assignment_file))
        .route("/api/class/{c_id}/assignment/{a_id}/cancel-submission", delete(handlers::classroom::cancel_submission))
        .route("/api/class/{c_id}/submissions", get(handlers::classroom::list_submissions))
        .route("/api/class/{c_id}/submissions/{s_id}", get(handlers::classroom::get_submission))
        .route("/api/class/{c_id}/submissions/{s_id}/grade", post(handlers::classroom::grade_submission))
        .route("/api/class/{c_id}/submissions/{s_id}/cancel-grade", put(handlers::classroom::cancel_grade))
        .route("/api/class/{c_id}/download-submission-file/{f_id}", get(handlers::classroom::download_submission_file))
        .route("/api/class/{c_id}/download-material-file/{f_id}", get(handlers::classroom::download_material_file))
        .layer(cors)
        .layer(CookieManagerLayer::new())
        .with_state(pool);

    Ok(app)
}