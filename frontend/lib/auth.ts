import { api } from "./api";
import { ROLE_HOME, STORAGE_KEYS } from "./constants";

export type AuthUser = {
  id: string;
  username: string;
  role: "demandeur" | "brancardier" | "regulateur" | "administrateur";
  first_name: string;
  last_name: string;
  email?: string;
  service?: string;
  porter_id?: string;
};

type LoginResponse = {
  token: string;
  user: AuthUser;
};

export async function login(username: string, password: string): Promise<LoginResponse> {
  const response = await api.post<LoginResponse>("/auth/login", { username, password });

  if (typeof window !== "undefined") {
    window.localStorage.setItem(STORAGE_KEYS.authToken, response.token);
    window.localStorage.setItem(STORAGE_KEYS.authUser, JSON.stringify(response.user));
  }

  return response;
}

export function logout(): void {
  if (typeof window === "undefined") {
    return;
  }

  window.localStorage.removeItem(STORAGE_KEYS.authToken);
  window.localStorage.removeItem(STORAGE_KEYS.authUser);
}

export function restoreAuth(): { token: string | null; user: AuthUser | null } {
  if (typeof window === "undefined") {
    return { token: null, user: null };
  }

  const token = window.localStorage.getItem(STORAGE_KEYS.authToken);
  const rawUser = window.localStorage.getItem(STORAGE_KEYS.authUser);

  if (!token || !rawUser) {
    return { token: null, user: null };
  }

  try {
    return { token, user: JSON.parse(rawUser) as AuthUser };
  } catch {
    logout();
    return { token: null, user: null };
  }
}

export function getRoleHome(role: string | undefined): string {
  if (!role) {
    return "/login";
  }

  return ROLE_HOME[role] ?? "/login";
}