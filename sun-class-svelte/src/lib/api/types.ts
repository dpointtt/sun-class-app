export const baseUrl = "http://localhost:8081/api";

export type AuthResponse = {
    user_id: number,
    email: string
}

export type UserProfileResponse = {
    id: number,
    name: string,
    email: string
}