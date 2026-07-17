import { create } from "zustand";
import type { TransformDefinition, TransformSummary } from "@/domain/Transform/TransformDefinition";

interface TransformState {
  summaries: Map<number, TransformSummary>;
  definitions: Map<number, TransformDefinition>;
  setSummaries: (transforms: TransformSummary[]) => void;
  upsertDefinition: (transform: TransformDefinition) => void;
  upsertDefinitions: (transforms: TransformDefinition[]) => void;
  clear: () => void;
}

export const useTransformStore = create<TransformState>()((set) => ({
  summaries: new Map(),
  definitions: new Map(),
  setSummaries: (transforms) =>
    set((state) => {
      const summaries = new Map<number, TransformSummary>(
        Array.from(state.definitions.values()).map((transform) => [
          transform.transform_id,
          {
            transform_id: transform.transform_id,
            name: transform.name,
            description: transform.description,
            icon: transform.icon,
          },
        ])
      );

      for (const transform of transforms) {
        summaries.set(transform.transform_id, transform);
      }

      return { summaries };
    }),
  upsertDefinition: (transform) =>
    set((state) => {
      const definitions = new Map(state.definitions);
      definitions.set(transform.transform_id, transform);

      const summaries = new Map(state.summaries);
      summaries.set(transform.transform_id, {
        transform_id: transform.transform_id,
        name: transform.name,
        description: transform.description,
        icon: transform.icon,
      });

      return { definitions, summaries };
    }),
  upsertDefinitions: (transforms) =>
    set((state) => {
      const definitions = new Map(state.definitions);
      const summaries = new Map(state.summaries);

      for (const transform of transforms) {
        definitions.set(transform.transform_id, transform);
        summaries.set(transform.transform_id, {
          transform_id: transform.transform_id,
          name: transform.name,
          description: transform.description,
          icon: transform.icon,
        });
      }

      return { definitions, summaries };
    }),
  clear: () => set({ summaries: new Map(), definitions: new Map() }),
}));
