import type {   STOP_COMMAND } from "@/Events";
import type { BaseCommandDto } from "./BaseCommandDto";

export interface StopCommandDto extends BaseCommandDto{
        command: typeof STOP_COMMAND,
};