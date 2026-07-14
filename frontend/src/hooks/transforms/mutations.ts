import { useMutation, useQueryClient } from "@tanstack/react-query";
import {
  apiCreateTransform,
  apiUpdateTransform,
  apiAddPort,
  apiDeletePort,
  apiCreateCompileTicket,
  type CreateTransformParams,
  type CreateCompileTicketParams,
} from "@/Services/TransformService";
import { QUERY_KEYS } from "@/constants/queryKeys";
import type { TransformPort } from "@/domain/Transform/Transform";

export function useRequestCompileTransform() {
  return useMutation({
    mutationFn: (params: CreateCompileTicketParams) => apiCreateCompileTicket(params),
  });
}

export function useCreateTransform() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (params: CreateTransformParams) => apiCreateTransform(params),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: QUERY_KEYS.transforms.all() });
    },
  });
}

export interface SaveTransformParams {
  name: string;
  description?: string;
  // Ports as the user left them in the panel; we diff against originalPorts.
  ports: LocalPort[];
  originalPorts: TransformPort[];
}

// A port that may not yet exist on the server (no port_id) or is being kept.
export interface LocalPort {
  port_id?: number; // undefined = newly added, not yet persisted
  name: string;
  direction: "input" | "output";
  port_order: number;
}

export function useSaveTransform(transformId: number) {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: async ({ name, description, ports, originalPorts }: SaveTransformParams) => {
      await apiUpdateTransform(transformId, { name, description });

      const keptIds = new Set(ports.filter((p) => p.port_id != null).map((p) => p.port_id!));
      const toDelete = originalPorts.filter((p) => !keptIds.has(p.port_id));
      const toAdd = ports.filter((p) => p.port_id == null);

      await Promise.all(toDelete.map((p) => apiDeletePort(p.port_id)));
      await Promise.all(
        toAdd.map((p) =>
          apiAddPort(transformId, {
            name: p.name,
            direction: p.direction,
            port_order: p.port_order,
          })
        )
      );
    },
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: QUERY_KEYS.transforms.all() });
      qc.invalidateQueries({ queryKey: QUERY_KEYS.transforms.byId(transformId) });
    },
  });
}
