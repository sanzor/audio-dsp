import ReactFlow, {
  Background,
  BackgroundVariant,
  ReactFlowProvider,
  useNodesState,
  useEdgesState,
  useReactFlow,
  addEdge,
  reconnectEdge,
  type Node,
  type Edge,
  type Connection,
  type NodeChange,
} from "reactflow";
import { useCallback, useEffect, useMemo, useRef } from "react";
import "reactflow/dist/style.css";
import { useUIStore } from "@/Stores/UIStore";
import { useGraphStore } from "@/Stores/GraphStore";
import { useTransformStore } from "@/Stores/TransformStore";
import { useWorkletStore } from "@/Stores/WorkletStore";
import type { NodeType } from "@/domain/Graph/Node";
import type { TransformDefinition } from "@/domain/Transform/TransformDefinition";
import type { Graph } from "@/domain/Graph/Graph";
import { CanvasToolbar } from "./canvas-toolbar";
import { useGraphController } from "@/controllers/GraphController";
import { SourceNode } from "./source-node";
import { SinkNode } from "./sink-node";
import { useActiveGraphId } from "@/hooks/graphs/useActiveGraphId";
import { NodeDetailsModal } from "../modals/graph/node-details-modal";
import { apiGetTransformDefinition } from "@/Services/TransformService";
import { useGraphCompiler } from "@/audio/hooks/useGraphCompiler";
import { useWorklet as useWorklet } from "@/audio/hooks/useWorklet";
import { CompileOutputModal } from "./compile-output-modal";
import { SaveCompileStatusOverlay } from "./save-compile-status-overlay";
import { useCanDropTransform } from "./useCanDropTransform";
import { RuntimeStatusOverlay } from "./runtime-status-overlay";
import { useSaveCompileFlow } from "./useSaveCompileFlow";


const NODE_TYPES = { source: SourceNode, sink: SinkNode };

function defaultParamsForTransform(transform?: TransformDefinition) {
  if (!transform) return {};

  const ordered = [...transform.params].sort((a, b) => a.param_order - b.param_order);
  return Object.fromEntries(ordered.map((param) => [param.name, param.default_value]));
}

function nextTempIdSeed(graph?: { nodes: Array<{ id: number }>; edges: Array<{ id: number }> }) {
  if (!graph) return -1;

  const ids = [
    ...graph.nodes.map((node) => node.id),
    ...graph.edges.map((edge) => edge.id),
  ];

  const minId = ids.length > 0 ? Math.min(...ids, -1) : -1;
  return minId - 1;
}

// Minimal validity notion for this surface: no self-loops, no exact duplicate
// (source, target) pairs. There's no named-port/cardinality concept here (unlike
// Creator's composite canvas) — nodes have at most one unnamed handle per side,
// so this is deliberately the whole rule set the current model supports.
function isConnectionAllowed(edges: Edge[], connection: Connection | Edge): boolean {
  if (connection.source == null || connection.target == null) return false;
  if (connection.source === connection.target) return false;
  return !edges.some(
    (e) => e.source === connection.source && e.target === connection.target
  );
}

// ─── Canvas ───────────────────────────────────────────────────────────────────

function CanvasInner({ graphId, graph }: { graphId: number | undefined; graph: Graph | undefined }) {
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);
  const nextCanvasTempIdRef = useRef(-1);

  const onNodesChangeSafe = useCallback(
    (changes: NodeChange[]) => {
      const filtered = changes.filter(
        (c) => !(c.type === "remove" && nodes.find((n) => n.id === c.id && (n.data.nodeType === "source" || n.data.nodeType === "sink")))
      );
      onNodesChange(filtered);
    },
    [nodes, onNodesChange],
  );
  const { screenToFlowPosition, fitView } = useReactFlow();
  const {openModal,closeModal,modalState} = useUIStore();

  const { canDropTransform, regionId } = useCanDropTransform();

  const {summaries,definitions,upsertDefinition}=useTransformStore();
  const  {effectsEnabled,setEffectsEnabled,graphPlaybackState }=useWorkletStore();

  const graphController = useGraphController();
  const isGraphLive = graphId != null && effectsEnabled;
  const nodeDetailsModalState = modalState?.type === "nodeDetails" ? modalState : null;

  const nodeDetailsTarget = useMemo(() => {
    if (!nodeDetailsModalState?.nodeId) return null;
    return nodes.find((node) => (node.data.nodeId as number | undefined) === nodeDetailsModalState.nodeId) ?? null;
  }, [nodeDetailsModalState, nodes]);

  const nextCanvasTempId = useCallback(() => {
    const id = nextCanvasTempIdRef.current;
    nextCanvasTempIdRef.current -= 1;
    return id;
  }, []);

  useEffect(() => {
    nextCanvasTempIdRef.current = nextTempIdSeed(graph);

    setNodes(
      graph?.nodes.map((n) => {
        const isStructural = n.nodeType === "source" || n.nodeType === "sink";
        const transformName = n.transformId != null
          ? (summaries.get(n.transformId)?.name ?? String(n.id))
          : String(n.id);
        return {
          id: String(n.id),
          type: (n.nodeType ?? "default") as NodeType,
          position: n.position,
          deletable: !isStructural,
          draggable: true,
          data: {
            label: n.nodeType === "source" ? "Input" : n.nodeType === "sink" ? "Output" : transformName,
            nodeId: n.id,
            transformId: n.transformId ?? null,
            params: n.params ?? {},
            nodeType: n.nodeType,
          },
        };
      }) ?? []
    );
    setEdges(
      graph?.edges.map((e) => ({
        id: String(e.id),
        source: String(e.fromNodeId),
        target: String(e.toNodeId),
      })) ?? []
    );
  }, [regionId, graphId, graph, setNodes, setEdges, summaries]);

  useEffect(() => {
    setNodes((current) =>
      current.map((node) => {
        const nodeType = node.data.nodeType as NodeType;
        if (nodeType === "source") {
          if (node.data.label === "Input") return node;
          return { ...node, data: { ...node.data, label: "Input" } };
        }
        if (nodeType === "sink") {
          if (node.data.label === "Output") return node;
          return { ...node, data: { ...node.data, label: "Output" } };
        }

        const transformId = node.data.transformId as number | null | undefined;
        if (transformId == null) {
          return node;
        }

        const label = summaries.get(transformId)?.name ?? node.data.label;
        if (label === node.data.label) {
          return node;
        }

        return {
          ...node,
          data: {
            ...node.data,
            label,
          },
        };
      })
    );
  }, [summaries, setNodes]);

  const onConnect = useCallback(
    (connection: Connection) =>
      setEdges((es) => {
        if (!isConnectionAllowed(es, connection)) return es;
        return addEdge(
          {
            ...connection,
            id: String(nextCanvasTempId()),
          },
          es
        );
      }),
    [setEdges, nextCanvasTempId]
  );

  const isValidConnection = useCallback(
    (connection: Connection) => isConnectionAllowed(edges, connection),
    [edges]
  );

  const onReconnect = useCallback(
    (oldEdge: Edge, newConnection: Connection) => {
      setEdges((es) =>
        reconnectEdge(oldEdge, newConnection, es, { shouldReplaceId: false })
      );
    },
    [setEdges]
  );

  const onNodeDoubleClick = useCallback(
    (_: React.MouseEvent, node: Node) => {
      const nodeType = node.data.nodeType as NodeType;
      if (nodeType === "source" || nodeType === "sink") return;
      openModal({
        type: "nodeDetails",
        nodeId: (node.data.nodeId as number) ?? null,
        transformId: (node.data.transformId as number) ?? null,
      });
    },
    [openModal]
  );

  const onDragOver = useCallback((e: React.DragEvent) => {
    if (!canDropTransform) {
      e.dataTransfer.dropEffect = "none";
      return;
    }
    e.preventDefault();
    e.dataTransfer.dropEffect = "move";
  }, [canDropTransform]);

  const onDrop = useCallback(
    async (e: React.DragEvent) => {
      if (!canDropTransform) return;
      e.preventDefault();
      const raw = e.dataTransfer.getData("application/transform");
      if (!raw) return;

      const { transformId, name } = JSON.parse(raw) as { transformId: number; name: string };
      const position = screenToFlowPosition({ x: e.clientX, y: e.clientY });
      const tempNodeId = nextCanvasTempId();
      const definition = definitions.get(transformId);

      setNodes((ns) => [
        ...ns,
        {
          id: String(tempNodeId),
          type: "default",
          position,
          data: {
            label: name,
            nodeId: tempNodeId,
            transformId,
            params: defaultParamsForTransform(definition),
            nodeType: "default",
          },
        },
      ]);

      if (!definition) {
        try {
          const fetched = await apiGetTransformDefinition(transformId);
          upsertDefinition(fetched);
          setNodes((current) =>
            current.map((n) => {
              if ((n.data.nodeId as number | undefined) !== tempNodeId) return n;
              const params = (n.data.params as Record<string, number> | undefined) ?? {};
              if (Object.keys(params).length > 0) return n;
              return { ...n, data: { ...n.data, params: defaultParamsForTransform(fetched) } };
            })
          );
        } catch (error) {
          console.error("Failed to load transform definition:", error);
        }
      }
    },
    [canDropTransform, screenToFlowPosition, setNodes, nextCanvasTempId, definitions, upsertDefinition]
  );

  // ─── Toolbar handlers ───────────────────────────────────────────────────────

  const { compileNow } = useGraphCompiler(graphId, nodes, edges);
  const { uploadToWorklet, canUploadToWorklet} = useWorklet();

  const { saveState, saveProgress, saveCompileState, saveCompileMessage, handleSave } = useSaveCompileFlow({
    graphId,
    nodes,
    edges,
    handleSaveGraph: graphController.handleSaveGraph,
    compileNow,
    compiled: graphPlaybackState.compiled,
  });

  const handleFitView = useCallback(() => {
    fitView({ padding: 0.2 });
  }, [fitView]);

  const handleClearNodes = useCallback(() => {
    if (graphId == null) return;
    graphController.handleClearGraphNodes(graphId);
  }, [graphId, graphController]);

  const handleRename = useCallback(() => {
    if (graphId == null) return;
    graphController.handleRenameGraph(graphId);
  }, [graphId, graphController]);

  const handleDelete = useCallback(async () => {
    if (graphId == null) return;
    await graphController.handleDeleteGraph(graphId);
  }, [graphId, graphController]);

  const handleCopy = useCallback(() => {
    if (graphId == null) return;
    graphController.handleCopyGraph(graphId);
  }, [graphId, graphController]);

  const clearCanvasSelection = useCallback(() => {
    setNodes((ns) =>
      ns.map((node) =>
        node.selected ? { ...node, selected: false } : node
      )
    );
    setEdges((es) =>
      es.map((edge) =>
        edge.selected ? { ...edge, selected: false } : edge
      )
    );
  }, [setNodes, setEdges]);

  const handleCanvasMouseDownCapture = useCallback(
    (event: React.MouseEvent<HTMLDivElement>) => {
      if (!(event.target instanceof Element)) return;
      const interactiveTarget = event.target.closest(
        ".react-flow__node, .react-flow__edge, .react-flow__handle, .react-flow__controls"
      );
      if (interactiveTarget) return;
      clearCanvasSelection();
    },
    [clearCanvasSelection]
  );

  const hasClearableNodes = nodes.some((n) => n.data.nodeType === "default");

  // Stable nodeTypes reference — must not be recreated on every render
  const nodeTypes = useMemo(() => NODE_TYPES, []);

  return (
    <div className={`flex flex-col w-full h-full overflow-hidden rounded-[10px]${isGraphLive ? " graph-live-shell" : ""}`}>
      <CanvasToolbar
        selectedGraphId={graphId}
        hasClearableNodes={hasClearableNodes}
        isSaving={saveState === "saving"}
        effectsEnabled={isGraphLive}
        graphPlaybackState={graphPlaybackState}
        canActivate={canUploadToWorklet}
        onCompile={compileNow}
        onActivate={uploadToWorklet}
        onSave={handleSave}
        onToggleEffects={() => setEffectsEnabled(!effectsEnabled)}
        onFitView={handleFitView}
        onClearNodes={handleClearNodes}
        onRename={handleRename}
        onDelete={handleDelete}
        onCopy={handleCopy}
      />
      <div
        className="canvas-area relative flex-1 min-h-0"
        onDragOver={onDragOver}
        onDrop={onDrop}
        onMouseDownCapture={handleCanvasMouseDownCapture}
      >
        <ReactFlow
          className="editor-flow-canvas"
          nodes={nodes}
          edges={edges}
          nodeTypes={nodeTypes}
          onNodesChange={onNodesChangeSafe}
          onEdgesChange={onEdgesChange}
          onConnect={onConnect}
          onReconnect={onReconnect}
          isValidConnection={isValidConnection}
          onNodeDoubleClick={onNodeDoubleClick}
          onPaneClick={clearCanvasSelection}
          edgesUpdatable
          deleteKeyCode={["Backspace", "Delete"]}
          fitView
        >
          <Background variant={BackgroundVariant.Lines} gap={20} color="rgba(255,255,255,0.03)" />
        </ReactFlow>
        <RuntimeStatusOverlay />
        {saveState !== "hidden" && (
          <div className="graph-save-progress">
            <div className="graph-save-progress__meta">
              <span>
                {saveState === "saving"
                  ? "Saving graph..."
                  : saveState === "success"
                    ? "Graph saved"
                    : "Save failed"}
              </span>
              <span>{Math.round(saveProgress)}%</span>
            </div>
            <div className="graph-save-progress__track">
              <div
                className={`graph-save-progress__fill graph-save-progress__fill--${saveState}`}
                style={{ width: `${saveProgress}%` }}
              />
            </div>
          </div>
        )}
        <SaveCompileStatusOverlay
          state={saveCompileState}
          message={saveCompileMessage}
        />
      </div>
      {nodeDetailsModalState && (
        <NodeDetailsModal
          nodeId={nodeDetailsModalState.nodeId}
          position={nodeDetailsTarget?.position ?? null}
          initialParams={(nodeDetailsTarget?.data.params as Record<string, number> | undefined) ?? {}}
          transformId={nodeDetailsModalState.transformId}
          open
          onClose={closeModal}
          onSubmitParams={(nodeId, params) => {
            const updatedNodes = nodes.map((node) =>
              (node.data.nodeId as number | undefined) === nodeId
                ? { ...node, data: { ...node.data, params } }
                : node
            );
            setNodes(updatedNodes);
            compileNow(updatedNodes);
            handleSave(updatedNodes);
          }}
        />
      )}
      <CompileOutputModal />
    </div>
  );
}

export function CanvasPanel() {
  const graphId = useActiveGraphId();
  const graph = useGraphStore((s) =>
    graphId != null ? s.graphs.get(graphId) : undefined
  );
  return (
    <ReactFlowProvider>
      <CanvasInner graphId={graphId} graph={graph} />
    </ReactFlowProvider>
  );
}
