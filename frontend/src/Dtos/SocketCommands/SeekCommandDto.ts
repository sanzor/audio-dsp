import type {  SEEK_COMMAND } from "@/Events";
import type { BaseCommandDto } from "./BaseCommandDto";

export interface SeekCommandDto extends BaseCommandDto{
        command: typeof SEEK_COMMAND,
};