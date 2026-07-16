"use client";

import { useState, useCallback } from "react";
import { Download } from "lucide-react";
import { useDropzone } from "react-dropzone";
import { encodeImage, decodeImage } from "../lib/steg";

export default function Home() {
  const [mode, setMode] = useState<"encode" | "decode" | null>(null);
  const [file, setFile] = useState<File | null>(null);
  const [text, setText] = useState("");
  const [decoded, setDecoded] = useState("");
  const [error, setError] = useState("");

  const onDrop = useCallback((acceptedFiles: File[]) => {
    setFile(acceptedFiles[0] ?? null);
  }, []);

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    accept: {
      "image/png": [],
      "image/jpeg": [],
    },
  });

  const { getRootProps: getDecodeRootProps, getInputProps: getDecodeInputProps, isDragActive: isDecodeDragActive } = useDropzone({
    onDrop,
    accept: {
      "image/png": [],
      "image/jpeg": [],
    },
  });

  return (
    <div className="flex flex-col flex-1 items-center justify-center bg-zinc-50 font-sans dark:bg-black">
      <main className="flex flex-1 w-full max-w-3xl flex-col items-center py-20 px-16 bg-white dark:bg-black width-full">

        <h1 className="text-[6vw] text-nowrap text-center select-none">Steganography Encoder</h1>

        {error && <div className="mt-5 rounded-xl border border-red-700 bg-red-950 p-3 text-red-300">{error}</div>}

        <div className="flex mt-10">
          <button className={`px-10 py-4 text-lg rounded-l-xl border border-zinc-700 transition ${mode === "encode" ? "bg-zinc-700 text-white" : "bg-zinc-900 text-zinc-300 hover:bg-zinc-800"}`} onClick={() => { setMode("encode"); setFile(null); setError(""); }}>Encode</button>
          <button className={`px-10 py-4 text-lg rounded-r-xl border border-l-0 border-zinc-700 transition ${mode === "decode" ? "bg-zinc-700 text-white" : "bg-zinc-900 text-zinc-300 hover:bg-zinc-800"}`} onClick={() => { setMode("decode"); setFile(null); setError(""); }}>Decode</button>
        </div>

        {mode === "encode" && (
          <div className="mt-10 flex w-full flex-col gap-4">
            <textarea className="w-full resize-y rounded-xl border border-zinc-700 bg-zinc-900 p-3 text-zinc-200 placeholder-zinc-500 focus:outline-none focus:ring-2 focus:ring-zinc-600" rows={8} value={text} onChange={(e) => setText(e.target.value)} placeholder="Secret text" />

            <div {...getRootProps()} className={`border-2 border-dashed rounded-xl p-10 text-center cursor-pointer transition ${isDragActive ? "border-zinc-400 bg-zinc-800" : "border-zinc-700 bg-zinc-900 hover:bg-zinc-800"}`}>
              <input {...getInputProps()} />
              <p className="text-zinc-300">{file ? file.name : "Drag and drop an image here, or click to select"}</p>
              <p className="mt-2 text-sm text-zinc-500">Only PNG/JPEG images are supported</p>
            </div>

            <button className="px-10 py-4 text-lg rounded-xl border border-zinc-700 bg-zinc-700 text-white transition hover:bg-zinc-600 disabled:cursor-not-allowed disabled:opacity-50" disabled={!file || !text} onClick={async () => {
              try {
                setError("");

                if (!file) {
                  setError("Please select an image first.");
                  return;
                }

                const blob = await encodeImage(file, text);
                const url = URL.createObjectURL(blob);
                const a = document.createElement("a");
                a.href = url;
                a.download = "encoded.png";
                a.click();
                URL.revokeObjectURL(url);
              } catch (err) {
                setError(err instanceof Error ? err.message : "Encoding failed.");
              }
            }}>Encode</button>
          </div>
        )}

        {mode === "decode" && (
          <div className="mt-10 flex w-full flex-col gap-4">

            <div {...getDecodeRootProps()} className={`border-2 border-dashed rounded-xl p-10 text-center cursor-pointer transition ${isDecodeDragActive ? "border-zinc-400 bg-zinc-800" : "border-zinc-700 bg-zinc-900 hover:bg-zinc-800"}`}>
              <input {...getDecodeInputProps()} />
              <p className="text-zinc-300">{file ? file.name : "Drag and drop an image here, or click to select"}</p>
              <p className="mt-2 text-sm text-zinc-500">Only PNG/JPEG images are supported</p>
            </div>

            <button className="px-10 py-4 text-lg rounded-xl border border-zinc-700 bg-zinc-700 text-white transition hover:bg-zinc-600 disabled:cursor-not-allowed disabled:opacity-50" disabled={!file} onClick={async () => {
              try {
                setError("");

                if (!file) {
                  setError("Please select an image first.");
                  return;
                }

                setDecoded(await decodeImage(file));
              } catch (err) {
                setError(err instanceof Error ? err.message : "Decoding failed.");
              }
            }}>Decode</button>

            {decoded && (
              <>
                <textarea className="w-full resize-y rounded-xl border border-zinc-700 bg-zinc-900 p-3 text-zinc-200 focus:outline-none" rows={8} readOnly value={decoded} />

                <button className="w-fit px-4 py-2 text-sm rounded-lg border border-zinc-700 bg-zinc-800 text-zinc-200 hover:bg-zinc-700 transition" onClick={() => {
                  const blob = new Blob([decoded], { type: "text/plain" });
                  const url = URL.createObjectURL(blob);
                  const a = document.createElement("a");
                  a.href = url;
                  a.download = "decoded.txt";
                  a.click();
                  URL.revokeObjectURL(url);
                }}><Download size={16} /></button>
              </>
            )}

          </div>
        )}

      </main>
    </div>
  );
}
