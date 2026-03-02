import { expect, test } from "playwright/test";

import { apiLogin, createPendingTicket, loginFromUi } from "./helpers/auth";

test("porter: accepter mission, avancer puis suspendre", async ({ page, request }) => {
  const patientName = `E2E Porter ${Date.now()}`;

  const adminSession = await apiLogin(request, {
    username: "admin",
    password: "password123",
  });

  await createPendingTicket(request, adminSession.token, patientName);

  await loginFromUi(
    page,
    {
      username: "jean.martin",
      password: "password123",
    },
    "/porter",
  );

  await page.getByPlaceholder("Rechercher patient, destination, ticket...").fill(patientName);
  const missionCard = page.locator("article").filter({ hasText: patientName }).first();
  await expect(missionCard).toBeVisible();
  await missionCard.getByTestId("queue-card-action").click();
  await expect(page.getByText("Mission assignee")).toBeVisible();

  await page.getByRole("link", { name: "Active" }).click();
  await expect(page).toHaveURL(/\/porter\/active/);
  await expect(page.getByText("Mission active")).toBeVisible();
  await expect(page.getByText(patientName)).toBeVisible();

  await page.getByTestId("mission-advance-button").click();
  await expect(page.getByText("Mission mise a jour")).toBeVisible();

  await page.getByTestId("mission-suspend-button").click();
  await expect(page.getByText("Mission mise a jour")).toBeVisible();
  await expect(page.getByText("Suspendu")).toBeVisible();
});
