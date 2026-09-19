// Lightweight summary shape for draft-bucket (bucket 2) data, mirroring how
// domain/Transform/TransformSummary.ts relates to TransformDefinition in the
// bucket-3 domain folder. Nothing currently fetches a *list* of drafts
// specifically — there's no frontend call to the backend's
// get_transform_draft/get_transform_drafts GET routes today, since the
// Creator's "browse everything I own" list (transforms-sidebar.tsx) is
// actually served by the bucket-3 workspace endpoint (a pre-existing quirk,
// not addressed here) — so this type currently exists only for return-type
// consistency / future-proofing alongside TransformDraftDefinition.
export interface TransformDraftSummary {
  transform_id: number;
  name: string | null;
  description: string | null;
  icon?: string;
  kind: "primitive" | "composite";
}
