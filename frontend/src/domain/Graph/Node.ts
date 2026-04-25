import type { Port } from "./Port"

export type NodeType = "default" | "source" | "sink";

export interface Node{
    id:number,
    graphId:number,
    transformId?: number | null,
    position:{x:number,y:number}
    params?: Record<string, number>,
    ports:Port[],
    createdAt:Date,
    nodeType: NodeType,
}
