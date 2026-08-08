import { describe, it, expect, beforeEach } from "vitest";
import { useTransformStore } from "@/Stores/TransformStore";
import type { TransformDefinition } from "@/domain/Transform/TransformDefinition";
import {
  process as compileGraphInput,
  inputPortCountOf,
  inputPortIndexByName,
  type GraphInput,
} from "./GraphCompiler";

// ─── Cycle handling, exercised through the composite preview call path ───────
//
// composite-preview-controls.ts's togglePreview() builds a GraphInput from
// the in-progress composite canvas graph (node ids, transform ids,
// inputPortCountOf/inputPortIndexByName resolved off useTransformStore
// definitions) and hands it straight to this same compileGraphInput/process
// function before sending the result to the shared preview worklet.
//
// These tests build a GraphInput the same way (composite nodes wired through
// named ports resolved off useTransformStore) with a connection that loops
// back to an earlier node.

function singleIoDefinition(transformId: number): TransformDefinition {
  return {
    transform_id: transformId,
    name: `transform-${transformId}`,
    kind: "primitive",
    published: true,
    is_validated: false,
    ports: [
      { port_id: transformId * 10 + 1, name: "in", direction: "input", port_order: 0, kind: "program", cardinality: "single" },
      { port_id: transformId * 10 + 2, name: "out", direction: "output", port_order: 0, kind: "program", cardinality: "single" },
    ],
    params: [],
  };
}

describe("GraphCompiler cycle handling (composite preview call path)", () => {
  beforeEach(() => {
    useTransformStore.getState().clear();
    useTransformStore.getState().upsertDefinitions([1, 2, 3, 4].map(singleIoDefinition));
  });

  it("marks no back-edges and assigns no feedback buffers for an acyclic composite chain (control case)", () => {
    const graphInput: GraphInput = {
      nodes: new Map([
        [1, { id: 1, transformId: 1, params: {}, inputPortCount: inputPortCountOf(1) }],
        [2, { id: 2, transformId: 2, params: {}, inputPortCount: inputPortCountOf(2) }],
        [3, { id: 3, transformId: 3, params: {}, inputPortCount: inputPortCountOf(3) }],
      ]),
      edges: new Map([
        [0, { id: 0, fromNodeId: 1, toNodeId: 2, toPortIndex: inputPortIndexByName(2, "in") }],
        [1, { id: 1, fromNodeId: 2, toNodeId: 3, toPortIndex: inputPortIndexByName(3, "in") }],
      ]),
    };

    const result = compileGraphInput(graphInput);
    expect(result.ok).toBe(true);
    if (!result.ok) return;

    expect(result.graph.feedbackBufferIndices).toHaveLength(0);
    expect(result.graph.executionOrder.map((n) => n.nodeId)).toEqual([1, 2, 3]);
  });

  it("WORKING CASE: back-edge source with a separate downstream sink gets a real buffer that the worklet actually writes each quantum", () => {
    // 1 -> 2 -> 3 -> 1 (closes the loop) AND 3 -> 4 (node 3 also feeds a
    // downstream sink). Node 3 has a forward outgoing edge in addition to
    // its back edge, so buildNodes() gives it a real (non -1)
    // outputBufferIndex — the generated worklet code's
    // `buffers[node.outputBufferIndex].set(out_i)` branch actually fires for
    // node 3 every quantum, so the value node 1 reads back as feedback next
    // frame is node 3's real audio, not permanent silence.
    const graphInput: GraphInput = {
      nodes: new Map([
        [1, { id: 1, transformId: 1, params: {}, inputPortCount: inputPortCountOf(1) }],
        [2, { id: 2, transformId: 2, params: {}, inputPortCount: inputPortCountOf(2) }],
        [3, { id: 3, transformId: 3, params: {}, inputPortCount: inputPortCountOf(3) }],
        [4, { id: 4, transformId: 4, params: {}, inputPortCount: inputPortCountOf(4) }],
      ]),
      edges: new Map([
        [0, { id: 0, fromNodeId: 1, toNodeId: 2, toPortIndex: inputPortIndexByName(2, "in") }],
        [1, { id: 1, fromNodeId: 2, toNodeId: 3, toPortIndex: inputPortIndexByName(3, "in") }],
        [2, { id: 2, fromNodeId: 3, toNodeId: 1, toPortIndex: inputPortIndexByName(1, "in") }], // back edge, closes the loop
        [3, { id: 3, fromNodeId: 3, toNodeId: 4, toPortIndex: inputPortIndexByName(4, "in") }], // forward, node 3's "real" downstream
      ]),
    };

    const result = compileGraphInput(graphInput);
    expect(result.ok).toBe(true);
    if (!result.ok) return;

    expect(result.graph.executionOrder).toHaveLength(4);
    expect(result.graph.executionOrder.map((n) => n.nodeId)).toEqual([1, 2, 3, 4]);
    expect(result.graph.feedbackBufferIndices.length).toBeGreaterThan(0);

    const node3 = result.graph.executionOrder.find((n) => n.nodeId === 3)!;
    expect(node3.outputBufferIndex).not.toBe(-1); // real buffer slot — the worklet will write into it
    expect(result.graph.feedbackBufferIndices).toContain(node3.outputBufferIndex);

    const node1 = result.graph.executionOrder.find((n) => n.nodeId === 1)!;
    expect(node1.inputs[0]).toContainEqual({ kind: "feedback", bufferIndex: node3.outputBufferIndex });
  });

  it("FIXED: a back-edge source with NO other outgoing edge (the loop's last/terminal node) gets its output written into both its feedback buffer AND the worklet output", () => {
    // 1 -> 2 -> 3 -> 1: the simplest, most natural cyclic composite shape —
    // a chain whose last node loops back to an earlier one, with node 3
    // ALSO acting as the composite's terminal output (it has no other
    // outgoing edge). The DFS/back-edge/topo-sort phases handle this
    // correctly (asserted below), and assignOutputBuffers() reserves a real
    // buffer index for node 3 and lists it in feedbackBufferIndices, since
    // it has hasBackOut = true.
    //
    // Previously, buildNodes() set outputBufferIndex to -1 for node 3
    // purely because it has no *forward* outgoing edge, and
    // generateTransformFunction() treated outputBufferIndex === -1 and
    // !== -1 as mutually exclusive — so node 3's real output went straight
    // into the worklet's `output` accumulator (correct, it IS the audible
    // terminal node) but was never written into the reserved feedback
    // buffer, leaving node 1's feedback read permanently silent.
    //
    // Fix: outputBufferIndex is now assigned whenever hasForwardOut ||
    // hasBackOut (real buffer whenever one is needed), and a new
    // independent `writesToOutput` flag (`!hasForwardOut`) tells the
    // worklet to *additionally* sum the node into `output` when nothing
    // downstream consumes it. Both facts can be true at once for node 3
    // here.
    const graphInput: GraphInput = {
      nodes: new Map([
        [1, { id: 1, transformId: 1, params: {}, inputPortCount: inputPortCountOf(1) }],
        [2, { id: 2, transformId: 2, params: {}, inputPortCount: inputPortCountOf(2) }],
        [3, { id: 3, transformId: 3, params: {}, inputPortCount: inputPortCountOf(3) }],
      ]),
      edges: new Map([
        [0, { id: 0, fromNodeId: 1, toNodeId: 2, toPortIndex: inputPortIndexByName(2, "in") }],
        [1, { id: 1, fromNodeId: 2, toNodeId: 3, toPortIndex: inputPortIndexByName(3, "in") }],
        [2, { id: 2, fromNodeId: 3, toNodeId: 1, toPortIndex: inputPortIndexByName(1, "in") }], // back edge, closes the loop
      ]),
    };

    const result = compileGraphInput(graphInput);
    expect(result.ok).toBe(true);
    if (!result.ok) return;

    // Cycle detection / scheduling itself is correct: all 3 nodes scheduled,
    // node 1 first (its only incoming edge is a back-edge, so its forward
    // in-degree is 0).
    expect(result.graph.executionOrder).toHaveLength(3);
    expect(result.graph.executionOrder.map((n) => n.nodeId)).toEqual([1, 2, 3]);

    // A buffer IS reserved and flagged for feedback...
    expect(result.graph.feedbackBufferIndices).toHaveLength(1);
    const reservedFeedbackIndex = result.graph.feedbackBufferIndices[0];

    // ...and node 3 itself (the reservation's source) now gets that real
    // buffer index AND is flagged to also write to the worklet's output,
    // since it has no forward consumer.
    const node3 = result.graph.executionOrder.find((n) => n.nodeId === 3)!;
    expect(node3.outputBufferIndex).not.toBe(-1);
    expect(node3.outputBufferIndex).toBe(reservedFeedbackIndex);
    expect(node3.writesToOutput).toBe(true);

    // Node 1's feedback read points at the same reserved index that node
    // 3's compiled descriptor now actually writes to every quantum.
    const node1 = result.graph.executionOrder.find((n) => n.nodeId === 1)!;
    expect(node1.inputs[0]).toContainEqual({ kind: "feedback", bufferIndex: reservedFeedbackIndex });
  });

  it("a normal interior node (forward out, no back edge) writes only to its buffer, not to output", () => {
    // Sanity check on the other three outputBufferIndex/writesToOutput
    // combinations, using node 2 from the control-case chain (1 -> 2 -> 3):
    // node 2 has a forward outgoing edge and no back edge.
    const graphInput: GraphInput = {
      nodes: new Map([
        [1, { id: 1, transformId: 1, params: {}, inputPortCount: inputPortCountOf(1) }],
        [2, { id: 2, transformId: 2, params: {}, inputPortCount: inputPortCountOf(2) }],
        [3, { id: 3, transformId: 3, params: {}, inputPortCount: inputPortCountOf(3) }],
      ]),
      edges: new Map([
        [0, { id: 0, fromNodeId: 1, toNodeId: 2, toPortIndex: inputPortIndexByName(2, "in") }],
        [1, { id: 1, fromNodeId: 2, toNodeId: 3, toPortIndex: inputPortIndexByName(3, "in") }],
      ]),
    };

    const result = compileGraphInput(graphInput);
    expect(result.ok).toBe(true);
    if (!result.ok) return;

    const node2 = result.graph.executionOrder.find((n) => n.nodeId === 2)!;
    expect(node2.outputBufferIndex).not.toBe(-1);
    expect(node2.writesToOutput).toBe(false);

    const node3 = result.graph.executionOrder.find((n) => n.nodeId === 3)!;
    expect(node3.outputBufferIndex).toBe(-1);
    expect(node3.writesToOutput).toBe(true);
  });
});

// ─── Worklet code generation ───────────────────────────────────────────────
//
// generateTransformFunction (graph-worklet.js) turns a CompiledGraph into a
// flat sequence of statements. Rather than standing up a real AudioWorklet
// runtime, assert directly on the generated source string for the buffer
// write / output write decision per node — graph-worklet.js is guarded so it
// can be imported under plain Node (see the AudioWorkletProcessor presence
// check at the bottom of that file).

describe("generateTransformFunction write-through codegen", () => {
  it("emits both a buffer write and an output write for a node with a real buffer AND no forward consumer", async () => {
    const { generateTransformFunction } = await import("../worklet/graph-worklet.js");

    const compiledGraph = {
      executionOrder: [
        {
          nodeId: 3,
          transformId: 3,
          params: [],
          inputs: [[]],
          outputBufferIndex: 0,
          writesToOutput: true,
        },
      ],
      bufferCount: 1,
      feedbackBufferIndices: [0],
    };

    const source = generateTransformFunction(compiledGraph).toString();

    expect(source).toContain("buffers[0].set(out_0)");
    expect(source).toContain("output = addAll(output, out_0)");
  });

  it("emits only a buffer write for a node with a forward consumer and no back edge", async () => {
    const { generateTransformFunction } = await import("../worklet/graph-worklet.js");

    const compiledGraph = {
      executionOrder: [
        {
          nodeId: 2,
          transformId: 2,
          params: [],
          inputs: [[]],
          outputBufferIndex: 0,
          writesToOutput: false,
        },
      ],
      bufferCount: 1,
      feedbackBufferIndices: [],
    };

    const source = generateTransformFunction(compiledGraph).toString();

    expect(source).toContain("buffers[0].set(out_0)");
    expect(source).not.toContain("output = addAll(output, out_0)");
  });

  it("emits only an output write for a true sink node (no buffer, no back edge)", async () => {
    const { generateTransformFunction } = await import("../worklet/graph-worklet.js");

    const compiledGraph = {
      executionOrder: [
        {
          nodeId: 3,
          transformId: 3,
          params: [],
          inputs: [[]],
          outputBufferIndex: -1,
          writesToOutput: true,
        },
      ],
      bufferCount: 0,
      feedbackBufferIndices: [],
    };

    const source = generateTransformFunction(compiledGraph).toString();

    expect(source).not.toContain(".set(out_0)");
    expect(source).toContain("output = addAll(output, out_0)");
  });
});
