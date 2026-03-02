import { expect, test } from "playwright/test";

import { apiLogin, createPendingTicket, loginFromUi } from "./helpers/auth";

test("dashboard: assignation puis reassignation d'un ticket", async ({ page, request }) => {
  const patientName = `E2E Assign ${Date.now()}`;
  const adminSession = await apiLogin(request, {
    username: "admin",
    password: "password123",
  });

  await createPendingTicket(request, adminSession.token, patientName);

  await loginFromUi(
    page,
    {
      username: "admin",
      password: "password123",
    },
    "/dashboard",
  );

  await page.getByLabel("Recherche").fill(patientName);

  const ticketRow = page.locator("tbody tr").filter({ hasText: patientName }).first();
  await expect(ticketRow).toBeVisible();
  await ticketRow.getByTestId("table-assign-button").click();

  await expect(page.getByRole("dialog", { name: "Assigner un brancardier" })).toBeVisible();
  await page.getByTestId("assign-option").first().click();
  await expect(page.getByText("Ticket assigne")).toBeVisible();

  await expect(page.getByTestId("quick-reassign-button")).toBeVisible();
  await page.getByTestId("quick-reassign-button").click();
  await expect(page.getByRole("dialog", { name: "Reassigner le ticket" })).toBeVisible();
  await page.getByTestId("reassign-option").first().click();
  await expect(page.getByText("Ticket reassign")).toBeVisible();
});
