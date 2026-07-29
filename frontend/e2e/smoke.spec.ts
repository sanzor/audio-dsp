import { test, expect } from "@playwright/test";

// Baseline reachability checks for the two authenticated surfaces
// (see agents/architecture.md — Editor at /dashboard, Creator at /creator).
// Deliberately URL/landmark-based rather than deep-selector-based: the app
// has no data-testid convention yet, so assertions here should stay
// conservative until qa-agent adds stable test hooks alongside real coverage.

test("dashboard (editor surface) loads for an authenticated user", async ({ page }) => {
  await page.goto("/dashboard");
  await expect(page).toHaveURL(/\/dashboard/);
  await expect(page.getByText(/something went wrong/i)).toHaveCount(0);
});

test("creator surface loads for an authenticated user", async ({ page }) => {
  await page.goto("/creator");
  await expect(page).toHaveURL(/\/creator/);
  await expect(page.getByText(/something went wrong/i)).toHaveCount(0);
});
