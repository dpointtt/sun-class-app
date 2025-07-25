export type Assignment = {
    id: string,
    title: string,
    due_date: string
};

export type ClassUser = {
    name: string,
    role: string,
};

export type Class = {
    id: string,
    title: string,
    teacher: string,
    upcoming_assignment: string,
};

export type ClassList = {
    enrolled_classes: Class[],
    teaching_classes: Class[],
};

export type ClassData = {
    id: string,
    title: string,
    description: string,
    teacher: string,
    assignments: Assignment[],
    users: ClassUser[],
    join_code: string,
};

export type AssignmentMaterial = {
    id: string,
    file_link: string,
    file_name: string,
};

export type AssignmentSubmission = {
    id: string,
    user_id: string,
    submitted_at: string,
    points_earned?: number,
};

export type AssignmentFile = {
    id: string | null,
    file_name: string,
    content_type: string,
    file_type: string,
};

export type AssignmentData = {
    id: string,
    class_id: string,
    title: string,
    class_title: string,
    description: string,
    due_date: string,
    points: number,
    materials: AssignmentFile[],
    submission_files: AssignmentFile[],
    is_submitted: boolean,
    grade: number | null,
};

export type Submission = {
  id: string;
  studentName: string;
  submittedAt: string | null; // ISO date or null if not submitted
  grade: number | null;
}

export interface SubmissionListItem {
  id: number;
  assignment_id: number;
  assignment_title: string;
  student_name: string;
  submitted_at: string | null;
  is_graded: boolean;
  grade: number | null;
}

export interface SubmissionResponse {
  id: number;
  assignment_id: number;
  assignment_title: string;
  assignment_points: number;
  student_name: string;
  submitted_at: string | null;
  is_graded: boolean;
  grade: number | null;
  graded_at: string | null;
  grader_name: string | null;
  files: AssignmentFile[];
}

export interface CreatedAssignmentResponse {
    id: number
}