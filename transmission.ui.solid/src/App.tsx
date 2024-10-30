import type { Component } from 'solid-js';
import { createSignal } from 'solid-js';

const App: Component = () => {
  const [readMessage, writeMessage] = createSignal("No message.");
  return (
    <div>
      <p class="text-5xl">
        Here's some words =&gt; some words
      </p>
      <p class="text-5xl">
        Here's some numbers =&gt; 1234567890
      </p>
      <button class="btn btn-accent" onClick={() => writeMessage("Message from frontend.")}>Ok!</button>
      <p id="message">{readMessage()}</p>
    </div>
  );
};

export default App;
