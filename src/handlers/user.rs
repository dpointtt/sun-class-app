use axum::response::IntoResponse;
use axum::{Json, extract::State, http::StatusCode};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use tower_cookies::Cookies;

use crate::dto::{
    ClassroomListResponse, EditProfileRequest, UserClassroomResponse, UserProfileResponse,
};
use crate::middlewares::jwt::check_auth;

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
struct FUCKEDClassroom {
    id: i32,
    title: String,
    teacher: String,
    assignment_id: Option<i32>,
    assignment_title: Option<String>,
    due_date: Option<DateTime<Utc>>,
}

pub async fn get_user(
    State(pool): State<PgPool>,
    cookies: Cookies,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let claims = check_auth(cookies).await?;

    let user = sqlx::query!(
        "SELECT id, name, email FROM users WHERE id = $1",
        claims.sub
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let _user = match user {
        Some(user) => user,
        None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    };

    Ok(StatusCode::OK)
}

pub async fn profile(
    State(pool): State<PgPool>,
    cookies: Cookies,
) -> Result<Json<UserProfileResponse>, (StatusCode, String)> {
    let claims = check_auth(cookies).await?;

    let user = sqlx::query!(
        "SELECT id, name, email FROM users WHERE id = $1",
        claims.sub
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = match user {
        Some(user) => user,
        None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    };

    Ok(Json(UserProfileResponse {
        id: user.id,
        name: user.name,
        email: user.email,
    }))
}

pub async fn edit_profile(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Json(request): Json<EditProfileRequest>,
) -> Result<Json<UserProfileResponse>, (StatusCode, String)> {
    let claims = check_auth(cookies).await?;

    let user = sqlx::query!(
        "UPDATE users SET name = $1 WHERE id = $2 RETURNING id, name, email",
        request.name,
        claims.sub
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = match user {
        Some(user) => user,
        None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    };

    Ok(Json(UserProfileResponse {
        id: user.id,
        name: user.name,
        email: user.email,
    }))
}

pub async fn list_classes(
    State(pool): State<PgPool>,
    cookies: Cookies,
) -> Result<Json<UserClassroomResponse>, (StatusCode, String)> {
    let claims = check_auth(cookies).await?;

    let user = sqlx::query!("SELECT id FROM users WHERE id = $1", claims.sub)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let _user = match user {
        Some(user) => user,
        None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    };

    let enrolled_classrooms = sqlx::query_as!(
        FUCKEDClassroom,
        "SELECT 
        c.id,
        c.name AS title,
        u.name AS teacher,
        COALESCE(a.id, NULL) AS assignment_id,
        COALESCE(a.title, NULL) AS assignment_title,
        COALESCE(a.due_date, NULL) AS due_date
    FROM classrooms c
    JOIN users u ON c.creator_id = u.id
    JOIN user_classroom_roles uc ON uc.classroom_id = c.id
    LEFT JOIN LATERAL (
        SELECT id, title, due_date
        FROM assignments
        WHERE assignments.classroom_id = c.id AND due_date > NOW()
        ORDER BY due_date ASC
        LIMIT 1
    ) a ON true
    WHERE uc.user_id = $1 AND uc.role = 'student'",
        claims.sub
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let enrolled_classes: Vec<ClassroomListResponse> = enrolled_classrooms
        .into_iter()
        .map(|c| ClassroomListResponse {
            id: c.id,
            title: c.title,
            teacher: c.teacher,
            upcoming_assignment: c.assignment_title,
        })
        .collect();

    let teaching_classrooms = sqlx::query_as!(
        FUCKEDClassroom,
        r#"
    SELECT 
        c.id,
        c.name AS title,
        u.name AS teacher,
        COALESCE(a.id, NULL) AS assignment_id,
        COALESCE(a.title, NULL) AS assignment_title,
        COALESCE(a.due_date, NULL) AS due_date
    FROM classrooms c
    JOIN users u ON c.creator_id = u.id
    LEFT JOIN LATERAL (
        SELECT id, title, due_date
        FROM assignments
        WHERE assignments.classroom_id = c.id
        ORDER BY due_date ASC
        LIMIT 1
    ) a ON true
    WHERE c.creator_id = $1
    "#,
        claims.sub
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let teaching_classes: Vec<ClassroomListResponse> = teaching_classrooms
        .into_iter()
        .map(|c| ClassroomListResponse {
            id: c.id,
            title: c.title,
            teacher: c.teacher,
            upcoming_assignment: c.assignment_title,
        })
        .collect();

    Ok(Json(UserClassroomResponse {
        enrolled_classes,
        teaching_classes,
    }))
}
