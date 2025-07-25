use std::fs::File;
use std::io::{Read, Write};

use axum::body::Body;
use axum::extract::Path;
use axum::http::{HeaderMap, Response, header};
use axum::response::IntoResponse;
use axum::{Json, extract::Multipart, extract::State, http::StatusCode};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use sqlx::PgPool;
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::dto::{
    AssignmentFile, AssignmentInfo, AssignmentResponse, ClassroomResponse, ClassroomRoleResponse, ClassroomUser, CreateAssignmentRequest, CreateClassroomRequest, CreatedAssignmentResponse, GradeSubmissionRequest, JoinClassroomRequest, SubmissionInfo, SubmissionResponse
};
use crate::middlewares::jwt::check_auth;

pub async fn create_class(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Json(request): Json<CreateClassroomRequest>,
) -> Result<String, (StatusCode, String)> {
    let claims = check_auth(cookies).await?;

    let user = sqlx::query!("SELECT id FROM users WHERE id = $1", claims.sub)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let _user = match user {
        Some(user) => user,
        None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    };

    let unique_string_id = uuid::Uuid::new_v4().to_string();

    let classroom = sqlx::query!(
        "INSERT INTO classrooms (id_base64, name, description, join_code, creator_id)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id",
        unique_string_id,
        request.title,
        request.description,
        unique_string_id,
        claims.sub
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    sqlx::query!(
        "INSERT INTO user_classroom_roles (classroom_id, user_id, role)
        VALUES ($1, $2, 'teacher')",
        classroom.id,
        claims.sub
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok("Classroom created successfully".to_string())
}

pub async fn get_class(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Path(id): Path<i32>,
) -> Result<Json<ClassroomResponse>, (StatusCode, String)> {
    let claims = check_auth(cookies).await?;

    let user = sqlx::query!("SELECT id FROM users WHERE id = $1", claims.sub)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let _user = match user {
        Some(user) => user,
        None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    };

    let classroom = sqlx::query!(
        r#"SELECT c.id, c.join_code, c.name as title, c.description, u.name as teacher FROM classrooms c JOIN users u ON c.creator_id = u.id WHERE c.id = $1"#,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let classroom = match classroom {
        Some(classroom) => classroom,
        None => return Err((StatusCode::NOT_FOUND, "Classroom not found".to_string())),
    };

    let assignments = sqlx::query!(
        "SELECT id, title, due_date FROM assignments WHERE classroom_id = $1",
        classroom.id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let assignments: Vec<AssignmentInfo> = assignments
        .into_iter()
        .map(|a| AssignmentInfo {
            id: a.id,
            title: a.title,
            due_date: a
                .due_date
                .map_or_else(|| "No due date".to_string(), |date| date.to_string()),
        })
        .collect();

    let users = sqlx::query!(
        "SELECT u.name, uc.role as role FROM users u JOIN user_classroom_roles uc ON u.id = uc.user_id WHERE uc.classroom_id = $1",
        classroom.id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let users: Vec<ClassroomUser> = users
        .into_iter()
        .map(|u| ClassroomUser {
            name: u.name,
            role: u.role,
        })
        .collect();

    Ok(Json(ClassroomResponse {
        id: classroom.id,
        title: classroom.title,
        description: classroom
            .description
            .unwrap_or_else(|| "No description".to_string()),
        teacher: classroom.teacher,
        assignments: assignments,
        users: users,
        join_code: classroom.join_code,
    }))
}

pub async fn get_class_role(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Path(id): Path<i32>,
) -> Result<Json<ClassroomRoleResponse>, (StatusCode, String)> {
    let claims = check_auth(cookies).await?;

    let user = sqlx::query!("SELECT id FROM users WHERE id = $1", claims.sub)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let _user = match user {
        Some(user) => user,
        None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    };

    let classroom = sqlx::query!(
        r#"SELECT c.name FROM classrooms c JOIN users u ON c.creator_id = u.id WHERE c.id = $1"#,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let _classroom = match classroom {
        Some(classroom) => classroom,
        None => return Err((StatusCode::NOT_FOUND, "Classroom not found".to_string())),
    };

    let is_enrolled = sqlx::query!(
        "SELECT role FROM user_classroom_roles WHERE classroom_id = $1 AND user_id = $2",
        id,
        claims.sub
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let is_enrolled = match is_enrolled {
        Some(is_enrolled) => is_enrolled,
        None => return Err((StatusCode::NOT_FOUND, "User is not enrolled in this classroom".to_string())),
    };

    Ok(Json(ClassroomRoleResponse {
        is_teacher: is_enrolled.role == "teacher",
    }))
}

pub async fn join_class(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Json(request): Json<JoinClassroomRequest>,
) -> Result<String, (StatusCode, String)> {
    let claims = check_auth(cookies).await?;

    let user = sqlx::query!("SELECT id FROM users WHERE id = $1", claims.sub)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let _user = match user {
        Some(user) => user,
        None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    };

    let classroom = sqlx::query!(
        r#"SELECT c.id FROM classrooms c JOIN users u ON c.creator_id = u.id WHERE c.join_code = $1"#,
        request.join_code
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let classroom = match classroom {
        Some(classroom) => classroom,
        None => return Err((StatusCode::NOT_FOUND, "Classroom not found".to_string())),
    };

    let is_enrolled = sqlx::query!(
        "SELECT 1 as one FROM user_classroom_roles WHERE classroom_id = $1 AND user_id = $2",
        classroom.id,
        claims.sub
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .is_some();

    if is_enrolled {
        return Err((
            StatusCode::FORBIDDEN,
            "User is already enrolled in this classroom".to_string(),
        ));
    }

    sqlx::query!(
        "INSERT INTO user_classroom_roles (user_id, classroom_id, role) VALUES ($1, $2, $3)",
        claims.sub,
        classroom.id,
        "student"
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok("User was joined to classroom".to_string())
}

pub async fn create_assignment(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Path(id): Path<i32>,
    Json(request): Json<CreateAssignmentRequest>,
) -> Result<Json<CreatedAssignmentResponse>, (StatusCode, String)> {
    let claims = check_auth(cookies).await?;

    let user = sqlx::query!("SELECT id FROM users WHERE id = $1", claims.sub)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let _user = match user {
        Some(user) => user,
        None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    };

    let is_teacher = sqlx::query!(
        "SELECT 1 as one FROM user_classroom_roles WHERE classroom_id = $1 AND user_id = $2 AND role = 'teacher'",
        id,
        claims.sub
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .is_some();

    if !is_teacher {
        return Err((StatusCode::FORBIDDEN, "User is not a teacher".to_string()));
    }

    let parsed_due_date: Option<DateTime<Utc>> = match &request.due_date {
        Some(date_str) => {
            // "2025-06-07T14:30"
            let naive = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M").map_err(|e| {
                (
                    StatusCode::BAD_REQUEST,
                    format!("Invalid datetime format: {}", e),
                )
            })?;
            Some(TimeZone::from_utc_datetime(&Utc, &naive))
        }
        None => None,
    };

    let unique_string_id = uuid::Uuid::new_v4().to_string();

    let created_assignment = sqlx::query!(
        "INSERT INTO assignments (id_base64, classroom_id, title, description, due_date, points, created_by) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id",
        unique_string_id,
        id,
        request.title,
        request.description,
        parsed_due_date,
        request.points,
        claims.sub
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(CreatedAssignmentResponse {
        id: created_assignment.unwrap().id,
    }))
}

pub async fn get_assignment(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Path((c_id, a_id)): Path<(i32, i32)>,
) -> Result<Json<AssignmentResponse>, (StatusCode, String)> {
    let claims = check_auth(cookies).await?;

    let user = sqlx::query!("SELECT id FROM users WHERE id = $1", claims.sub)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let _user = match user {
        Some(user) => user,
        None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    };

    let is_enrolled = sqlx::query!(
        "SELECT 1 as one FROM user_classroom_roles WHERE classroom_id = $1 AND user_id = $2",
        c_id,
        claims.sub
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .is_some();

    if !is_enrolled {
        return Err((
            StatusCode::FORBIDDEN,
            "User is not enrolled in this classroom".to_string(),
        ));
    }

    let assignment = sqlx::query!(
        r#"
    SELECT 
        assignments.id,
        assignments.classroom_id,
        assignments.title,
        assignments.description,
        assignments.due_date,
        assignments.points,
        classrooms.name AS class_title
    FROM assignments
    JOIN classrooms ON assignments.classroom_id = classrooms.id
    WHERE assignments.classroom_id = $1 AND assignments.id = $2
    "#,
        c_id,
        a_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let assignment = match assignment {
        Some(assignment) => assignment,
        None => return Err((StatusCode::NOT_FOUND, "Assignment not found".to_string())),
    };

    let materials = sqlx::query!(
        "SELECT id, file_name, content_type, assignment_file_type FROM assignment_files WHERE assignment_id = $1 AND assignment_file_type = 'material'",
        a_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let materials: Vec<AssignmentFile> = materials
        .into_iter()
        .map(|m| AssignmentFile {
            id: m.id,
            file_name: m.file_name,
            content_type: m.content_type.unwrap_or("unknown".to_string()),
            file_type: m.assignment_file_type.unwrap_or("unknown".to_string()),
        })
        .collect();

    let submission_files = sqlx::query!(
        "SELECT id, file_name, content_type, assignment_file_type FROM assignment_files WHERE assignment_id = $1 AND user_id = $2 AND assignment_file_type = 'submission'",
        a_id,
        claims.sub
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let submission_files: Vec<AssignmentFile> = submission_files
        .into_iter()
        .map(|m| AssignmentFile {
            id: m.id,
            file_name: m.file_name,
            content_type: m.content_type.unwrap_or("unknown".to_string()),
            file_type: m.assignment_file_type.unwrap_or("unknown".to_string()),
        })
        .collect();

    let is_submitted = sqlx::query!(
        "SELECT 1 as one FROM submissions WHERE assignment_id = $1 AND user_id = $2 AND submitted_at IS NOT NULL",
        a_id,
        claims.sub
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .is_some();

    let grade: Option<i32>;

    if is_submitted {
        grade = sqlx::query!(
            "SELECT grade FROM submissions WHERE assignment_id = $1 AND user_id = $2",
            a_id,
            claims.sub
        )
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .and_then(|s| s.grade);
    } else {
        grade = None;
    };

    Ok(Json(AssignmentResponse {
        id: assignment.id,
        class_id: assignment.classroom_id,
        title: assignment.title,
        class_title: assignment.class_title,
        description: assignment
            .description
            .unwrap_or_else(|| "No description".to_string()),
        due_date: assignment
            .due_date
            .map_or_else(|| "No due date".to_string(), |date| date.to_string()),
        points: assignment.points.unwrap_or(0),
        materials: materials,
        submission_files: submission_files,
        is_submitted: is_submitted,
        grade: grade,
    }))
}

pub async fn save_submission_multipart_files(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Path((c_id, a_id)): Path<(i32, i32)>,
    mut multipart: Multipart,
) -> Result<String, (StatusCode, String)> {
    let claims = check_auth(cookies).await?;

    let user = sqlx::query!("SELECT id FROM users WHERE id = $1", claims.sub)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let _user = match user {
        Some(user) => user,
        None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    };

    let is_enrolled = sqlx::query!(
        "SELECT 1 as one FROM user_classroom_roles WHERE classroom_id = $1 AND user_id = $2",
        c_id,
        claims.sub
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .is_some();

    if !is_enrolled {
        return Err((
            StatusCode::FORBIDDEN,
            "User is not enrolled in this classroom".to_string(),
        ));
    }

    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let _name = field.name().unwrap().to_string();
        let filename = field.file_name().unwrap_or("unknown").to_string();
        let content_type = field
            .content_type()
            .map_or("unknown".to_string(), |ct| ct.to_string());
        let data = field.bytes().await.unwrap();

        let unique_name = format!("{}_{}", Uuid::new_v4(), filename);
        let path = format!("./uploads/{}", unique_name);

        let mut file =
            File::create(&path).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        file.write_all(&data)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        sqlx::query!(
            "INSERT INTO assignment_files (assignment_id, user_id, file_name, file_path, content_type, assignment_file_type)
             VALUES ($1, $2, $3, $4, $5, $6)",
            a_id,
            claims.sub,
            filename,
            path,
            content_type,
            "submission"
        )
        .execute(&pool)
        .await
        .map_err(|e| {
            let _ = std::fs::remove_file(&path);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

        println!("Length of `{}` is {} bytes", filename, data.len());
    }

    let is_resubmitting = sqlx::query!(
        "SELECT 1 as one FROM submissions WHERE assignment_id = $1 AND user_id = $2",
        a_id,
        claims.sub,
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .is_some();

    if !is_resubmitting {
        sqlx::query!(
            "INSERT INTO submissions (assignment_id, user_id, is_graded, submitted_at)
            VALUES ($1, $2, $3, NOW())",
            a_id,
            claims.sub,
            false
        )
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    } else {
        sqlx::query!(
            "UPDATE submissions SET submitted_at = NOW() WHERE assignment_id = $1 AND user_id = $2",
            a_id,
            claims.sub,
        )
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    Ok("Submission saved successfully".to_string())
}

pub async fn delete_assignment_file(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Path((c_id, a_id, f_id)): Path<(i32, i32, i32)>,
) -> impl IntoResponse {
    let claims = check_auth(cookies).await?;

    let user = sqlx::query!("SELECT id FROM users WHERE id = $1", claims.sub)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let _user = match user {
        Some(user) => user,
        None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    };

    let file = sqlx::query!(
        r#"
        SELECT id, file_path FROM assignment_files 
        WHERE id = $1 AND assignment_id = $2
        "#,
        f_id,
        a_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    if file.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            format!("File with ID {} not found", f_id),
        ));
    }

    sqlx::query!(
        r#"
        DELETE FROM assignment_files WHERE id = $1
        "#,
        f_id
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete file: {}", e),
        )
    })?;

    let _ = std::fs::remove_file(file.unwrap().file_path);

    Ok((StatusCode::OK, "File deleted successfully"))
}

pub async fn cancel_submission(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Path((c_id, a_id)): Path<(i32, i32)>,
) -> impl IntoResponse {
    let claims = check_auth(cookies).await?;

    let user = sqlx::query!("SELECT id FROM users WHERE id = $1", claims.sub)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let _user = match user {
        Some(user) => user,
        None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    };

    let submission = sqlx::query!(
        r#"
        SELECT id FROM submissions 
        WHERE assignment_id = $1 AND user_id = $2
        "#,
        a_id,
        claims.sub
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
    })?;

    if submission.is_none() {
        return Err((StatusCode::NOT_FOUND, "Submission not found".to_string()));
    }

    sqlx::query!(
        r#"
        DELETE FROM submissions WHERE assignment_id = $1 AND user_id = $2
        "#,
        a_id,
        claims.sub
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete submission: {}", e),
        )
    })?;

    Ok((StatusCode::OK, "Submission was canceled"))
}

pub async fn list_submissions(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Path(c_id): Path<i32>,
) -> Result<Json<Vec<SubmissionInfo>>, (StatusCode, String)> {
    let claims = check_auth(cookies).await?;

    let user = sqlx::query!("SELECT id FROM users WHERE id = $1", claims.sub)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let _user = match user {
        Some(user) => user,
        None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    };

    let is_teacher = sqlx::query!(
        "SELECT 1 as one FROM user_classroom_roles WHERE classroom_id = $1 AND user_id = $2 AND role = 'teacher'",
        c_id,
        claims.sub
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .is_some();

    if !is_teacher {
        return Err((
            StatusCode::FORBIDDEN,
            "Only teachers can view all submissions".to_string(),
        ));
    }

    let submissions = sqlx::query!(
        r#"
        SELECT 
            s.id,
            s.assignment_id,
            s.user_id,
            s.submitted_at,
            s.is_graded,
            s.grade,
            u.name as student_name,
            a.title as assignment_title
        FROM submissions s
        JOIN users u ON s.user_id = u.id
        JOIN assignments a ON s.assignment_id = a.id
        WHERE a.classroom_id = $1
        ORDER BY s.submitted_at DESC
        "#,
        c_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let submissions: Vec<SubmissionInfo> = submissions
        .into_iter()
        .map(|s| SubmissionInfo {
            id: s.id,
            assignment_id: s.assignment_id,
            assignment_title: s.assignment_title,
            student_name: s.student_name,
            submitted_at: s.submitted_at.map(|dt| dt.to_string()),
            is_graded: s.is_graded,
            grade: s.grade,
        })
        .collect();

    Ok(Json(submissions))
}

pub async fn get_submission(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Path((c_id, s_id)): Path<(i32, i32)>,
) -> Result<Json<SubmissionResponse>, (StatusCode, String)> {
    let claims = check_auth(cookies).await?;

    let user = sqlx::query!("SELECT id FROM users WHERE id = $1", claims.sub)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let _user = match user {
        Some(user) => user,
        None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    };

    let is_teacher = sqlx::query!(
        "SELECT 1 as one FROM user_classroom_roles WHERE classroom_id = $1 AND user_id = $2 AND role = 'teacher'",
        c_id,
        claims.sub
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .is_some();

    if !is_teacher {
        return Err((
            StatusCode::FORBIDDEN,
            "You can only view your own submissions".to_string(),
        ));
    }

    let submission = sqlx::query!(
        r#"
        SELECT 
            s.id,
            s.assignment_id,
            s.user_id,
            s.submitted_at,
            s.is_graded,
            s.grade,
            s.graded_at,
            s.graded_by,
            u.name as student_name,
            a.title as assignment_title,
            a.points as assignment_points,
            COALESCE(grader.name) as grader_name
        FROM submissions s
        JOIN users u ON s.user_id = u.id
        JOIN assignments a ON s.assignment_id = a.id
        LEFT JOIN users grader ON s.graded_by = grader.id
        WHERE s.id = $1 AND a.classroom_id = $2
        "#,
        s_id,
        c_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let submission = match submission {
        Some(submission) => submission,
        None => return Err((StatusCode::NOT_FOUND, "Submission not found".to_string())),
    };

    let files = sqlx::query!(
        "SELECT id, file_name, content_type FROM assignment_files WHERE assignment_id = $1 AND user_id = $2 AND assignment_file_type = 'submission'",
        submission.assignment_id,
        submission.user_id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let files: Vec<crate::dto::AssignmentFile> = files
        .into_iter()
        .map(|f| crate::dto::AssignmentFile {
            id: f.id,
            file_name: f.file_name,
            content_type: f.content_type.unwrap_or("unknown".to_string()),
            file_type: "submission".to_string(),
        })
        .collect();

    Ok(Json(SubmissionResponse {
        id: submission.id,
        assignment_id: submission.assignment_id,
        assignment_title: submission.assignment_title,
        assignment_points: submission.assignment_points.unwrap_or(0),
        student_name: submission.student_name,
        submitted_at: submission.submitted_at.map(|dt| dt.to_string()),
        is_graded: submission.is_graded,
        grade: submission.grade,
        graded_at: submission.graded_at.map(|dt| dt.to_string()),
        grader_name: submission.grader_name,
        files: files,
    }))
}

pub async fn grade_submission(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Path((c_id, s_id)): Path<(i32, i32)>,
    Json(request): Json<GradeSubmissionRequest>,
) -> Result<String, (StatusCode, String)> {
    let claims = check_auth(cookies).await?;

    let user = sqlx::query!("SELECT id FROM users WHERE id = $1", claims.sub)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let _user = match user {
        Some(user) => user,
        None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    };

    let is_teacher = sqlx::query!(
        "SELECT 1 as one FROM user_classroom_roles WHERE classroom_id = $1 AND user_id = $2 AND role = 'teacher'",
        c_id,
        claims.sub
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .is_some();

    if !is_teacher {
        return Err((
            StatusCode::FORBIDDEN,
            "Only teachers can grade submissions".to_string(),
        ));
    }

    let submission_exists = sqlx::query!(
        r#"
        SELECT 1 as one FROM submissions s
        JOIN assignments a ON s.assignment_id = a.id
        WHERE s.id = $1 AND a.classroom_id = $2
        "#,
        s_id,
        c_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .is_some();

    if !submission_exists {
        return Err((StatusCode::NOT_FOUND, "Submission not found".to_string()));
    }

    sqlx::query!(
        r#"
        UPDATE submissions 
        SET grade = $1, is_graded = true, graded_at = NOW(), graded_by = $2
        WHERE id = $3
        "#,
        request.grade,
        claims.sub,
        s_id
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok("Submission graded successfully".to_string())
}

pub async fn cancel_grade(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Path((c_id, s_id)): Path<(i32, i32)>,
) -> Result<String, (StatusCode, String)> {
    let claims = check_auth(cookies).await?;

    let user = sqlx::query!("SELECT id FROM users WHERE id = $1", claims.sub)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let _user = match user {
        Some(user) => user,
        None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    };

    let is_teacher = sqlx::query!(
        "SELECT 1 as one FROM user_classroom_roles WHERE classroom_id = $1 AND user_id = $2 AND role = 'teacher'",
        c_id,
        claims.sub
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .is_some();

    if !is_teacher {
        return Err((
            StatusCode::FORBIDDEN,
            "Only teachers can grade submissions".to_string(),
        ));
    }

    let submission_exists = sqlx::query!(
        r#"
        SELECT 1 as one FROM submissions s
        JOIN assignments a ON s.assignment_id = a.id
        WHERE s.id = $1 AND a.classroom_id = $2
        "#,
        s_id,
        c_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .is_some();

    if !submission_exists {
        return Err((StatusCode::NOT_FOUND, "Submission not found".to_string()));
    }

    sqlx::query!(
        r#"
        UPDATE submissions 
        SET grade = null, is_graded = false, graded_by = null, graded_at = null, submitted_at = null
        WHERE id = $1
        "#,
        s_id
    )
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok("Grade canceled successfully".to_string())
}

pub async fn download_submission_file(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Path((c_id, f_id)): Path<(i32, i32)>,
) -> Result<Response<Body>, (StatusCode, String)> {
    let claims = check_auth(cookies).await?;

    let user = sqlx::query!("SELECT id FROM users WHERE id = $1", claims.sub)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let _user = match user {
        Some(user) => user,
        None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    };

    let file = sqlx::query!(
        r#"
        SELECT 
            af.id,
            af.file_name,
            af.file_path,
            af.content_type,
            af.user_id,
            a.classroom_id
        FROM assignment_files af
        JOIN assignments a ON af.assignment_id = a.id
        WHERE af.id = $1 AND a.classroom_id = $2 AND af.assignment_file_type = 'submission'
        "#,
        f_id,
        c_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let file = match file {
        Some(file) => file,
        None => return Err((StatusCode::NOT_FOUND, "File not found".to_string())),
    };

    let is_teacher = sqlx::query!(
        "SELECT 1 as one FROM user_classroom_roles WHERE classroom_id = $1 AND user_id = $2 AND role = 'teacher'",
        c_id,
        claims.sub
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .is_some();

    if !is_teacher {
        return Err((
            StatusCode::FORBIDDEN,
            "You don't have permission to download this file".to_string(),
        ));
    }

    let mut file_handle = File::open(&file.file_path).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to open file: {}", e),
        )
    })?;

    let mut contents = Vec::new();
    file_handle.read_to_end(&mut contents).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read file: {}", e),
        )
    })?;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        file.content_type
            .unwrap_or("application/octet-stream".to_string())
            .parse()
            .unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{}\"", file.file_name)
            .parse()
            .unwrap(),
    );

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(contents))
        .unwrap();

    let (mut parts, body) = response.into_parts();
    parts.headers = headers;

    Ok(Response::from_parts(parts, body))
}

pub async fn download_material_file(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Path((c_id, f_id)): Path<(i32, i32)>,
) -> Result<Response<Body>, (StatusCode, String)> {
    let claims = check_auth(cookies).await?;

    let user = sqlx::query!("SELECT id FROM users WHERE id = $1", claims.sub)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let _user = match user {
        Some(user) => user,
        None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    };

    let file = sqlx::query!(
        r#"
        SELECT 
            af.id,
            af.file_name,
            af.file_path,
            af.content_type,
            af.user_id,
            a.classroom_id
        FROM assignment_files af
        JOIN assignments a ON af.assignment_id = a.id
        WHERE af.id = $1 AND a.classroom_id = $2 AND af.assignment_file_type = 'material'
        "#,
        f_id,
        c_id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let file = match file {
        Some(file) => file,
        None => return Err((StatusCode::NOT_FOUND, "File not found".to_string())),
    };

    let is_enrolled = sqlx::query!(
        "SELECT 1 as one FROM user_classroom_roles WHERE classroom_id = $1 AND user_id = $2",
        c_id,
        claims.sub
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .is_some();

    if !is_enrolled {
        return Err((
            StatusCode::FORBIDDEN,
            "You don't have permission to download this file".to_string(),
        ));
    }

    let mut file_handle = File::open(&file.file_path).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to open file: {}", e),
        )
    })?;

    let mut contents = Vec::new();
    file_handle.read_to_end(&mut contents).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read file: {}", e),
        )
    })?;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        file.content_type
            .unwrap_or("application/octet-stream".to_string())
            .parse()
            .unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{}\"", file.file_name)
            .parse()
            .unwrap(),
    );

    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(contents))
        .unwrap();

    let (mut parts, body) = response.into_parts();
    parts.headers = headers;

    Ok(Response::from_parts(parts, body))
}

pub async fn add_assignment_materials(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Path((c_id, a_id)): Path<(i32, i32)>,
    mut multipart: Multipart,
) -> Result<String, (StatusCode, String)> {
    let claims = check_auth(cookies).await?;

    let user = sqlx::query!("SELECT id FROM users WHERE id = $1", claims.sub)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let _user = match user {
        Some(user) => user,
        None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    };

    let is_teacher = sqlx::query!(
        "SELECT 1 as one FROM user_classroom_roles WHERE classroom_id = $1 AND user_id = $2 AND role='teacher'",
        c_id,
        claims.sub
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .is_some();

    if !is_teacher {
        return Err((
            StatusCode::FORBIDDEN,
            "User is not enrolled nor teacher in this classroom".to_string(),
        ));
    }

    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let _name = field.name().unwrap().to_string();
        let filename = field.file_name().unwrap_or("unknown").to_string();
        let content_type = field
            .content_type()
            .map_or("unknown".to_string(), |ct| ct.to_string());
        let data = field.bytes().await.unwrap();

        let unique_name = format!("{}_{}", Uuid::new_v4(), filename);
        let path = format!("./uploads/{}", unique_name);

        let mut file =
            File::create(&path).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        file.write_all(&data)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        sqlx::query!(
            "INSERT INTO assignment_files (assignment_id, user_id, file_name, file_path, content_type, assignment_file_type)
             VALUES ($1, $2, $3, $4, $5, $6)",
            a_id,
            claims.sub,
            filename,
            path,
            content_type,
            "material"
        )
        .execute(&pool)
        .await
        .map_err(|e| {
            let _ = std::fs::remove_file(&path);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

        println!("Length of `{}` is {} bytes", filename, data.len());
    }

    Ok("Assignment materials saved successfully".to_string())
}
