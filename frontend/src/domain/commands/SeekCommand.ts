import {  SEEK_COMMAND } from "../../Events";
import type { Command } from "./Command";


export interface SeekCommand extends Command {
    kind: typeof SEEK_COMMAND;
}