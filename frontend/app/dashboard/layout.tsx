"use client";

import { useState } from "react";

import { Sidebar } from "@/components/dashboard/Sidebar";

export default function DashboardLayout({ children }: { children: React.ReactNode }) {
  const [open, setOpen] = useState(false);

  return (
    <div className="relative flex min-h-screen bg-dash-bg">
      <div className="hidden tablet:block">
        <Sidebar />
      </div>

      {open ? (
        <div className="fixed inset-0 z-[300] tablet:hidden">
          <button
            aria-label="Fermer la navigation"
            className="absolute inset-0 bg-slate-950/55 backdrop-blur-sm"
            onClick={() => setOpen(false)}
            type="button"
          />
          <Sidebar className="relative z-[301] h-full" onNavigate={() => setOpen(false)} />
        </div>
      ) : null}

      <div className="flex-1 p-3 sm:p-4 md:p-6">
        <button
          aria-label="Ouvrir la navigation"
          className="sticky top-3 z-[150] mb-3 inline-flex min-h-[44px] items-center rounded-md border border-slate-300 bg-white/95 px-3 py-2 text-sm font-medium text-slate-700 shadow-sm backdrop-blur transition hover:bg-white tablet:hidden"
          onClick={() => setOpen(true)}
          type="button"
        >
          ☰ Menu
        </button>
        {children}
      </div>
    </div>
  );
}
