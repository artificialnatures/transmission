import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

export function sendMessage(message : string) {
    invoke('send_message', { message });
}

export function subscribeToMessage(message : string) {
    listen<string>('message', (event) => {
        message = event.payload;
    });
}