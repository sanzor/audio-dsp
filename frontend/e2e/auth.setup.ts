import { test as setup, expect } from "@playwright/test";

// Matches database/seeds/users.sql — the "test@gmail.com" dev seed user,
// which already owns projects (see database/seeds/projects.sql) so login
// lands on /dashboard instead of /onboarding.
const SEED_EMAIL = "test@gmail.com";
const SEED_PASSWORD = "test";

const authFile = "./e2e/.auth/user.json";

setup("authenticate as seed dev user", async ({ page }) => {
  await page.goto("/login");

  await page.locator("#email").fill(SEED_EMAIL);
  await page.locator("#password").fill(SEED_PASSWORD);
  await page.getByRole("button", { name: /sign in/i }).click();

  await expect(page).toHaveURL(/\/dashboard/);

  await page.context().storageState({ path: authFile });
});
