import { useCompositeCanvasStore, type CanvasLeafNode } from "@/Stores/CompositeCanvasStore";
import { useCreatorPreviewStore } from "@/Stores/CreatorPreviewStore";
import { useTransformStore } from "@/Stores/TransformStore";
import { apiGetTransformBinaries } from "@/Services/TransformService";
import { process as compileGraphInput, inputPortCountOf, inputPortIndexByName, type GraphInput } from "@/audio/pipeline/GraphCompiler";

// ─── Preview: compiles the in-progress graph and runs it through the same
// preview session single-transform preview uses (CreatorPreviewStore).

export function useCompositePreviewControls(transformId: number) {
  const editingGraph = useCompositeCanvasStore((s) => s.editingGraph);
  const previewStatus = useCreatorPreviewStore((s) => s.status);
  const previewTransformId = useCreatorPreviewStore((s) => s.previewTransformId);
  const playPreview = useCreatorPreviewStore((s) => s.play);
  const stopPreview = useCreatorPreviewStore((s) => s.stop);

  const isPreviewingThis = previewTransformId === transformId && previewStatus !== "idle" && previewStatus !== "error";
  const isLoading = previewTransformId === transformId && previewStatus === "loading";

  async function togglePreview() {
    if (isPreviewingThis) {
      stopPreview();
      return;
    }
    if (!editingGraph || editingGraph.nodes.size === 0) return;

    // Phase 3: disabled nodes and their incident edges are excluded from the
    // Preview/Play compile — same filtering toGraphDefinition() applies for
    // Save (see CompositeCanvasStore.ts) — so Play always reflects exactly
    // what's currently enabled on the canvas. Input/Output boundary nodes are
    // also excluded here — they're pure wiring placeholders with no
    // transform_binary, so GraphCompiler (which only knows how to compile
    // real transform nodes) never sees them; any edge touching one drops out
    // along with it via the enabledIds filter below.
    const nodes = [...editingGraph.nodes.values()].filter(
      (n): n is CanvasLeafNode => n.node_kind === "leaf" && !editingGraph.disabledNodes.has(n.node_id)
    );
    if (nodes.length === 0) return;
    const enabledIds = new Set(nodes.map((n) => n.node_id));
    const enabledLeafById = new Map(nodes.map((n) => [n.node_id, n]));
    const edges = [...editingGraph.edges.values()].filter(
      (e) => enabledIds.has(e.from_node_id) && enabledIds.has(e.to_node_id)
    );

    const graphInput: GraphInput = {
      nodes: new Map(
        nodes.map((n) => {
          const params = [...(useTransformStore.getState().definitions.get(n.transform_id)?.params ?? [])]
            .sort((a, b) => a.param_order - b.param_order)
            .reduce<Record<string, number>>((acc, p) => {
              acc[p.name] = p.default_value;
              return acc;
            }, {});
          return [n.node_id, { id: n.node_id, transformId: n.transform_id, params, inputPortCount: inputPortCountOf(n.transform_id) }];
        })
      ),
      edges: new Map(
        edges.map((e, i) => {
          const toTransformId = enabledLeafById.get(e.to_node_id)?.transform_id;
          return [
            i,
            {
              id: i,
              fromNodeId: e.from_node_id,
              toNodeId: e.to_node_id,
              toPortIndex: toTransformId != null ? inputPortIndexByName(toTransformId, e.to_port) : 0,
            },
          ];
        })
      ),
    };

    const result = compileGraphInput(graphInput);
    if (!result.ok) return;

    const distinctTransformIds = [...new Set(nodes.map((n) => n.transform_id))];
    const binariesMap = await apiGetTransformBinaries(distinctTransformIds);
    const binaries: Record<number, Uint8Array> = {};
    for (const [id, bytes] of binariesMap) binaries[id] = bytes;

    const resourceKey = JSON.stringify({
      nodes: nodes.map((n) => [n.node_id, n.transform_id]),
      edges,
    });

    void playPreview(transformId, resourceKey, result.graph, binaries, []);
  }

  return { isPreviewingThis, isLoading, togglePreview };
}
