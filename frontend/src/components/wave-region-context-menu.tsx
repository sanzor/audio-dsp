import { ContextMenuContent, ContextMenuTrigger } from "@radix-ui/react-context-menu"
import { ContextMenu, ContextMenuItem } from "./ui/context-menu"

export interface WaveRegionContextMenuProps{
    trackId:string,
    regionSetId:string,
    regionId:string,
    onEdit:(regionId:string,regionSetId:string,trackId:string)=>void,
    onDelete:(regionId:string,regionSetId:string,trackId:string)=>void
    onDetails:(regionId:string,regionSetId:string,trackId:string)=>void
    onRename:(regionId:string,regionSetId:string,trackId:string)=>void
}

export function WaveRegionContextMenu({onDelete,onDetails,onEdit,onRename,trackId,regionSetId,regionId}:WaveRegionContextMenuProps){
    return (
        <ContextMenu>
            <ContextMenuTrigger>
            </ContextMenuTrigger>
            <ContextMenuContent>
                <ContextMenuItem onClick={()=>onEdit(regionId,regionSetId,trackId)}>Edit</ContextMenuItem>
                <ContextMenuItem onClick={() => onDetails(regionId,regionSetId,trackId)}>Details</ContextMenuItem>
                <ContextMenuItem onClick={() => onDelete(regionId,regionSetId,trackId)}>Delete</ContextMenuItem>
                <ContextMenuItem onClick={() => onRename(regionId,regionSetId,trackId)}>Rename</ContextMenuItem>
            </ContextMenuContent>
        </ContextMenu>
    )
}