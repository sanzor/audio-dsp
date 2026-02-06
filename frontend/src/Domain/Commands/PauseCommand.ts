import { PLAY_COMMAND } from "../../Events";
import type { Command } from "./Command";


export interface PauseCommand extends Command {
    kind: typeof PLAY_COMMAND;
}