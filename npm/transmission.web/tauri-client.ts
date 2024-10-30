import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

export function sendMessage(message : string) {
    invoke("send_message", { message: message });
}

export function logMessage(message : string) {
    invoke("log_message", { message: message });
}

export function subscribeToMessage(writeMessage : (message : string) => void) {
    invoke("log_message", { message: "Frontend subscribing to message events." });
    listen<string>("message", (event) => {
        let logMessage = "Frontend received message: " + event.payload;
        invoke("log_message", { message: logMessage });
        writeMessage(event.payload);
    });
}