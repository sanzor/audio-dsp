import { useEffect } from "react";
import ReactFlow, {
  Background,
  BackgroundVariant,
  ReactFlowProvider,
  useReactFlow,
  type Connection,
  type Edge,
  type NodeChange,
  type EdgeChange,
} from "reactflow";
import "reactflow/dist/style.css";
import { useCreatorStore } from "@/Stores/CreatorStore";
import { useCompositeCanvasStore, type CanvasNode, type EditingCompositeGraph } from "@/Stores/CompositeCanvasStore";
import { useGetTransformDefinition } from "@/hooks/transforms/queries";
import { useSaveCompositeTransform, usePublishTransform } from "@/hooks/transforms/mutations";
import { useTransformStore } from "@/Stores/TransformStore";
import { usePublishWithPortShapeDiff } from "../usePublishWithPortShapeDiff";
import { ToolbarButton } from "../toolbar-button";
import { CompositePalette } from "./composite-palette";
import { NODE_TYPES } from "./composite-canvas-node";
import { useCompositeGraphView } from "./composite-graph-view";
import { NodeInspectorPanel } from "./composite-node-inspector";
import { CompositeIoGhostSlot } from "./composite-io-placeholder";
import type { TransformPort } from "@/domain/Transform/TransformPort";
import { IO_NODE_PORT_NAME, type CompositeEdge } from "@/domain/Transform/CompositeGraphDefinition";

function portsFor(transformId: number): TransformPort[] {
  return useTransformStore.getState().definitions.get(transformId)?.ports ?? [];
}

// Resolves the port a wiring endpoint presents in the given direction —
// a real TransformPort for a leaf node (looked up by name), or the fixed
// synthetic pseudo-port every Input/Output node exposes on its one implicit
// handle. Unconstrained cardinality ("many") for the synthetic port since
// no client-side rule about IO-node fan-in/fan-out exists beyond what
// composite_validator.rs independently re-checks server-side.
function resolvePort(node: CanvasNode, portName: string, direction: "input" | "output"): TransformPort | undefined {
  if (node.node_kind === "leaf") {
    return portsFor(node.transform_id).find((p) => p.name === portName && p.direction === direction);
  }
  if (portName !== IO_NODE_PORT_NAME) return undefined;
  return { port_id: -1, name: IO_NODE_PORT_NAME, direction, port_order: 0, kind: "program", cardinality: "many" };
}

// Shared validity predicate for a prospective wiring endpoint pair — used
// both to gate onConnect (post-drop) and to drive reactflow's built-in
// connecting/valid handle classes during drag (isValidConnection). See the
// .connecting/.valid rule in index.css for why those are the two classes to
// target — not "connectingto"/"invalid", which is the v12 (@xyflow/react)
// successor's naming, not this pinned v11.11.4's.
function isConnectionAllowed(editingGraph: EditingCompositeGraph | null, connection: Connection | Edge): boolean {
  if (!connection.source || !connection.target || !connection.sourceHandle || !connection.targetHandle) return false;
  const fromNodeId = Number(connection.source);
  const toNodeId = Number(connection.target);
  const fromPort = connection.sourceHandle.replace(/^out-/, "");
  const toPort = connection.targetHandle.replace(/^in-/, "");

  const fromNode = editingGraph?.nodes.get(fromNodeId);
  const toNode = editingGraph?.nodes.get(toNodeId);
  if (!fromNode || !toNode) return false;

  const outputPort = resolvePort(fromNode, fromPort, "output");
  const inputPort = resolvePort(toNode, toPort, "input");
  if (!outputPort || !inputPort) return false;

  if (inputPort.cardinality === "single") {
    const alreadyConnected = [...(editingGraph?.edges.values() ?? [])].some(
      (e) => e.to_node_id === toNodeId && e.to_port === toPort
    );
    if (alreadyConnected) return false;
  }
  return true;
}

// ─── Inner canvas ─────────────────────────────────────────────────────────────

function CompositeCanvasInner() {
  const selectedId = useCreatorStore((s) => s.selectedTransformId);
  const { data: definition, error: definitionError } = useGetTransformDefinition(selectedId);
  const { screenToFlowPosition } = useReactFlow();

  const editingGraph = useCompositeCanvasStore((s) => s.editingGraph);
  const beginEditingCompositeGraph = useCompositeCanvasStore((s) => s.beginEditingCompositeGraph);
  const addNode = useCompositeCanvasStore((s) => s.addNode);
  const removeNode = useCompositeCanvasStore((s) => s.removeNode);
  const moveNode = useCompositeCanvasStore((s) => s.moveNode);
  const addEdge = useCompositeCanvasStore((s) => s.addEdge);
  const removeEdge = useCompositeCanvasStore((s) => s.removeEdge);
  const addIoNode = useCompositeCanvasStore((s) => s.addIoNode);
  const markSaved = useCompositeCanvasStore((s) => s.markSaved);
  const isDirty = useCompositeCanvasStore((s) => s.isDirty());
  const toGraphDefinition = useCompositeCanvasStore((s) => s.toGraphDefinition);
  const selectedNodeId = useCompositeCanvasStore((s) => s.selectedNodeId);
  const selectNode = useCompositeCanvasStore((s) => s.selectNode);

  const saveMutation = useSaveCompositeTransform(selectedId ?? -1);
  const publishMutation = usePublishTransform(selectedId ?? -1);
  const { handlePublish } = usePublishWithPortShapeDiff(selectedId, publishMutation);

  // (Re)initialize the editing graph whenever a different composite is
  // selected — seed each node's position from the loaded definition (now
  // persisted server-side, see CompositeGraphDefinition.ts), falling back
  // to a left-to-right auto-layout only for nodes with no saved position at
  // all (older rows predating this field come back as `{x:0, y:0}` rather
  // than omitting the key, so this can't distinguish "explicitly saved at
  // 0,0" from "never saved" — not worth the complexity to special-case).
  useEffect(() => {
    if (selectedId == null || definition == null || definition.transform_id !== selectedId) return;
    if (editingGraph != null && editingGraph.transformId === selectedId) return;

    const positions = new Map<number, { x: number; y: number }>();
    (definition.graph_definition?.nodes ?? []).forEach((n, i) => {
      positions.set(n.node_id, n.position ?? { x: (i % 4) * 260, y: Math.floor(i / 4) * 200 });
    });
    beginEditingCompositeGraph(selectedId, definition.graph_definition, positions);
  }, [selectedId, definition, editingGraph, beginEditingCompositeGraph]);

  const { rfNodes, rfEdges, hasExposedInput, hasExposedOutput } = useCompositeGraphView(editingGraph);

  if (selectedId == null) {
    return (
      <div className="w-full h-full flex items-center justify-center">
        <span className="text-sm" style={{ color: "var(--text-muted)" }}>Select or create a transform to begin.</span>
      </div>
    );
  }
  if (definitionError != null) {
    return (
      <div className="w-full h-full flex items-center justify-center">
        <span className="text-xs font-mono" style={{ color: "#ff6b6b" }}>
          Failed to load: {(definitionError as Error).message ?? "unknown error"}
        </span>
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

  const selectedNode = selectedNodeId != null ? editingGraph.nodes.get(selectedNodeId) ?? null : null;

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
    if (!isConnectionAllowed(editingGraph, connection)) return;
    const fromNodeId = Number(connection.source);
    const toNodeId = Number(connection.target);
    const fromPort = connection.sourceHandle!.replace(/^out-/, "");
    const toPort = connection.targetHandle!.replace(/^in-/, "");

    const edge: CompositeEdge = { from_node_id: fromNodeId, from_port: fromPort, to_node_id: toNodeId, to_port: toPort };
    addEdge(edge);
  }

  function onDragOver(e: React.DragEvent) {
    e.preventDefault();
    e.dataTransfer.dropEffect = "move";
  }

  function onDrop(e: React.DragEvent) {
    e.preventDefault();
    const position = screenToFlowPosition({ x: e.clientX, y: e.clientY });

    const transformRaw = e.dataTransfer.getData("application/transform");
    if (transformRaw) {
      const { transformId } = JSON.parse(transformRaw) as { transformId: number; name: string };
      addNode(transformId, position);
      return;
    }

    const ioRaw = e.dataTransfer.getData("application/composite-io");
    if (ioRaw) {
      const { direction } = JSON.parse(ioRaw) as { direction: "input" | "output" };
      addIoNode(direction, position);
    }
  }

  function handleSave() {
    saveMutation.mutate(toGraphDefinition(), { onSuccess: () => markSaved() });
  }

  return (
    <div className="flex h-full min-h-0">
      <CompositePalette />
      <div className="flex-1 flex flex-col min-w-0">
        <div
          className="flex items-center justify-end gap-2 px-3 h-8 flex-shrink-0"
          style={{ borderBottom: "1px solid rgba(255,255,255,0.06)", backgroundColor: "var(--bg-darker)" }}
        >
          <ToolbarButton variant="save" onClick={handleSave} disabled={!isDirty || saveMutation.isPending}>
            {saveMutation.isPending ? "Saving…" : "Save Draft"}
          </ToolbarButton>
          <ToolbarButton
            variant="publish"
            onClick={handlePublish}
            disabled={publishMutation.isPending}
            title={publishMutation.error?.message ?? saveMutation.error?.message}
          >
            {publishMutation.isPending ? "Publishing…" : "Publish"}
          </ToolbarButton>
          {(saveMutation.isError || publishMutation.isError) && (
            <span className="font-mono text-[10px] max-w-[240px] truncate" style={{ color: "#ff6b6b" }}>
              {(saveMutation.error as Error | null)?.message ?? (publishMutation.error as Error | null)?.message}
            </span>
          )}
        </div>

        <div className="flex-1 min-h-0 flex flex-col" onDragOver={onDragOver} onDrop={onDrop}>
          <div className="flex-1 min-h-0">
            <ReactFlow
              className="composite-flow-canvas"
              nodes={rfNodes}
              edges={rfEdges}
              nodeTypes={NODE_TYPES}
              onNodesChange={onNodesChange}
              onEdgesChange={onEdgesChange}
              onConnect={onConnect}
              isValidConnection={(c) => isConnectionAllowed(editingGraph, c)}
              fitView
              fitViewOptions={{ padding: 0.4 }}
              deleteKeyCode={["Backspace", "Delete"]}
              zoomOnScroll
              panOnDrag
              defaultEdgeOptions={{ style: { strokeWidth: 2.5 } }}
            >
              <Background variant={BackgroundVariant.Dots} gap={24} size={1} color="rgba(255,255,255,0.04)" />
              {!hasExposedInput && <CompositeIoGhostSlot direction="input" />}
              {!hasExposedOutput && <CompositeIoGhostSlot direction="output" />}
            </ReactFlow>
          </div>
          {selectedNode != null && selectedNode.node_kind === "leaf" && (
            <NodeInspectorPanel
              nodeId={selectedNode.node_id}
              transformId={selectedNode.transform_id}
              compositeTransformId={selectedId}
              onClose={() => selectNode(null)}
            />
          )}
        </div>
      </div>
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
