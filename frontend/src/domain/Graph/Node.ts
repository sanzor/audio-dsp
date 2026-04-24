import type { Port } from "./Port"


export interface Node{
    id:number,
    graphId:number,
    transformId?: number | null,
    position:{x:number,y:number}
    params?: Record<string, number>,
    ports:Port[],
    createdAt:Date
}
