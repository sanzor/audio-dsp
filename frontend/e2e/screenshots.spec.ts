import { test } from "@playwright/test";

// Automated UI capture for design review — run via `pnpm screenshots`.
// Output lands in e2e/screenshots/ (gitignored) for the dag-ui-expert /
// marketing-ui-expert consultant lens (see agents/consultants/) to review
// instead of anyone taking screenshots by hand.
// See agents/skills/design/capture-ui-screenshots.md for the workflow.

test.describe("authenticated surfaces", () => {
  test("dashboard (editor)", async ({ page }) => {
    await page.goto("/dashboard");
    await page.waitForLoadState("networkidle");
    await page.screenshot({ path: "e2e/screenshots/dashboard.png", fullPage: true });
  });

  test("creator", async ({ page }) => {
    await page.goto("/creator");
    await page.waitForLoadState("networkidle");
    await page.screenshot({ path: "e2e/screenshots/creator.png", fullPage: true });
  });
});

test.describe("unauthenticated surfaces", () => {
  test.use({ storageState: { cookies: [], origins: [] } });

  test("login", async ({ page }) => {
    await page.goto("/login");
    await page.waitForLoadState("networkidle");
    await page.screenshot({ path: "e2e/screenshots/login.png", fullPage: true });
  });
});
