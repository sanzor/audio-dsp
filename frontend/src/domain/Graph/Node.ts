import type { Port } from "./Port"


export interface Node{
    id:string,
    graphId:string,
    position:{x:number,y:number}
    ports:Port[],
    createdAt:Date
}