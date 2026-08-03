import { test, expect } from "@playwright/test";
import fs from "fs";

test("build a graph and drag connections", async ({ page }) => {
  page.on("console", (m) => console.log("PAGE:", m.text()));
  page.on("pageerror", (e) => console.log("PAGEERROR:", e.message));
  await page.goto("/dashboard");
  await page.waitForLoadState("networkidle");

  await page.getByText("Regression Pack").click();
  await page.waitForTimeout(300);

  // Right click track -> Create Region Set
  await page.getByText("Low E Pulse").click({ button: "right" });
  await page.waitForTimeout(200);
  await page.getByText("Create Region Set", { exact: true }).click();
  await page.waitForTimeout(200);
  await page.getByPlaceholder("Enter track name").fill("RS1");
  await page.getByRole("button", { name: "Submit" }).click();
  await page.waitForTimeout(500);

  await page.screenshot({ path: "e2e/screenshots/tmp-1-after-regionset.png", fullPage: true });

  // Click on the new region set to select it (use last() in case earlier runs left stray ones)
  await page.getByText("RS1").last().click();
  await page.waitForTimeout(300);

  // Right click region set -> Create Region
  await page.getByText("RS1").last().click({ button: "right" });
  await page.waitForTimeout(200);
  await page.getByText("Create Region", { exact: true }).click();
  await page.waitForTimeout(200);
  await page.getByPlaceholder("Enter region name").fill("R1");
  await page.getByRole("button", { name: "Save" }).click();
  await page.waitForTimeout(500);

  await page.screenshot({ path: "e2e/screenshots/tmp-2-after-region.png", fullPage: true });

  // Expand RS1 in the sidebar to reveal R1 as a tree item, then select + right-click it there
  await page.getByText("RS1", { exact: true }).last().click();
  await page.waitForTimeout(300);
  await page.screenshot({ path: "e2e/screenshots/tmp-2b-rs1-expanded.png", fullPage: true });

  const sidebarR1 = page.locator(".tree-node", { hasText: "R1" }).last();
  await sidebarR1.click();
  await page.waitForTimeout(300);
  await sidebarR1.click({ button: "right" });
  await page.waitForTimeout(200);
  await page.getByText("Create Graph", { exact: true }).click();
  await page.waitForTimeout(200);
  await page.getByPlaceholder("Enter region name").fill("G1");
  await page.getByRole("button", { name: "Save" }).click();
  await page.waitForTimeout(500);

  // Select the region again to make it active (creating the graph may not auto-select)
  await sidebarR1.click();
  await page.waitForTimeout(500);
  await page.screenshot({ path: "e2e/screenshots/tmp-4-graph-created.png", fullPage: true });

  // Drag a transform ("Gain") from the Store panel onto the canvas to create a default node.
  const gainItem = page.getByText("Gain", { exact: true });
  const canvasArea = page.locator(".canvas-area");
  await canvasArea.evaluate((el) => {
    el.addEventListener("dragover", () => console.log("NATIVE DRAGOVER"));
    el.addEventListener("drop", () => console.log("NATIVE DROP"));
  });
  await gainItem.evaluate((el) => {
    el.addEventListener("dragstart", () => console.log("NATIVE DRAGSTART"));
  });
  const dataTransfer = await page.evaluateHandle(() => new DataTransfer());
  await gainItem.dispatchEvent("dragstart", { dataTransfer });
  const canvasBox = await canvasArea.boundingBox();
  await canvasArea.dispatchEvent("dragover", {
    dataTransfer,
    clientX: (canvasBox?.x ?? 0) + 250,
    clientY: (canvasBox?.y ?? 0) + 150,
  });
  await canvasArea.dispatchEvent("drop", {
    dataTransfer,
    clientX: (canvasBox?.x ?? 0) + 250,
    clientY: (canvasBox?.y ?? 0) + 150,
  });
  await gainItem.dispatchEvent("dragend", { dataTransfer });
  await page.waitForTimeout(500);
  await page.screenshot({ path: "e2e/screenshots/tmp-5-transform-dropped.png", fullPage: true });
});
