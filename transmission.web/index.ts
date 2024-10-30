export async function hello() : Promise<string> {
    const transmission = await WebAssembly.instantiateStreaming(fetch("transmission-wasm/transmission_bg.wasm"));
    const readMessage = transmission.instance.exports.read_transmission_message as () => string;
    return readMessage();
}