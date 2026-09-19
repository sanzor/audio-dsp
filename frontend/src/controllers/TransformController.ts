// controllers/TransformController.ts
//
// Bucket 3 (published) controller. Intentionally thin/near-empty: bucket 3
// is read-only from the frontend today — there are zero frontend-triggered
// bucket-3 mutations/actions to orchestrate. Reads already go straight
// through hooks/transforms/queries.ts's hooks, called directly from
// components (e.g. composite/composite-palette.tsx's usePublishedTransforms(),
// code-editor.tsx's/composite-canvas.tsx's useGetTransformDefinition()) —
// there's nothing for a controller to add on top of a plain query hook when
// there's no imperative action, error-recovery flow, or cross-store
// side-effect to coordinate.
//
// This file exists (rather than being omitted) per explicit product
// decision to keep bucket 2 (controllers/TransformDraftController.ts) and
// bucket 3 as two separate controllers regardless of how much either
// currently does. If/when a frontend-triggered bucket-3 action appears
// (there is none today — do not invent speculative ones), it belongs here.
export {};
