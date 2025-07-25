use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::Type;

#[derive(Serialize, Deserialize, FromRow, Debug)]
#[allow(dead_code)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "classroom_role", rename_all = "lowercase")]
pub enum ClassroomRole {
    Creator,
    Teacher,
    Student,
}

impl From<String> for ClassroomRole {
    fn from(role: String) -> Self {
        match role.as_str() {
            "creator" => ClassroomRole::Creator,
            "teacher" => ClassroomRole::Teacher,
            "student" => ClassroomRole::Student,
            _ => panic!("Invalid role"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct UserClassroomRole {
    pub user_id: i32,
    pub classroom_id: i32,
    pub role: ClassroomRole,
    pub joined_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Submission {
    pub id: i32,
    pub assignment_id: i32,
    pub user_id: i32,
    pub file_drive_id: String,
    pub file_name: String,
    pub grade: Option<i32>,
    pub feedback: Option<String>,
    pub submitted_at: DateTime<Utc>,
    pub graded_at: Option<DateTime<Utc>>,
    pub graded_by: Option<i32>,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Classroom {
    pub id: i32,
    pub id_base64: String,
    pub name: String,
    pub description: String,
    pub join_code: String,
    pub creator_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Assignment {
    pub id: i32,
    pub id_base64: String,
    pub classroom_id: i32,
    pub title: String,
    pub description: String,
    pub due_date: DateTime<Utc>,
    pub points: i32,
    pub created_by: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}