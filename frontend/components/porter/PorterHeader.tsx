"use client";

import { StatusToggle } from "./StatusToggle";

export function PorterHeader({ name }: { name: string }) {
  return (
    <header className="sticky top-0 z-20 border-b border-porter-border bg-porter-surface/95 px-4 py-3 backdrop-blur">
      <div className="mx-auto flex w-full max-w-3xl items-center justify-between gap-3">
        <div>
          <p className="font-title text-lg font-semibold leading-tight text-[#EAF4F9]">{name}</p>
          <p className="mt-0.5 text-xs tracking-[0.02em] text-[#B9D9E6]">Application brancardier</p>
        </div>
        <StatusToggle />
      </div>
    </header>
  );
}
