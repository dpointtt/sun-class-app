import { baseUrl } from "./types";

export const create_class = (title: string, description: string) =>
  new Request(baseUrl + "/class/create", {
    method: "POST",
    body: JSON.stringify({ title, description }),
    credentials: "include",
    headers: {
      "Content-Type": "application/json;charset=UTF-8",
    },
  });

export const load_class_info = (id: string) =>
  new Request(baseUrl + "/class/" + id, {
    method: "GET",
    credentials: "include",
  });

export const create_assignment = (
  classId: string,
  title: string,
  description: string,
  due_date: string,
  points: number
) =>
  new Request(baseUrl + "/class/" + classId + "/create-assignment", {
    method: "POST",
    body: JSON.stringify({ title, description, due_date, points }),
    credentials: "include",
    headers: {
      "Content-Type": "application/json;charset=UTF-8",
    },
  });

export const join_class = (join_code: string) =>
  new Request(baseUrl + "/class/join", {
    method: "POST",
    body: JSON.stringify({ join_code }),
    credentials: "include",
    headers: {
      "Content-Type": "application/json;charset=UTF-8",
    },
  });

export const load_assignment_info = (classId: string, assignmentId: string) =>
  new Request(baseUrl + "/class/" + classId + "/assignment/" + assignmentId, {
    method: "GET",
    credentials: "include",
  });

export const upload_assignment_files = (
  classId: string,
  assignmentId: string,
  files: FormData
) => {
  return new Request(
    baseUrl + "/class/" + classId + "/assignment/" + assignmentId + "/submit",
    {
      method: "POST",
      body: files,
      credentials: "include",
    }
  );
};

export const delete_assignment_files = (
  classId: string,
  assignmentId: string,
  fileId: string
) => {
  return new Request(
    baseUrl +
      "/class/" +
      classId +
      "/assignment/" +
      assignmentId +
      "/delete-file/" +
      fileId,
    {
      method: "DELETE",
      credentials: "include",
    }
  );
};

export const cancel_submission = (classId: string, assignmentId: string) => {
  return new Request(
    baseUrl +
      "/class/" +
      classId +
      "/assignment/" +
      assignmentId +
      "/cancel-submission",
    {
      method: "DELETE",
      credentials: "include",
    }
  );
};

// List all submissions for a classroom (teacher only)
export const list_submissions = (classId: string) =>
  new Request(baseUrl + "/class/" + classId + "/submissions", {
    method: "GET",
    credentials: "include",
  });

// Get detailed information about a specific submission
export const get_submission = (classId: string, submissionId: string) =>
  new Request(baseUrl + "/class/" + classId + "/submissions/" + submissionId, {
    method: "GET",
    credentials: "include",
  });

// Grade a submission (teacher only)
export const grade_submission = (
  classId: string,
  submissionId: string,
  grade: number | null,
) =>
  new Request(
    baseUrl + "/class/" + classId + "/submissions/" + submissionId + "/grade",
    {
      method: "POST",
      body: JSON.stringify({
        grade,
      }),
      credentials: "include",
      headers: {
        "Content-Type": "application/json;charset=UTF-8",
      },
    }
  );

export const cancel_grade = (classId: string, submissionId: string) =>
  new Request(baseUrl + "/class/" + classId + "/submissions/" + submissionId + "/cancel-grade", {
    method: "PUT",
    credentials: "include",
  });

// Download a submission file
export const download_submission_file = (classId: string, fileId: string) =>
  new Request(baseUrl + "/class/" + classId + "/download-submission-file/" + fileId, {
    method: "GET",
    credentials: "include",
  });

  export const download_material_file = (classId: string, fileId: string) =>
  new Request(baseUrl + "/class/" + classId + "/download-material-file/" + fileId, {
    method: "GET",
    credentials: "include",
  });

  export const save_assignment_materials = (
  classId: string,
  assignmentId: string,
  files: FormData
) => {
  return new Request(
    baseUrl + "/class/" + classId + "/assignment/" + assignmentId + "/add-materials",
    {
      method: "POST",
      body: files,
      credentials: "include",
    }
  );
};