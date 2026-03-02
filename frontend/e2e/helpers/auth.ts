import type { APIRequestContext, Page } from "playwright/test";
import { expect } from "playwright/test";

const API_BASE_URL = process.env.E2E_API_BASE_URL ?? "http://localhost:4000/api";

type LoginResponse = {
  token: string;
  user: {
    id: string;
    role: string;
  };
};

export async function apiLogin(
  request: APIRequestContext,
  credentials: { username: string; password: string },
): Promise<LoginResponse> {
  const response = await request.post(`${API_BASE_URL}/auth/login`, {
    data: credentials,
  });
  expect(response.ok()).toBeTruthy();
  return (await response.json()) as LoginResponse;
}

export async function createPendingTicket(
  request: APIRequestContext,
  token: string,
  patientName: string,
): Promise<string> {
  const response = await request.post(`${API_BASE_URL}/tickets`, {
    headers: {
      Authorization: `Bearer ${token}`,
    },
    data: {
      patient_name: patientName,
      origin: "Urgences",
      destination: "Radiologie",
      transport_type: "PATIENT",
      transport_subtype: "TP-BRANC",
      mode: "Brancard",
      notes: "Ticket e2e",
    },
  });

  expect(response.ok()).toBeTruthy();
  const body = (await response.json()) as { ticket: { id: string } };
  return body.ticket.id;
}

export async function sendMessageToAdmins(
  request: APIRequestContext,
  token: string,
  message: string,
): Promise<void> {
  const response = await request.post(`${API_BASE_URL}/notifications/send-message`, {
    headers: {
      Authorization: `Bearer ${token}`,
    },
    data: {
      message,
      channel: "general",
      target_type: "all_admins",
    },
  });

  expect(response.ok()).toBeTruthy();
}

export async function loginFromUi(
  page: Page,
  credentials: { username: string; password: string },
  expectedPathPrefix: string,
): Promise<void> {
  await page.goto("/login");
  await page.getByLabel("Nom d'utilisateur").fill(credentials.username);
  await page.getByLabel("Mot de passe").fill(credentials.password);
  await page.getByRole("button", { name: "Se connecter" }).click();
  await expect(page).toHaveURL(new RegExp(`${expectedPathPrefix}`));
}
