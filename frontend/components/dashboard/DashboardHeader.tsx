import Link from "next/link";

import { NotificationBell } from "@/components/layout/NotificationBell";

export function DashboardHeader() {
  return (
    <header className="hply-fade-in relative overflow-hidden rounded-xl border border-blue-300/40 bg-gradient-to-r from-blue-700 via-blue-600 to-sky-500 px-5 py-5 text-white shadow-card">
      <div className="absolute -right-10 -top-10 h-36 w-36 rounded-full bg-white/10 blur-2xl" />
      <div className="absolute -bottom-14 left-28 h-36 w-36 rounded-full bg-cyan-200/20 blur-2xl" />
      <div className="absolute inset-y-0 right-0 w-1/3 bg-gradient-to-l from-white/10 to-transparent" />

      <div className="relative flex flex-wrap items-center justify-between gap-3">
        <div>
          <h1 className="font-title text-2xl font-semibold tracking-tight">Tableau de bord</h1>
          <p className="mt-1 text-sm leading-relaxed text-blue-100">
            Supervision en temps reel des demandes de transport internes
          </p>
        </div>

        <div className="flex items-center gap-3">
          <NotificationBell />
          <Link
            className="inline-flex min-h-[44px] items-center rounded-full bg-white px-4 py-2 text-sm font-semibold tracking-[0.01em] text-blue-700 shadow transition hover:-translate-y-px hover:shadow-md"
            href="/transport/new"
          >
            + Nouveau
          </Link>
        </div>
      </div>
    </header>
  );
}
