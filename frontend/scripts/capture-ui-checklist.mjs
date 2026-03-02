import { mkdir } from "node:fs/promises";
import { existsSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

import { chromium } from "playwright";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const baseUrl = process.env.HPLY_BASE_URL ?? "http://localhost:3000";
const apiBaseUrl = process.env.HPLY_API_BASE_URL ?? "http://localhost:4000/api";
const captureDir = path.resolve(__dirname, "..", "..", "docs", "ui-captures");

async function launchBrowser() {
  const edgeCandidates = [
    "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe",
    "C:\\Program Files\\Microsoft\\Edge\\Application\\msedge.exe",
  ];

  for (const executablePath of edgeCandidates) {
    if (existsSync(executablePath)) {
      return chromium.launch({ executablePath, headless: true });
    }
  }

  try {
    return await chromium.launch({ channel: "msedge", headless: true });
  } catch {
    return chromium.launch({ headless: true });
  }
}

async function createSession(username, password) {
  const response = await fetch(`${apiBaseUrl}/auth/login`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ username, password }),
  });

  if (!response.ok) {
    throw new Error(`Echec login API pour ${username}: ${response.status}`);
  }

  return response.json();
}

async function createAuthenticatedContext(browser, username, password, viewport) {
  const session = await createSession(username, password);
  const context = await browser.newContext(viewport);

  await context.addInitScript((payload) => {
    window.localStorage.setItem("hporterly_token_v1", payload.token);
    window.localStorage.setItem("hporterly_user_v1", JSON.stringify(payload.user));
  }, session);

  return context;
}

async function captureLogin(browser) {
  const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
  const page = await context.newPage();

  await page.goto(`${baseUrl}/login`, { waitUntil: "networkidle" });
  await page.getByRole("heading", { name: "HPorterly" }).waitFor();
  await page.screenshot({
    path: path.join(captureDir, "login-desktop.png"),
    fullPage: true,
  });

  await context.close();
}

async function captureDashboard(browser) {
  const context = await createAuthenticatedContext(browser, "admin", "password123", {
    viewport: { width: 1440, height: 980 },
  });
  const page = await context.newPage();

  await page.goto(`${baseUrl}/dashboard`, { waitUntil: "networkidle" });
  await page.getByRole("heading", { name: "Tableau de bord" }).waitFor();
  await page.locator("table").first().waitFor();
  await page.screenshot({
    path: path.join(captureDir, "dashboard-table-desktop.png"),
    fullPage: true,
  });

  await context.close();
}

async function capturePorterMobile(browser) {
  const context = await createAuthenticatedContext(browser, "jean.martin", "password123", {
    viewport: { width: 390, height: 844 },
    isMobile: true,
    hasTouch: true,
  });
  const page = await context.newPage();

  await page.goto(`${baseUrl}/porter`, { waitUntil: "networkidle" });
  await page.getByRole("heading", { name: "Queue partagee" }).waitFor();
  await page.screenshot({
    path: path.join(captureDir, "porter-mobile.png"),
    fullPage: true,
  });

  await context.close();
}

async function captureAdmin(browser) {
  const context = await createAuthenticatedContext(browser, "admin", "password123", {
    viewport: { width: 1440, height: 980 },
  });
  const page = await context.newPage();

  await page.goto(`${baseUrl}/admin/porters`, { waitUntil: "networkidle" });
  await page.getByRole("heading", { name: "Administration" }).waitFor();
  await page.getByRole("heading", { name: "Gestion brancardiers" }).waitFor();
  await page.screenshot({
    path: path.join(captureDir, "admin-porters-desktop.png"),
    fullPage: true,
  });

  await context.close();
}

async function main() {
  await mkdir(captureDir, { recursive: true });
  const browser = await launchBrowser();

  try {
    await captureLogin(browser);
    await captureDashboard(browser);
    await capturePorterMobile(browser);
    await captureAdmin(browser);
    console.log(`Captures generees dans ${captureDir}`);
  } finally {
    await browser.close();
  }
}

main().catch((error) => {
  console.error("Echec capture UI checklist:", error);
  process.exit(1);
});
