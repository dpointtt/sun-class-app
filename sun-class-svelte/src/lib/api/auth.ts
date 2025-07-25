import { baseUrl, type AuthResponse } from "./types";

export const login = async (
  email: string,
  password: string
): Promise<AuthResponse> => {
  const response = await fetch(baseUrl + "/auth/login", {
    method: "POST",
    headers: {
      "content-type": "application/json;charset=UTF-8",
    },
    body: JSON.stringify({ email, password }),
    credentials: "include",
  })
    .then((response) => response.json())
    .catch((error) => {
      throw new Error("An error occurred during login.");
    });
  return response;
};

export const register = async (
  name: string,
  email: string,
  password: string
): Promise<AuthResponse> => {
  const response = await fetch(baseUrl + "/auth/register", {
    method: "POST",
    headers: {
      "content-type": "application/json;charset=UTF-8",
    },
    body: JSON.stringify({ name, email, password }),
    credentials: "include",
  })
    .then((response) => response.json())
    .catch((error) => {
      throw new Error("An error occurred during registration.");
    });
  return response;
};

export const logout = async (): Promise<void> => {
  await fetch(baseUrl + "/auth/logout", {
    method: "POST",
    headers: {
      "content-type": "application/json;charset=UTF-8",
    },
    credentials: "include",
  }).catch(console.error);
};
