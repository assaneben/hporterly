import { expect, test } from "playwright/test";

import { apiLogin, loginFromUi, sendMessageToAdmins } from "./helpers/auth";

test("notifications: reception puis marquage comme lu", async ({ page, request }) => {
  const message = `E2E notification ${Date.now()}`;

  const porterSession = await apiLogin(request, {
    username: "jean.martin",
    password: "password123",
  });

  await sendMessageToAdmins(request, porterSession.token, message);

  await loginFromUi(
    page,
    {
      username: "admin",
      password: "password123",
    },
    "/dashboard",
  );

  await page.getByTestId("notification-bell").click();
  await expect(page.getByTestId("notification-panel")).toBeVisible();
  await expect(page.getByText(message)).toBeVisible();

  await page.getByTestId("notification-mark-all").click();
  await expect(page.getByTestId("notification-unread-badge")).toHaveCount(0);
});
