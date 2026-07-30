// Client-side-only reachability check backing the composite canvas's
// on-node enable/disable coloring (agents/decisions/0005-composite-node-inspector.md,
// Phase 3). No equivalent exists in composite_validator.rs — that validator
// only does flat set-membership checks (every referenced node/port exists),
// never a graph traversal — and this deliberately doesn't grow into one:
// it's a UI affordance, not a save/publish-blocking rule.
//
// For each node, answers: "if this node were disabled right now, would any
// exposed-input -> exposed-output path that's currently connected stop being
// connected?" That's "load-bearing" (disabling breaks connectivity). A node
// that isn't on any currently-live path is "safe" to disable. A node already
// disabled is reported as "disabled" outright, no traversal needed.

import type { CompositeEdge } from "@/domain/Transform/CompositeGraphDefinition";

export type NodeDisableSafety = "safe" | "load-bearing" | "disabled";

export interface ComputeNodeDisableSafetyInput {
  nodeIds: number[];
  edges: Pick<CompositeEdge, "from_node_id" | "to_node_id">[];
  // Nodes carrying an exposed port whose underlying port direction is
  // "input" / "output" respectively — i.e. where the composite's own
  // boundary signal enters/exits the graph. A node can be in both sets
  // (a pass-through node exposing one port each way).
  exposedInputNodeIds: ReadonlySet<number>;
  exposedOutputNodeIds: ReadonlySet<number>;
  disabledNodes: ReadonlySet<number>;
}

// Forward-directed reachable (exposedInput, exposedOutput) node-id pairs,
// restricted to `active` nodes/edges, optionally with one extra node
// excluded (the node being hypothetically disabled).
function reachablePairs(
  active: ReadonlySet<number>,
  edges: ComputeNodeDisableSafetyInput["edges"],
  exposedInputNodeIds: ReadonlySet<number>,
  exposedOutputNodeIds: ReadonlySet<number>,
  exclude: number | null
): Set<string> {
  const adjacency = new Map<number, number[]>();
  for (const e of edges) {
    if (e.from_node_id === exclude || e.to_node_id === exclude) continue;
    if (!active.has(e.from_node_id) || !active.has(e.to_node_id)) continue;
    const list = adjacency.get(e.from_node_id) ?? [];
    list.push(e.to_node_id);
    adjacency.set(e.from_node_id, list);
  }

  const pairs = new Set<string>();
  for (const startId of exposedInputNodeIds) {
    if (startId === exclude || !active.has(startId)) continue;
    const visited = new Set<number>([startId]);
    const queue = [startId];
    while (queue.length > 0) {
      const cur = queue.shift()!;
      if (exposedOutputNodeIds.has(cur)) pairs.add(`${startId}->${cur}`);
      for (const next of adjacency.get(cur) ?? []) {
        if (!visited.has(next)) {
          visited.add(next);
          queue.push(next);
        }
      }
    }
  }
  return pairs;
}

export function computeNodeDisableSafety(input: ComputeNodeDisableSafetyInput): Map<number, NodeDisableSafety> {
  const { nodeIds, edges, exposedInputNodeIds, exposedOutputNodeIds, disabledNodes } = input;
  const result = new Map<number, NodeDisableSafety>();

  const enabledIds = new Set(nodeIds.filter((id) => !disabledNodes.has(id)));
  const baseline = reachablePairs(enabledIds, edges, exposedInputNodeIds, exposedOutputNodeIds, null);

  for (const id of nodeIds) {
    if (disabledNodes.has(id)) {
      result.set(id, "disabled");
      continue;
    }
    // Nothing is currently connected end-to-end — no node can be "load
    // bearing" for a path that doesn't exist yet.
    if (baseline.size === 0) {
      result.set(id, "safe");
      continue;
    }
    const withoutNode = reachablePairs(enabledIds, edges, exposedInputNodeIds, exposedOutputNodeIds, id);
    let loadBearing = false;
    for (const pair of baseline) {
      if (!withoutNode.has(pair)) {
        loadBearing = true;
        break;
      }
    }
    result.set(id, loadBearing ? "load-bearing" : "safe");
  }

  return result;
}
