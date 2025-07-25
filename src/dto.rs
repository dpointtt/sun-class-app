use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub user_id: i32,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct EditProfileRequest {
    pub name: String,
    pub avatar_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserProfileResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateClassroomRequest {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClassroomListResponse {
    pub id: i32,
    pub title: String,
    pub teacher: String,
    pub upcoming_assignment: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ClassroomResponse {
    pub id: i32,
    pub title: String,
    pub teacher: String,
    pub description: String,
    pub assignments: Vec<AssignmentInfo>,
    pub users: Vec<ClassroomUser>,
    pub join_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClassroomUser {
    pub name: String,
    pub role: String,
}

#[derive(Serialize, Deserialize)]
pub struct AssignmentInfo {
    pub id: i32,
    pub title: String,
    pub due_date: String,
}

#[derive(Serialize, Deserialize)]
pub struct SubmissionInfo {
    pub id: i32,
    pub assignment_id: i32,
    pub assignment_title: String,
    pub student_name: String,
    pub submitted_at: Option<String>,
    pub is_graded: bool,
    pub grade: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct SubmissionResponse {
    pub id: i32,
    pub assignment_id: i32,
    pub assignment_title: String,
    pub assignment_points: i32,
    pub student_name: String,
    pub submitted_at: Option<String>,
    pub is_graded: bool,
    pub grade: Option<i32>,
    pub graded_at: Option<String>,
    pub grader_name: Option<String>,
    pub files: Vec<AssignmentFile>,
}

#[derive(Deserialize)]
pub struct GradeSubmissionRequest {
    pub grade: Option<i32>,
    // pub feedback: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AssignmentResponse {
    pub id: i32,
    pub class_id: i32,
    pub title: String,
    pub class_title: String,
    pub description: String,
    pub due_date: String,
    pub points: i32,
    pub materials: Vec<AssignmentFile>,
    pub submission_files: Vec<AssignmentFile>,
    pub is_submitted: bool,
    pub grade: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct AssignmentFile {
    pub id: i32,
    pub file_name: String,
    pub content_type: String,
    pub file_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserClassroomResponse {
    pub enrolled_classes: Vec<ClassroomListResponse>,
    pub teaching_classes: Vec<ClassroomListResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAssignmentRequest {
    pub title: String,
    pub description: String,
    pub due_date: Option<String>,
    pub points: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatedAssignmentResponse {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClassroomRoleResponse {
    pub is_teacher: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JoinClassroomRequest {
    pub join_code: String,
}