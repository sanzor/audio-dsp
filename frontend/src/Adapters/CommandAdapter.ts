


import type { Command } from "@/Domain/Commands/Command";
import { PLAY_COMMAND, PAUSE_COMMAND,
        SEEK_COMMAND,
        STOP_COMMAND,
        } from "../Events";
import type { PlayCommand } from "@/Domain/Commands/PlayCommand";
import type { PlayCommandDto } from "@/Dtos/SocketCommands/PlayCommandDto";
import type { PauseCommand } from "@/Domain/Commands/PauseCommand";
import type { PauseCommandDto } from "@/Dtos/SocketCommands/PauseCommandDto";
import type { SeekCommand } from "@/Domain/Commands/SeekCommand";
import type { SeekCommandDto } from "@/Dtos/SocketCommands/SeekCommandDto";
import type { StopCommand } from "@/Domain/Commands/StopCommand";
import type { BaseCommandDto } from "@/Dtos/SocketCommands/BaseCommandDto";
import type { StopCommandDto } from "@/Dtos/SocketCommands/StopCommandDto";


export function createSocketCommand(data:Command):BaseCommandDto|null{
    const command=innerCreateCommand(data);
    return command;
}
function innerCreateCommand(data:Command): BaseCommandDto|null{
    switch(data.kind){
        case PLAY_COMMAND:  
            if(isPlayCommand(data)) 
                return create_command_play();
            break;
        case PAUSE_COMMAND : 
            if(isPauseCommand(data))
                return create_command_pause();
            break;
        case SEEK_COMMAND:
            if(isSeekCommand(data))
                return create_command_seek();
            break;
        case STOP_COMMAND :
            if(isStopCommand(data))
                return create_command_stop();
            break;
        
           
        default:return null;
    }
    return null;
}
function create_command_play():PlayCommandDto{
    const  message:PlayCommandDto={
        command:PLAY_COMMAND
    };
    return message;
}
 function create_command_pause():PauseCommandDto{
   
    const message:PauseCommandDto={
        command:PAUSE_COMMAND
    }
    return message;
}

function create_command_seek():SeekCommandDto{
   
    const message:SeekCommandDto={
        command:SEEK_COMMAND
    }
    return message;
    
}
function create_command_stop():StopCommandDto{
    const  message:StopCommandDto={
        command:STOP_COMMAND
    }
    return message;
}



function isPauseCommand(command: Command): command is PauseCommand {
    return command.kind === PAUSE_COMMAND;
}

function isPlayCommand(command: Command): command is PlayCommand {
    return command.kind === PLAY_COMMAND;
}

function isSeekCommand(command: Command): command is SeekCommand {
    return command.kind === SEEK_COMMAND;
}

function isStopCommand(command: Command): command is StopCommand {
    return command.kind === STOP_COMMAND;
}
