import {  STOP_COMMAND } from "../../Events";
import type { Command } from "./Command";


export interface StopCommand extends Command {
    kind: typeof STOP_COMMAND;
}