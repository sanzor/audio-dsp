/* eslint-disable @typescript-eslint/no-explicit-any */
// MessageService.ts


import type { EventBus } from "@/EventBus.ts";
import { 
  COMMAND_RESULT
} from "../Constants.ts";
import { PAUSE_COMMAND, PLAY_COMMAND, SEEK_COMMAND, STOP_COMMAND } from "../Events";

export class MessageService {

  private eventBus: EventBus;

  constructor(eventBus: EventBus) {
    this.eventBus = eventBus;
  }
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  public handleMessage(data: any) {
    void this.eventBus;
    switch (data.kind) {
     
      case COMMAND_RESULT:
        this.handleCommandResult(data);
        break;
      default:
        console.warn("Unknown message kind:", data.kind);
    }
  }



  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  private handleCommandResult(data: any) {
    switch (data.command) {
      case PLAY_COMMAND:
        this.handlePlayCommandResult(data);
        break;

      case PAUSE_COMMAND:
        this.handlePauseCommandResult(data);
        break;

      case SEEK_COMMAND:
        this.handleSeekCommandResult(data.result);
        break;

      case STOP_COMMAND:
        this.handleStopCommandResult(data.result);
        break;
      
    
      default:
        console.warn("Unknown command result:", data.command);
    }
  }


  private handlePlayCommandResult(data: any) {
    void data;

  }

  private handlePauseCommandResult(data: any) {
    void data;
  
  }

  private handleStopCommandResult(data: any) {
    console.log("Newest messages received:", data.messages);
   // this.eventBus.publishEvent(GET_NEWEST_MESSAGES_COMMAND_RESULT, data.messages);
  }

  private handleSeekCommandResult(data: any) {
    void data;
   
  }
}
