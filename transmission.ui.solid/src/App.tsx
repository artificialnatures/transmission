import type { Component } from 'solid-js';
import { sendMessage, subscribeToMessage } from './tauri-client';

const App: Component = () => {
  let message = 'No message.';
  subscribeToMessage(message);
  return (
    <div>
      <p class="text-5xl">
        Here's some words =&gt; some words
      </p>
      <p class="text-5xl">
        Here's some numbers =&gt; 1234567890
      </p>
      <button class="btn btn-accent" onClick={() => sendMessage('Round trip through Tauri.')}>Ok!</button>
      <p id="message">{message}</p>
    </div>
  );
};

export default App;
