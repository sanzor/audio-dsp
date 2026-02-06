import type { PAUSE_COMMAND } from "@/Events";
import type { BaseCommandDto } from "./BaseCommandDto";

export interface PauseCommandDto extends BaseCommandDto{
        command: typeof PAUSE_COMMAND,
};