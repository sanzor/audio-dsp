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
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import "reactflow/dist/style.css";
import { useUIStore } from "@/Stores/UIStore";
import { useGraphStore } from "@/Stores/GraphStore";
import { useRegionStore } from "@/Stores/RegionStore";
import { useRegionSetStore } from "@/Stores/RegionSetStore";
import { useTransformStore } from "@/Stores/TransformStore";
import { useAudioEffectsStore } from "@/Stores/AudioEffectsStore";
import type { NodeType } from "@/domain/Graph/Node";
import type { TransformDefinition } from "@/domain/Transform/Transform";
import { CanvasToolbar } from "./canvas-toolbar";
import { useGraphController } from "@/controllers/GraphController";
import { SourceNode } from "./source-node";
import { SinkNode } from "./sink-node";
import { useActiveGraphId } from "@/hooks/graphs/useActiveGraphId";
import { NodeDetailsModal } from "../modals/graph/node-details-modal";
import { apiGetTransformDefinition } from "@/Services/TransformService";
import { useCanvasRuntime } from "@/audio/hooks/useCanvasRuntime";
import { CompileOutputModal } from "./compile-output-modal";
import { SaveCompileStatusOverlay, type SaveCompileStatusState } from "./save-compile-status-overlay";


const NODE_TYPES = { source: SourceNode, sink: SinkNode };
const COMPILE_AFTER_SAVE_TIMEOUT_MS = 8000;

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

// ─── Canvas ───────────────────────────────────────────────────────────────────

function CanvasInner() {
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);
  const [saveProgress, setSaveProgress] = useState(0);
  const [saveState, setSaveState] = useState<"hidden" | "saving" | "success" | "error">("hidden");
  const [saveCompileState, setSaveCompileState] = useState<SaveCompileStatusState>("hidden");
  const [saveCompileMessage, setSaveCompileMessage] = useState<string | null>(null);
  const saveProgressTimerRef = useRef<number | null>(null);
  const saveHideTimerRef = useRef<number | null>(null);
  const compileWaitTimerRef = useRef<number | null>(null);
  const awaitingCompileAfterSaveRef = useRef(false);
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
  const openModal = useUIStore((s) => s.openModal);
  const closeModal = useUIStore((s) => s.closeModal);
  const modalState = useUIStore((s) => s.modalState);
  const activeSelection = useUIStore((s) => s.activeSelection);
  const region = useRegionStore((s) =>
    activeSelection.regionId != null ? s.regions.get(activeSelection.regionId) : undefined
  );
  const regionSet = useRegionSetStore((s) =>
    activeSelection.regionSetId != null ? s.regionSets.get(activeSelection.regionSetId) : undefined
  );
  const regionId = activeSelection.regionId;
  const canDropTransform =
    region != null &&
    regionSet != null &&
    region.regionSetId === regionSet.id &&
    regionSet.region_ids.includes(region.regionId);

  const activeGraphId = useActiveGraphId();
  const activeGraphVersion = useGraphStore((s) =>
    activeGraphId != null ? s.graphs.get(activeGraphId)?.updatedAt?.getTime() : undefined
  );

  const summaries = useTransformStore((s) => s.summaries);
  const definitions = useTransformStore((s) => s.definitions);
  const upsertDefinition = useTransformStore((s) => s.upsertDefinition);
  const effectsEnabled = useAudioEffectsStore((s) => s.effectsEnabled);
  const setEffectsEnabled = useAudioEffectsStore((s) => s.setEffectsEnabled);
  const workletConnected = useAudioEffectsStore((s) => s.workletConnected);
  const runtimeStatus = useAudioEffectsStore((s) => s.runtimeStatus);
  const runtimeMessage = useAudioEffectsStore((s) => s.runtimeMessage);
  const graphPlaybackState = useAudioEffectsStore((s) => s.graphPlaybackState);
  const isGraphPlayable = useAudioEffectsStore((s) => s.isGraphPlayable);
  const compileModalOpen = useAudioEffectsStore((s) => s.compileModalOpen);
  const compileRun = useAudioEffectsStore((s) => s.compileRun);
  const setCompileModalOpen = useAudioEffectsStore((s) => s.setCompileModalOpen);
  const clearCompileRun = useAudioEffectsStore((s) => s.clearCompileRun);
  const graphController = useGraphController();
  const isGraphLive = activeGraphId != null && effectsEnabled;
  const nodeDetailsModalState = modalState?.type === "nodeDetails" ? modalState : null;
  const nodeDetailsTarget = useMemo(() => {
    if (!nodeDetailsModalState?.nodeId) return null;
    return nodes.find((node) => (node.data.nodeId as number | undefined) === nodeDetailsModalState.nodeId) ?? null;
  }, [nodeDetailsModalState, nodes]);

  const clearSaveTimers = useCallback(() => {
    if (saveProgressTimerRef.current != null) {
      window.clearInterval(saveProgressTimerRef.current);
      saveProgressTimerRef.current = null;
    }
    if (saveHideTimerRef.current != null) {
      window.clearTimeout(saveHideTimerRef.current);
      saveHideTimerRef.current = null;
    }
  }, []);

  const clearCompileWaitTimer = useCallback(() => {
    if (compileWaitTimerRef.current != null) {
      window.clearTimeout(compileWaitTimerRef.current);
      compileWaitTimerRef.current = null;
    }
  }, []);

  useEffect(() => clearSaveTimers, [clearSaveTimers]);
  useEffect(() => clearCompileWaitTimer, [clearCompileWaitTimer]);

  const nextCanvasTempId = useCallback(() => {
    const id = nextCanvasTempIdRef.current;
    nextCanvasTempIdRef.current -= 1;
    return id;
  }, []);

  useEffect(() => {
    const graph = activeGraphId != null
      ? useGraphStore.getState().getGraph(activeGraphId)
      : undefined;

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
  }, [regionId, activeGraphId, activeGraphVersion, setNodes, setEdges, summaries]);

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
      setEdges((es) =>
        addEdge(
          {
            ...connection,
            id: String(nextCanvasTempId()),
          },
          es
        )
      ),
    [setEdges, nextCanvasTempId]
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
    (e: React.DragEvent) => {
      if (!canDropTransform) return;
      e.preventDefault();
      const raw = e.dataTransfer.getData("application/transform");
      if (!raw) return;

      const { transformId, name } = JSON.parse(raw) as { transformId: number; name: string };
      const position = screenToFlowPosition({ x: e.clientX, y: e.clientY });
      const tempNodeId = nextCanvasTempId();
      const definition = definitions.get(transformId);

      const node: Node = {
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
      };

      setNodes((ns) => [...ns, node]);

      if (!definition) {
        void apiGetTransformDefinition(transformId)
          .then((fetched) => {
            upsertDefinition(fetched);
            setNodes((current) =>
              current.map((existingNode) => {
                if ((existingNode.data.nodeId as number | undefined) !== tempNodeId) {
                  return existingNode;
                }
                const currentParams = (existingNode.data.params as Record<string, number> | undefined) ?? {};
                if (Object.keys(currentParams).length > 0) {
                  return existingNode;
                }
                return {
                  ...existingNode,
                  data: {
                    ...existingNode.data,
                    params: defaultParamsForTransform(fetched),
                  },
                };
              })
            );
          })
          .catch((error) => {
            console.error("Failed to load transform definition:", error);
          });
      }
    },
    [canDropTransform, screenToFlowPosition, setNodes, nextCanvasTempId, definitions, upsertDefinition]
  );

  // ─── Toolbar handlers ───────────────────────────────────────────────────────

  const { compileNow, activate, canActivate } = useCanvasRuntime(activeGraphId, nodes, edges);

  useEffect(() => {
    if (!awaitingCompileAfterSaveRef.current) {
      if (saveCompileState === "error" && graphPlaybackState.compiled) {
        setSaveCompileState("hidden");
        setSaveCompileMessage(null);
      }
      return;
    }

    if (graphPlaybackState.compiled) {
      awaitingCompileAfterSaveRef.current = false;
      clearCompileWaitTimer();
      setSaveCompileState("hidden");
      setSaveCompileMessage(null);
      return;
    }

    if (runtimeStatus === "hydrating") {
      setSaveCompileState("waiting");
      setSaveCompileMessage(runtimeMessage ?? "Fetching transform binaries...");
      return;
    }

    if (runtimeStatus === "error") {
      awaitingCompileAfterSaveRef.current = false;
      clearCompileWaitTimer();
      setSaveCompileState("error");
      setSaveCompileMessage(runtimeMessage ?? "Compile failed after save.");
      return;
    }

    if (runtimeStatus === "idle") {
      awaitingCompileAfterSaveRef.current = false;
      clearCompileWaitTimer();
      setSaveCompileState("error");
      setSaveCompileMessage(runtimeMessage ?? "Compile did not produce a runnable graph.");
    }
  }, [
    clearCompileWaitTimer,
    graphPlaybackState.compiled,
    runtimeMessage,
    runtimeStatus,
    saveCompileState,
  ]);

  const handleSave = useCallback(async () => {
    if (activeGraphId == null || saveState === "saving") return;

    clearSaveTimers();
    setSaveState("saving");
    setSaveProgress(8);
    saveProgressTimerRef.current = window.setInterval(() => {
      setSaveProgress((current) => {
        const remaining = 94 - current;
        if (remaining <= 0) return current;
        return current + Math.max(1, remaining * 0.18);
      });
    }, 140);

    try {
      await graphController.handleSaveGraph(activeGraphId, nodes, edges);
      awaitingCompileAfterSaveRef.current = true;
      clearCompileWaitTimer();
      setSaveCompileState("waiting");
      setSaveCompileMessage("Checking saved graph...");
      compileNow();
      compileWaitTimerRef.current = window.setTimeout(() => {
        if (!awaitingCompileAfterSaveRef.current) return;
        awaitingCompileAfterSaveRef.current = false;
        setSaveCompileState("error");
        setSaveCompileMessage("Compile timed out while waiting for transform binaries.");
      }, COMPILE_AFTER_SAVE_TIMEOUT_MS);
      clearSaveTimers();
      setSaveState("success");
      setSaveProgress(100);
      saveHideTimerRef.current = window.setTimeout(() => {
        setSaveState("hidden");
        setSaveProgress(0);
        saveHideTimerRef.current = null;
      }, 550);
    } catch (error) {
      clearSaveTimers();
      setSaveState("error");
      setSaveProgress(100);
      saveHideTimerRef.current = window.setTimeout(() => {
        setSaveState("hidden");
        setSaveProgress(0);
        saveHideTimerRef.current = null;
      }, 1800);
      throw error;
    }
  }, [activeGraphId, nodes, edges, graphController, saveState, clearCompileWaitTimer, clearSaveTimers, compileNow]);

  const handleFitView = useCallback(() => {
    fitView({ padding: 0.2 });
  }, [fitView]);

  const handleClearNodes = useCallback(() => {
    if (activeGraphId == null) return;
    graphController.handleClearGraphNodes(activeGraphId);
  }, [activeGraphId, graphController]);

  const handleRename = useCallback(() => {
    if (activeGraphId == null) return;
    graphController.handleRenameGraph(activeGraphId);
  }, [activeGraphId, graphController]);

  const handleDelete = useCallback(async () => {
    if (activeGraphId == null) return;
    await graphController.handleDeleteGraph(activeGraphId);
  }, [activeGraphId, graphController]);

  const handleCopy = useCallback(() => {
    if (activeGraphId == null) return;
    graphController.handleCopyGraph(activeGraphId);
  }, [activeGraphId, graphController]);

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
        selectedGraphId={activeGraphId}
        hasClearableNodes={hasClearableNodes}
        isSaving={saveState === "saving"}
        effectsEnabled={isGraphLive}
        graphPlaybackState={graphPlaybackState}
        canActivate={canActivate}
        onCompile={compileNow}
        onActivate={activate}
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
          nodes={nodes}
          edges={edges}
          nodeTypes={nodeTypes}
          onNodesChange={onNodesChangeSafe}
          onEdgesChange={onEdgesChange}
          onConnect={onConnect}
          onReconnect={onReconnect}
          onNodeDoubleClick={onNodeDoubleClick}
          onPaneClick={clearCanvasSelection}
          edgesUpdatable
          deleteKeyCode={["Backspace", "Delete"]}
          fitView
        >
          <Background variant={BackgroundVariant.Lines} gap={20} color="rgba(255,255,255,0.03)" />
        </ReactFlow>
        <div
          className="pointer-events-none absolute right-3 top-3 z-20 max-w-sm rounded-md border px-3 py-2 text-xs"
          style={{
            background: "rgba(15, 23, 42, 0.9)",
            borderColor:
              activeGraphId == null
                ? "rgba(255,255,255,0.12)"
                : isGraphPlayable()
                  ? "rgba(74, 222, 128, 0.45)"
                  : "rgba(248, 113, 113, 0.45)",
            color: "var(--text-main)",
          }}
        >
          <div className="font-medium">
            Runtime: {runtimeStatus}
          </div>
          <div>Worklet: {workletConnected ? "connected" : "disconnected"}</div>
          <div>Effects: {effectsEnabled ? "enabled" : "bypassed"}</div>
          <div>Compiled: {graphPlaybackState.compiled ? "yes" : "no"}</div>
          <div>Playable: {isGraphPlayable() ? "yes" : "no"}</div>
          {runtimeMessage && <div>{runtimeMessage}</div>}
        </div>
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
            setNodes((current) =>
              current.map((node) =>
                (node.data.nodeId as number | undefined) === nodeId
                  ? {
                      ...node,
                      data: {
                        ...node.data,
                        params,
                      },
                    }
                  : node
              )
            );
          }}
        />
      )}
      <CompileOutputModal
        open={compileModalOpen}
        run={compileRun}
        onClear={clearCompileRun}
        onOpenChange={setCompileModalOpen}
      />
    </div>
  );
}

export function CanvasPanel() {
  return (
    <ReactFlowProvider>
      <CanvasInner />
    </ReactFlowProvider>
  );
}
