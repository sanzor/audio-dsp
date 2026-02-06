
import type { EventBus } from "@/EventBus.ts";
import { SOCKET_CLOSED, SOCKET_COMMAND } from "../Constants.ts";
import { MessageService } from "./MessageService";
import { close, connect, onClose, onMessage, send } from "./Websocket";
import type { Command } from "@/Domain/Commands/Command.ts";
import { createSocketCommand } from "@/Adapters/CommandAdapter";

export class WebSocketController {
  private socket: WebSocket | null = null;
  private isRetrying: boolean = false;
  private static instance: WebSocketController | null = null;
  private messageQueue: string[] = [];

  public static getInstance(eventBus: EventBus,messageService:MessageService): WebSocketController {
    if (!WebSocketController.instance) {
      console.log("WebSocketController: Creating singleton instance...");
      WebSocketController.instance = new WebSocketController(eventBus,messageService);
    }
    return WebSocketController.instance;
  }

  constructor(private eventBus: EventBus,private messageService:MessageService) {
    
    this.eventBus.subscribe(SOCKET_COMMAND, this.handleSocketCommand);
    this.eventBus.subscribe(SOCKET_CLOSED, this.handleSocketClosed);
  }




  public connect(url: string): void {
    if (this.socket) {
      if (this.socket.readyState === WebSocket.OPEN) {
        console.warn("WebSocketController: WebSocket is already connected.");
        return;
      }
      if (this.socket.readyState === WebSocket.CONNECTING) {
        console.warn("WebSocketController: WebSocket connection is already in progress.");
        return;
      }
    }

    console.log(`WebSocketController: Connecting to ${url}`);

    try {
      this.socket = connect(url);

      onMessage(this.socket, this.handleMessage);
      onClose(this.socket, this.handleClose);

      this.socket.onopen = () => {
        console.log("WebSocketController: Connection opened.");
        this.retryCount = 0; // Reset retry count
        this.isRetrying = false;
        while (this.messageQueue.length > 0) {
          const message = this.messageQueue.shift();
          if (message) {
            this.sendMessage(message);
          }
        }
        this.eventBus.publishEvent("WEBSOCKET_CONNECTED", {});
      };

      this.socket.onerror = (error) => {
        console.error("WebSocketController: Connection error:", error);
        if (!this.isRetrying) {
          this.eventBus.publishEvent("WEBSOCKET_CONNECTION_FAILED", error);
          this.retryConnection(url);
        }
      };
    } catch (error) {
      console.error("WebSocketController: Failed to connect:", error);
      this.eventBus.publishEvent("WEBSOCKET_CONNECTION_FAILED", error);
    }
  }

  public disconnect(): void {
    if (!this.socket) {
      console.warn("WebSocketController: WebSocket is not connected or already closed.");
      return;
    }

    if (this.socket.readyState === WebSocket.CLOSED) {
      console.warn("WebSocketController: WebSocket is already closed.");
      this.socket = null;
      return;
    }

    console.log("WebSocketController: Disconnecting WebSocket...");
     close();
    this.socket = null;
    this.eventBus.publishEvent("WEBSOCKET_DISCONNECTED", {});
  }

  private retryCount = 0;

  private retryConnection(url: string): void {
    this.isRetrying = true;
    const retryDelay = Math.min(1000 * Math.pow(2, this.retryCount), 30000); // Exponential backoff, max 30s
    console.log(`WebSocketController: Retrying connection in ${retryDelay / 1000} seconds...`);
  
    setTimeout(() => {
      this.retryCount++;
      this.connect(url);
    }, retryDelay);
  }

  private handleMessage = (message: MessageEvent): void => {
    console.log("WebSocketController: Message received:", message.data);

    try {
      const parsedData = JSON.parse(message.data);
      this.messageService.handleMessage(parsedData);
    } catch (error) {
      console.error("WebSocketController: Failed to parse message:", error);
    }
  };

  private handleClose = (): void => {
    console.log("WebSocketController: Connection closed.");
    this.socket = null;
    this.eventBus.publishEvent("WEBSOCKET_DISCONNECTED", {});
  };

  private handleSocketCommand = (event: CustomEvent): void => {
    const command: Command = event.detail;
    console.log(event.detail);
    const payload = JSON.stringify(createSocketCommand(command));

    console.log(payload);
    if (!this.socket || this.socket.readyState !== WebSocket.OPEN) {
      console.warn("WebSocketController: WebSocket is not connected. Queuing message:", payload);
      this.messageQueue.push(payload); // Queue the message
      return;
    }

    this.sendMessage(payload);
  };
  private sendMessage(payload: string): void {
    if (this.socket && this.socket.readyState === WebSocket.OPEN) {
      send(payload);
    } else {
      console.error("WebSocketController: Cannot send message, WebSocket is not open:", payload);
    }
  }
  private handleSocketClosed = (): void => {
    console.log("WebSocketController: Closing connection via EventBus.");
    this.disconnect();
  };
}
