import type { TrackInfo } from "@/Domain/Track/TrackInfo"

export interface CreateTrackResult{
    track_id:string
    track_info:TrackInfo
}