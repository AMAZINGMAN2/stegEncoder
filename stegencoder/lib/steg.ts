import init, { encrypt, decrypt } from "../wasm/stegEncoder";

let initialized = false;

// makes sure that the rust file is initialised before it is used
export async function initSteg() {
  if (!initialized) {
    await init();
    initialized = true;
  }
}

export async function encodeImage(file: File, text: string): Promise<Blob> {
  const bytes = new Uint8Array(await file.arrayBuffer());

  const encoded = encrypt(text, bytes);

  const buffer = new ArrayBuffer(encoded.byteLength);
  new Uint8Array(buffer).set(encoded);

  return new Blob([buffer], { type: "image/png" });
}

// the rust decode function
export async function decodeImage(file: File): Promise<string> {
  await initSteg();

  const bytes = new Uint8Array(await file.arrayBuffer());
  return decrypt(bytes);
}
