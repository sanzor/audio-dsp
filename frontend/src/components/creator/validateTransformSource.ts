const MAX_SOURCE_BYTES = 64 * 1024;

export interface SourceValidationResult {
  ok: boolean;
  issues: string[];
}

// Cheap, local, no-network checks that catch the most common ways a compile
// ticket would be wasted (empty source, forgetting the SDK export macro).
// This is a lint, not a compiler — it can't and doesn't try to catch real
// Rust errors, those only surface from the backend's actual cargo build.
export function validateTransformSource(code: string): SourceValidationResult {
  const issues: string[] = [];
  const trimmed = code.trim();

  if (!trimmed) {
    issues.push("Source is empty.");
    return { ok: false, issues };
  }

  if (new TextEncoder().encode(code).length > MAX_SOURCE_BYTES) {
    issues.push(`Source exceeds ${MAX_SOURCE_BYTES / 1024}KB limit.`);
  }

  if (!/export_transform!\s*\(/.test(code)) {
    issues.push("Missing transform_sdk::export_transform!(...) call — the compiled module won't export anything usable.");
  }

  if (!/fn\s+process\s*\(/.test(code)) {
    issues.push("No `fn process(...)` found — did you implement the Transform trait?");
  }

  if (!/fn\s+metadata\s*\(/.test(code)) {
    issues.push("No `fn metadata(...)` found — did you implement the Transform trait?");
  }

  return { ok: issues.length === 0, issues };
}
