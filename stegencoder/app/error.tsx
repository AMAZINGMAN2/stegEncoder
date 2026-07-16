"use client";

export default function Error({ reset }: { reset: () => void }) {
  return (
    <div className="flex h-screen items-center justify-center bg-black text-white">
      <div className="text-center">
        <h1 className="text-3xl">Something went wrong</h1>
        <button className="mt-5 rounded-xl bg-zinc-700 px-5 py-3" onClick={reset}>
          Try again
        </button>
      </div>
    </div>
  );
}
