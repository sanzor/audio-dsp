// A transform's in-progress (bucket 2 — draft) state — mirrors
// backend/api/src/transform_drafts/dto/responses.rs's TransformDraftDto.
// Returned by create/save; nothing currently reads structured
// ports/params/graph off these responses (Save's mutation callers only use
// onSuccess to invalidate the query cache and refetch the full bucket-3
// definition separately via hooks/transforms/queries.ts), so this
// intentionally isn't normalized into the bucket-3 TransformDefinition
// (domain/Transform/TransformDefinition.ts) — the two buckets' wire shapes
// are independent and shouldn't be coerced into one type.
export interface TransformDraftDefinition {
  transform_id: number;
  source_code: string | null;
  metadata_json: string | null;
  name: string | null;
  description: string | null;
  kind: "primitive" | "composite";
  has_binary: boolean;
}
