use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;
use tower_cookies::{Cookie, Cookies};

use crate::models::{UserClassroomRole, ClassroomRole};
use crate::dto::{AuthResponse, LoginRequest, RegisterRequest};
use crate::middlewares::jwt::{create_jwt, set_auth_cookie};

pub async fn register(
    State(pool): State<PgPool>, 
    cookies: Cookies, 
    Json(request): Json<RegisterRequest>
) -> Result<Json<AuthResponse>, (StatusCode, String)> {

    let existing_user = sqlx::query!(
        "SELECT email FROM users WHERE email = $1",
        request.email
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if existing_user.is_some() {
        return Err((StatusCode::BAD_REQUEST, "Email already exists".to_string()));
    }

    let hashed_password = hash(request.password, DEFAULT_COST)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let classroom_roles: Vec<UserClassroomRole> = Vec::new();
    
    let user = sqlx::query!(
        "INSERT INTO users (name, email, password_hash) 
         VALUES ($1, $2, $3)
         RETURNING id, name, email",
        request.name,
        request.email,
        hashed_password
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let token = create_jwt(user.id, classroom_roles)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    cookies.add(set_auth_cookie(&token));

    Ok(Json(AuthResponse {
        user_id: user.id,
        email: user.email,
    }))
}

pub async fn login(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Json(request): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {

    let user = sqlx::query!(
        "SELECT id, name, email, password_hash FROM users WHERE email = $1",
        request.email
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let user = match user {
        Some(user) => user,
        None => return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())),
    };
    
    let password_matches = verify(request.password, &user.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    if !password_matches {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    let classroom_roles = sqlx::query_as!(UserClassroomRole, 
        "SELECT user_id, classroom_id, role AS \"role: ClassroomRole\", joined_at FROM user_classroom_roles WHERE user_id = $1", 
        user.id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let token = create_jwt(user.id, classroom_roles)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    cookies.add(set_auth_cookie(&token));

    Ok(Json(AuthResponse {
        user_id: user.id,
        email: user.email,
    }))
}

pub async fn logout(cookies: Cookies) -> impl IntoResponse {
    let mut cookie = Cookie::new("auth_token", "");
    cookie.set_path("/");
    cookie.set_max_age(tower_cookies::cookie::time::Duration::seconds(0));
    cookies.add(cookie);
    (StatusCode::OK, "Logged out successfully".to_string())
}