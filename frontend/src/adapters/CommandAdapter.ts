import type { Command } from "@/domain/commands/Command";
import { PLAY_COMMAND, PAUSE_COMMAND, SEEK_COMMAND, STOP_COMMAND } from "../Events";
import type { PlayCommand } from "@/domain/commands/PlayCommand";
import type { PauseCommand } from "@/domain/commands/PauseCommand";
import type { SeekCommand } from "@/domain/commands/SeekCommand";
import type { StopCommand } from "@/domain/commands/StopCommand";

// ─── Socket Command Types ────────────────────────────────────────────────────

interface BaseCommandDto {
  command: string
}
interface PlayCommandDto extends BaseCommandDto {
  command: typeof PLAY_COMMAND
}
interface PauseCommandDto extends BaseCommandDto {
  command: typeof PAUSE_COMMAND
}
interface SeekCommandDto extends BaseCommandDto {
  command: typeof SEEK_COMMAND
}
interface StopCommandDto extends BaseCommandDto {
  command: typeof STOP_COMMAND
}

type SocketCommandDto =
  | PlayCommandDto
  | PauseCommandDto
  | SeekCommandDto
  | StopCommandDto

// ─── Adapter ─────────────────────────────────────────────────────────────────

export function createSocketCommand(data: Command): SocketCommandDto | null {
  return innerCreateCommand(data);
}

function innerCreateCommand(data: Command): SocketCommandDto | null {
  switch (data.kind) {
    case PLAY_COMMAND:
      if (isPlayCommand(data)) return { command: PLAY_COMMAND };
      break;
    case PAUSE_COMMAND:
      if (isPauseCommand(data)) return { command: PAUSE_COMMAND };
      break;
    case SEEK_COMMAND:
      if (isSeekCommand(data)) return { command: SEEK_COMMAND };
      break;
    case STOP_COMMAND:
      if (isStopCommand(data)) return { command: STOP_COMMAND };
      break;
  }
  return null;
}

function isPauseCommand(command: Command): command is PauseCommand { return command.kind === PAUSE_COMMAND; }
function isPlayCommand(command: Command): command is PlayCommand { return command.kind === PLAY_COMMAND; }
function isSeekCommand(command: Command): command is SeekCommand { return command.kind === SEEK_COMMAND; }
function isStopCommand(command: Command): command is StopCommand { return command.kind === STOP_COMMAND; }
