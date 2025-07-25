use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

use crate::models::UserClassroomRole;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: usize,
    pub iat: usize,
    pub classroom_roles: Vec<UserClassroomRole>,
}

pub fn create_jwt(user_id: i32, classroom_roles: Vec<UserClassroomRole>) -> Result<String, jsonwebtoken::errors::Error> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("valid timestamp")
        .timestamp();
    
    let claims = Claims {
        sub: user_id as i32,
        exp: expiration as usize,
        iat: Utc::now().timestamp() as usize,
        classroom_roles: classroom_roles.clone(),
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &validation,
    )?;
    
    Ok(token_data.claims)
}

pub fn set_auth_cookie(token: &str) -> tower_cookies::Cookie<'static> {
    tower_cookies::Cookie::build(("auth_token", token.to_owned()))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(tower_cookies::cookie::SameSite::Strict)
        .max_age(tower_cookies::cookie::time::Duration::days(7))
        .build()
}

pub async fn check_auth(
    cookies: tower_cookies::Cookies,
) -> Result<Claims, (axum::http::StatusCode, String)> {

    let cookie = cookies.get("auth_token").ok_or((
        axum::http::StatusCode::UNAUTHORIZED,
        "Authentication required".to_string(),
    ))?;
    
    let token = cookie.value();
    let claims = validate_jwt(token).map_err(|_| {
        (axum::http::StatusCode::UNAUTHORIZED, "Invalid token".to_string())
    })?;
    
    Ok(claims)
}