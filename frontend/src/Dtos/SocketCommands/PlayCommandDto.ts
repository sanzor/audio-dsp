import type { PLAY_COMMAND } from "@/Events";
import type { BaseCommandDto } from "./BaseCommandDto";

export interface PlayCommandDto extends BaseCommandDto{
        command: typeof PLAY_COMMAND,
};