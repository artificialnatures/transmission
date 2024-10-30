import { assertEquals } from "jsr:@std/assert";
import { read_transmission_message } from "./wasm/transmission.js";

Deno.test(function canLoadWasm() {
  assertEquals(read_transmission_message(), "transmission");
});
