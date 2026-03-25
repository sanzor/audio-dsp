import type { Port } from "./Port"


export interface Node{
    id:number,
    graphId:number,
    position:{x:number,y:number}
    ports:Port[],
    createdAt:Date
}