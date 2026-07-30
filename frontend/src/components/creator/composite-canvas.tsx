import { useEffect, useMemo } from "react";
import ReactFlow, {
  Background,
  BackgroundVariant,
  Handle,
  Position,
  ReactFlowProvider,
  useReactFlow,
  type Connection,
  type Edge as RFEdge,
  type Node as RFNode,
  type NodeChange,
  type EdgeChange,
  type NodeProps,
} from "reactflow";
import "reactflow/dist/style.css";
import { useCreatorStore } from "@/Stores/CreatorStore";
import { useCompositeCanvasStore, type CanvasNode } from "@/Stores/CompositeCanvasStore";
import { useCreatorPreviewStore } from "@/Stores/CreatorPreviewStore";
import { useGetTransformDefinition, useResolveTransformDefinitions } from "@/hooks/transforms/queries";
import { useSaveCompositeTransform, usePublishTransform } from "@/hooks/transforms/mutations";
import { useTransformStore } from "@/Stores/TransformStore";
import { apiGetPublishPortShapeDiff, apiGetTransformBinaries, type PortShapeSummary } from "@/Services/TransformService";
import { process as compileGraphInput, type GraphInput } from "@/audio/pipeline/GraphCompiler";
import { CompositePalette } from "./composite-palette";
import type { TransformPort } from "@/domain/Transform/TransformPort";
import type { CompositeEdge } from "@/domain/Transform/CompositeGraphDefinition";

const ROW_HEIGHT = 28;

// ─── Node ─────────────────────────────────────────────────────────────────────
// Generalizes canvas.tsx's TransformPreviewNode: multiple instances, real
// per-port Handles keyed by port NAME (not port_id, since composite wiring
// references ports by name — port_id is reassigned on every leaf republish).

interface CompositeNodeData {
  nodeId: number;
  transformId: number;
}

function CompositeTransformNode({ data }: NodeProps<CompositeNodeData>) {
  // Reads from the cache useResolveTransformDefinitions (in the parent
  // canvas) already populated for every node's transform — no per-node fetch.
  const definition = useTransformStore((s) => s.definitions.get(data.transformId));
  const removeNode = useCompositeCanvasStore((s) => s.removeNode);

  const inputs = definition?.ports.filter((p) => p.direction === "input") ?? [];
  const outputs = definition?.ports.filter((p) => p.direction === "output") ?? [];
  const rows = Math.max(inputs.length, outputs.length, 1);
  const height = rows * ROW_HEIGHT + 48;

  return (
    <div
      style={{
        width: 220,
        height,
        backgroundColor: "#1e1e1e",
        border: "2px solid #adc6ff",
        borderRadius: 8,
        boxShadow: "0 0 24px rgba(173,198,255,0.12)",
        display: "flex",
        flexDirection: "column",
        overflow: "hidden",
        position: "relative",
      }}
    >
      <div
        style={{
          padding: "6px 10px",
          borderBottom: "1px solid rgba(255,255,255,0.08)",
          fontSize: 11,
          fontFamily: "JetBrains Mono, monospace",
          color: "#adc6ff",
          fontWeight: 700,
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          flexShrink: 0,
        }}
      >
        <span className="truncate">{definition?.name ?? `#${data.nodeId}`}</span>
        <button
          onClick={() => removeNode(data.nodeId)}
          style={{ color: "#ff6b6b", background: "none", border: "none", cursor: "pointer", fontSize: 12 }}
          title="Remove from composite"
        >
          ×
        </button>
      </div>

      <div style={{ flex: 1, display: "flex", position: "relative" }}>
        <div style={{ flex: 1, display: "flex", flexDirection: "column", paddingTop: 4 }}>
          {inputs.map((port) => (
            <div key={port.name} style={{ height: ROW_HEIGHT, display: "flex", alignItems: "center", paddingLeft: 14, position: "relative" }}>
              <Handle
                type="target"
                position={Position.Left}
                id={`in-${port.name}`}
                style={{ left: -1, top: "50%", transform: "translateY(-50%)", width: 8, height: 8, backgroundColor: "#adc6ff", border: "none" }}
              />
              <span style={{ fontSize: 10, fontFamily: "JetBrains Mono, monospace", color: "#adc6ff", opacity: 0.8 }}>{port.name}</span>
            </div>
          ))}
        </div>
        <div style={{ flex: 1, display: "flex", flexDirection: "column", paddingTop: 4 }}>
          {outputs.map((port) => (
            <div key={port.name} style={{ height: ROW_HEIGHT, display: "flex", alignItems: "center", justifyContent: "flex-end", paddingRight: 14, position: "relative" }}>
              <span style={{ fontSize: 10, fontFamily: "JetBrains Mono, monospace", color: "#4ae176", opacity: 0.8 }}>{port.name}</span>
              <Handle
                type="source"
                position={Position.Right}
                id={`out-${port.name}`}
                style={{ right: -1, top: "50%", transform: "translateY(-50%)", width: 8, height: 8, backgroundColor: "#4ae176", border: "none" }}
              />
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

const NODE_TYPES = { composite: CompositeTransformNode };

function portsFor(transformId: number): TransformPort[] {
  return useTransformStore.getState().definitions.get(transformId)?.ports ?? [];
}

function portKey(nodeId: number, portName: string) {
  return `${nodeId}:${portName}`;
}

// ─── Preview: compiles the in-progress graph and runs it through the same
// preview session single-transform preview uses (CreatorPreviewStore).

function useCompositePreviewControls(transformId: number) {
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

    const nodes = [...editingGraph.nodes.values()];
    const graphInput: GraphInput = {
      nodes: new Map(
        nodes.map((n) => {
          const params = [...(useTransformStore.getState().definitions.get(n.transform_id)?.params ?? [])]
            .sort((a, b) => a.param_order - b.param_order)
            .reduce<Record<string, number>>((acc, p) => {
              acc[p.name] = p.default_value;
              return acc;
            }, {});
          return [n.node_id, { id: n.node_id, transformId: n.transform_id, params }];
        })
      ),
      edges: new Map(
        [...editingGraph.edges.values()].map((e, i) => [
          i,
          { id: i, fromNodeId: e.from_node_id, toNodeId: e.to_node_id },
        ])
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
      edges: [...editingGraph.edges.values()],
    });

    void playPreview(transformId, resourceKey, result.graph, binaries, []);
  }

  return { isPreviewingThis, isLoading, togglePreview };
}

// ─── Inner canvas ─────────────────────────────────────────────────────────────

function CompositeCanvasInner() {
  const selectedId = useCreatorStore((s) => s.selectedTransformId);
  const { data: definition } = useGetTransformDefinition(selectedId);
  const { screenToFlowPosition } = useReactFlow();

  const editingGraph = useCompositeCanvasStore((s) => s.editingGraph);
  const beginEditingCompositeGraph = useCompositeCanvasStore((s) => s.beginEditingCompositeGraph);
  const addNode = useCompositeCanvasStore((s) => s.addNode);
  const removeNode = useCompositeCanvasStore((s) => s.removeNode);
  const moveNode = useCompositeCanvasStore((s) => s.moveNode);
  const addEdge = useCompositeCanvasStore((s) => s.addEdge);
  const removeEdge = useCompositeCanvasStore((s) => s.removeEdge);
  const setExposedPort = useCompositeCanvasStore((s) => s.setExposedPort);
  const clearExposedPort = useCompositeCanvasStore((s) => s.clearExposedPort);
  const markSaved = useCompositeCanvasStore((s) => s.markSaved);
  const isDirty = useCompositeCanvasStore((s) => s.isDirty());
  const toGraphDefinition = useCompositeCanvasStore((s) => s.toGraphDefinition);

  const saveMutation = useSaveCompositeTransform(selectedId ?? -1);
  const publishMutation = usePublishTransform(selectedId ?? -1);
  const { togglePreview, isPreviewingThis, isLoading: previewLoading } = useCompositePreviewControls(selectedId ?? -1);

  // (Re)initialize the editing graph whenever a different composite is
  // selected — simple left-to-right auto-layout since node position isn't
  // persisted server-side.
  useEffect(() => {
    if (selectedId == null || definition == null || definition.transform_id !== selectedId) return;
    if (editingGraph != null && editingGraph.transformId === selectedId) return;

    const positions = new Map<number, { x: number; y: number }>();
    (definition.graph_definition?.nodes ?? []).forEach((n, i) => {
      positions.set(n.node_id, { x: (i % 4) * 260, y: Math.floor(i / 4) * 200 });
    });
    beginEditingCompositeGraph(selectedId, definition.graph_definition, positions);
  }, [selectedId, definition, editingGraph, beginEditingCompositeGraph]);

  const distinctTransformIds = useMemo(
    () => (editingGraph ? [...new Set([...editingGraph.nodes.values()].map((n) => n.transform_id))] : []),
    [editingGraph]
  );
  useResolveTransformDefinitions(distinctTransformIds);

  const rfNodes: RFNode<CompositeNodeData>[] = useMemo(
    () =>
      editingGraph
        ? [...editingGraph.nodes.values()].map((n: CanvasNode) => ({
            id: String(n.node_id),
            type: "composite",
            position: n.position,
            data: { nodeId: n.node_id, transformId: n.transform_id },
          }))
        : [],
    [editingGraph]
  );

  const rfEdges: RFEdge[] = useMemo(
    () =>
      editingGraph
        ? [...editingGraph.edges.values()].map((e) => ({
            id: `${e.from_node_id}:${e.from_port}->${e.to_node_id}:${e.to_port}`,
            source: String(e.from_node_id),
            target: String(e.to_node_id),
            sourceHandle: `out-${e.from_port}`,
            targetHandle: `in-${e.to_port}`,
          }))
        : [],
    [editingGraph]
  );

  // Every port on every node not touched by any edge is a candidate for
  // exposure — same "genuinely unconnected" derivation the backend
  // validator independently re-checks server-side.
  const candidatePorts: { nodeId: number; nodeName: string; port: TransformPort }[] = useMemo(() => {
    if (!editingGraph) return [];
    const touched = new Set<string>();
    for (const e of editingGraph.edges.values()) {
      touched.add(portKey(e.from_node_id, e.from_port));
      touched.add(portKey(e.to_node_id, e.to_port));
    }
    const candidates: { nodeId: number; nodeName: string; port: TransformPort }[] = [];
    for (const node of editingGraph.nodes.values()) {
      const def = useTransformStore.getState().definitions.get(node.transform_id);
      for (const port of portsFor(node.transform_id)) {
        if (!touched.has(portKey(node.node_id, port.name))) {
          candidates.push({ nodeId: node.node_id, nodeName: def?.name ?? `#${node.node_id}`, port });
        }
      }
    }
    return candidates;
  }, [editingGraph]);

  if (selectedId == null) {
    return (
      <div className="w-full h-full flex items-center justify-center">
        <span className="text-sm" style={{ color: "var(--text-muted)" }}>Select or create a transform to begin.</span>
      </div>
    );
  }
  if (definition == null || editingGraph == null) {
    return (
      <div className="w-full h-full flex items-center justify-center">
        <span className="text-xs font-mono" style={{ color: "var(--text-muted)" }}>Loading...</span>
      </div>
    );
  }

  function onNodesChange(changes: NodeChange[]) {
    for (const change of changes) {
      if (change.type === "position" && change.position) {
        moveNode(Number(change.id), change.position);
      } else if (change.type === "remove") {
        removeNode(Number(change.id));
      }
    }
  }

  function onEdgesChange(changes: EdgeChange[]) {
    for (const change of changes) {
      if (change.type === "remove" && editingGraph) {
        const edge = editingGraph.edges.get(change.id);
        if (edge) removeEdge(edge);
      }
    }
  }

  function onConnect(connection: Connection) {
    if (!connection.source || !connection.target || !connection.sourceHandle || !connection.targetHandle) return;
    const fromNodeId = Number(connection.source);
    const toNodeId = Number(connection.target);
    const fromPort = connection.sourceHandle.replace(/^out-/, "");
    const toPort = connection.targetHandle.replace(/^in-/, "");

    const fromNode = editingGraph?.nodes.get(fromNodeId);
    const toNode = editingGraph?.nodes.get(toNodeId);
    if (!fromNode || !toNode) return;

    const outputPort = portsFor(fromNode.transform_id).find((p) => p.name === fromPort && p.direction === "output");
    const inputPort = portsFor(toNode.transform_id).find((p) => p.name === toPort && p.direction === "input");
    if (!outputPort || !inputPort) return;

    if (inputPort.cardinality === "single") {
      const alreadyConnected = [...(editingGraph?.edges.values() ?? [])].some(
        (e) => e.to_node_id === toNodeId && e.to_port === toPort
      );
      if (alreadyConnected) return;
    }

    const edge: CompositeEdge = { from_node_id: fromNodeId, from_port: fromPort, to_node_id: toNodeId, to_port: toPort };
    addEdge(edge);
  }

  function onDragOver(e: React.DragEvent) {
    e.preventDefault();
    e.dataTransfer.dropEffect = "move";
  }

  function onDrop(e: React.DragEvent) {
    e.preventDefault();
    const raw = e.dataTransfer.getData("application/transform");
    if (!raw) return;
    const { transformId } = JSON.parse(raw) as { transformId: number; name: string };
    const position = screenToFlowPosition({ x: e.clientX, y: e.clientY });
    addNode(transformId, position);
  }

  function handleSave() {
    saveMutation.mutate(toGraphDefinition(), { onSuccess: () => markSaved() });
  }

  async function handlePublish() {
    if (selectedId == null) return;
    try {
      const diff = await apiGetPublishPortShapeDiff(selectedId);
      if (diff.changed) {
        const describe = (ports: PortShapeSummary[]) =>
          ports.length === 0 ? "(none)" : ports.map((p) => `${p.name} [${p.direction}, ${p.kind}/${p.cardinality}]`).join(", ");
        const proceed = window.confirm(
          "This transform's port shape has changed since it was last published.\n\n" +
            `Currently published: ${describe(diff.current)}\n` +
            `About to publish: ${describe(diff.incoming)}\n\n` +
            "Editor graphs already wired to the old shape will fail closed rather than silently misrouting audio, but they will need to be re-wired. Publish anyway?"
        );
        if (!proceed) return;
      }
    } catch {
      // Advisory only.
    }
    publishMutation.mutate();
  }

  return (
    <div className="flex h-full min-h-0">
      <CompositePalette />
      <div className="flex-1 flex flex-col min-w-0">
        <div
          className="flex items-center justify-end gap-2 px-3 h-8 flex-shrink-0"
          style={{ borderBottom: "1px solid rgba(255,255,255,0.06)", backgroundColor: "var(--bg-darker)" }}
        >
          <button
            onClick={togglePreview}
            disabled={editingGraph.nodes.size === 0}
            className="font-mono font-bold px-2.5 py-0.5 rounded text-[10px]"
            style={{
              color: isPreviewingThis ? "#ff6b6b" : "#4ae176",
              border: `1px solid ${isPreviewingThis ? "rgba(255,107,107,0.4)" : "rgba(74,225,118,0.4)"}`,
              opacity: editingGraph.nodes.size === 0 ? 0.5 : 1,
            }}
          >
            {previewLoading ? "Loading…" : isPreviewingThis ? "Stop" : "Play"}
          </button>
          <button
            onClick={handleSave}
            disabled={!isDirty || saveMutation.isPending}
            className="font-mono font-bold px-2.5 py-0.5 rounded text-[10px]"
            style={{ color: "#4ae176", border: "1px solid rgba(74,225,118,0.4)", opacity: !isDirty || saveMutation.isPending ? 0.5 : 1 }}
          >
            {saveMutation.isPending ? "Saving…" : "Save"}
          </button>
          <button
            onClick={handlePublish}
            disabled={publishMutation.isPending}
            title={publishMutation.error?.message ?? saveMutation.error?.message}
            className="font-mono font-bold px-2.5 py-0.5 rounded text-[10px]"
            style={{ color: "#f472b6", border: "1px solid rgba(244,114,182,0.4)", opacity: publishMutation.isPending ? 0.5 : 1 }}
          >
            {publishMutation.isPending ? "Publishing…" : "Publish"}
          </button>
          {(saveMutation.isError || publishMutation.isError) && (
            <span className="font-mono text-[10px] max-w-[240px] truncate" style={{ color: "#ff6b6b" }}>
              {(saveMutation.error as Error | null)?.message ?? (publishMutation.error as Error | null)?.message}
            </span>
          )}
        </div>

        <div className="flex-1 min-h-0" onDragOver={onDragOver} onDrop={onDrop}>
          <ReactFlow
            nodes={rfNodes}
            edges={rfEdges}
            nodeTypes={NODE_TYPES}
            onNodesChange={onNodesChange}
            onEdgesChange={onEdgesChange}
            onConnect={onConnect}
            fitView
            fitViewOptions={{ padding: 0.4 }}
            deleteKeyCode={["Backspace", "Delete"]}
            zoomOnScroll
            panOnDrag
          >
            <Background variant={BackgroundVariant.Dots} gap={24} size={1} color="rgba(255,255,255,0.04)" />
          </ReactFlow>
        </div>
      </div>

      <aside
        className="flex flex-col w-72 flex-shrink-0 overflow-hidden"
        style={{ backgroundColor: "var(--bg-darker)", borderLeft: "1px solid rgba(255,255,255,0.06)" }}
      >
        <div className="px-3 py-2" style={{ borderBottom: "1px solid rgba(255,255,255,0.06)" }}>
          <span className="text-[10px] font-mono font-bold" style={{ color: "var(--text-main)" }}>EXPOSED PORTS</span>
          <p className="mt-1 text-[9px]" style={{ color: "var(--text-muted)", opacity: 0.7 }}>
            Every unconnected port on every node. Check to expose it as one of this composite's own ports.
          </p>
        </div>
        <div className="flex-1 overflow-y-auto p-2 flex flex-col gap-1">
          {candidatePorts.length === 0 && (
            <span className="text-[10px] font-mono" style={{ color: "var(--text-muted)", opacity: 0.6 }}>
              No unconnected ports — drag transforms onto the canvas first.
            </span>
          )}
          {candidatePorts.map(({ nodeId, nodeName, port }) => {
            const exposed = editingGraph.exposedPorts.get(portKey(nodeId, port.name));
            return (
              <div key={portKey(nodeId, port.name)} className="flex flex-col gap-1 px-2 py-1.5 rounded text-xs" style={{ backgroundColor: "var(--bg-dark)", border: "1px solid rgba(255,255,255,0.05)" }}>
                <label className="flex items-center gap-2">
                  <input
                    type="checkbox"
                    checked={exposed != null}
                    onChange={(e) => {
                      if (e.target.checked) setExposedPort(nodeId, port.name, port.name);
                      else clearExposedPort(nodeId, port.name);
                    }}
                  />
                  <span style={{ color: "var(--text-muted)" }}>
                    {nodeName}.{port.name} ({port.direction})
                  </span>
                </label>
                {exposed != null && (
                  <input
                    className="w-full rounded px-2 py-1 text-xs"
                    style={{ backgroundColor: "var(--bg-darker)", border: "1px solid rgba(255,255,255,0.08)", color: "var(--text-main)" }}
                    value={exposed.exposed_name}
                    onChange={(e) => setExposedPort(nodeId, port.name, e.target.value)}
                    placeholder="Exposed name"
                  />
                )}
              </div>
            );
          })}
        </div>
      </aside>
    </div>
  );
}

// ─── Public export ────────────────────────────────────────────────────────────

export function CompositeCanvas() {
  return (
    <ReactFlowProvider>
      <CompositeCanvasInner />
    </ReactFlowProvider>
  );
}
