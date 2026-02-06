import { SOCKET_COMMAND } from "./Constants";
import type { Command } from "./Domain/Commands/Command";


  export type EventCallback<T = unknown> = (event: CustomEvent<T>) => void;

  export class EventBus {
    private eventBus: EventTarget;
    constructor(){
      this.eventBus=new EventTarget();
    }
    subscribe<T = unknown>(event: string, callback: EventCallback<T>) {
      console.log(`subscribing to event ${event}`);
      this.eventBus.addEventListener(event,callback as EventListener);
    }

    unsubscribe<T = unknown>(event: string, callback: EventCallback<T>) {
      console.log(`unsubscribing from event ${event}`);
      this.eventBus.removeEventListener(event,callback as EventListener);
    }

    publishEvent<T = unknown>(event: string, data: T) {
      console.log(`publishing event ${event}`);
      const customEvent=new CustomEvent(event,{detail:data});
      this.eventBus.dispatchEvent(customEvent);
    }
    publishCommand(command:Command){
      console.log(`publishing command ${JSON.stringify(command)}`);
      const customEvent=new CustomEvent(SOCKET_COMMAND,{detail:command});
      return this.eventBus.dispatchEvent(customEvent);
    }
  }


  export default new EventBus();
