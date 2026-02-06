import type { ABuffer } from "@/Domain/ABuffer";
import type { TrackInfo } from "@/Domain/Track/TrackInfo";

export interface RawTrack {
  info: TrackInfo
  data: ABuffer
}