export interface Edge{
    id:number,
    graphId:number,
    fromNodeId:number,
    toNodeId:number,
    fromPortId?: number | null,
    toPortId?: number | null,
    to:string
}
