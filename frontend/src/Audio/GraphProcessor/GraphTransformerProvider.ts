/**
 * GraphTransformerProvider
 *
 * Compiles an ActiveGraph + binary map into a descriptor that can be
 * sent to the worklet.  The descriptor IS the "transform" — when the
 * worklet receives it, it materialises it into a real function running
 * on the audio thread.
 *
 * update(newGraph) recompiles whenever topology or params change.
 * getDescriptor() returns the latest compiled result, or null if any
 * binary is missing or the graph is empty / invalid.
 */

import type { ActiveGraph } from '@/Stores/ActiveGraphState';
import type { CompiledGraph } from './dto/CompiledGraph';
import type { GraphInput } from './dto/GraphInput';
import { process as compile } from './GraphCompiler';

export interface TransformDescriptor {
  compiledGraph: CompiledGraph;
  binaries: Record<number, Uint8Array>; // transformId → wasm binary
}

export class GraphTransformerProvider {
  private descriptor: TransformDescriptor | null = null;

  constructor(
    private graph: ActiveGraph,
    private getBinary: (transformId: number) => Uint8Array | null,
  ) {
    this.descriptor = this.compile(graph);
  }

  update(newGraph: ActiveGraph): void {
    this.graph      = newGraph;
    this.descriptor = this.compile(newGraph);
  }

  getDescriptor(): TransformDescriptor | null {
    return this.descriptor;
  }

  private compile(graph: ActiveGraph): TransformDescriptor | null {
    const input: GraphInput = {
      nodes: new Map(
        [...graph.nodes.entries()].map(([id, node]) => [
          id,
          { id, transformId: node.transformId, params: node.params },
        ]),
      ),
      edges: new Map(
        [...graph.edges.entries()].map(([id, edge]) => [
          id,
          { id, fromNodeId: edge.fromNodeId, toNodeId: edge.toNodeId },
        ]),
      ),
    };

    const result = compile(input);
    if (!result.ok) return null;

    const binaries: Record<number, Uint8Array> = {};
    for (const node of graph.nodes.values()) {
      const binary = this.getBinary(node.transformId);
      if (!binary) return null; // not all binaries loaded yet
      binaries[node.transformId] = binary;
    }

    return { compiledGraph: result.graph, binaries };
  }
}
