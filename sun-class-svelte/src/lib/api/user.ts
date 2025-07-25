import { baseUrl } from "./types";

export const get_user = new Request(baseUrl + "/user", {
  method: "GET",
  credentials: "include",
});

export const get_user_role = (class_id: string) => new Request(baseUrl + "/class/" + class_id + "/role", {
  method: "GET",
  credentials: "include",
});

export const profile_request = new Request(baseUrl + "/user/profile", {
  method: "GET",
  credentials: "include",
});

export const edit_profile_request = (name: string, avatar_url: string) => new Request(baseUrl + "/user/profile/edit", {
  method: "POST",
  body: JSON.stringify({ name, avatar_url }),
  credentials: "include",
  headers: {
    "Content-Type": "application/json;charset=UTF-8",
  },
});

export const get_classes = new Request(baseUrl + "/user/classes", {
  method: "GET",
  credentials: "include",
});
