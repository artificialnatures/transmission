import type { Component } from 'solid-js';
import { createSignal, createEffect } from 'solid-js';
import { sendMessage, subscribeToMessage, logMessage } from './tauri-client';

const App: Component = () => {
  const [readMessage, writeMessage] = createSignal("No message.");
  logMessage("Frontend created message signal.");
  subscribeToMessage(writeMessage);
  return (
    <div>
      <p class="text-5xl">
        Here's some words =&gt; some words
      </p>
      <p class="text-5xl">
        Here's some numbers =&gt; 1234567890
      </p>
      <button class="btn btn-accent" onClick={() => sendMessage("Round trip through Tauri.")}>Ok!</button>
      <p id="message">{readMessage()}</p>
    </div>
  );
};

export default App;
