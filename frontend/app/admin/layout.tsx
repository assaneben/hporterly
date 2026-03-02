"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";

const links = [
  { href: "/admin/porters", label: "Brancardiers" },
  { href: "/admin/settings", label: "Referentiels" },
  { href: "/admin/priority-rules", label: "Regles priorite" },
  { href: "/admin/messages", label: "Messages" },
];

export default function AdminLayout({ children }: { children: React.ReactNode }) {
  const pathname = usePathname();

  return (
    <div className="min-h-screen bg-dash-bg p-4 md:p-6">
      <header className="hply-fade-in mb-4 overflow-hidden rounded-xl border border-slate-200 bg-gradient-to-r from-slate-900 to-slate-800 px-5 py-[18px] text-white shadow-card">
        <h1 className="font-title text-2xl font-semibold">Administration</h1>
        <p className="mt-1 text-sm leading-relaxed text-slate-300">Gestion des ressources, referentiels et regles metier</p>

        <nav className="mt-4 flex flex-wrap gap-2">
          {links.map((link) => {
            const active = pathname === link.href;
            return (
              <Link
                key={link.href}
                className={`rounded-full border px-3.5 py-1.5 text-sm font-semibold transition hover:-translate-y-px ${
                  active
                    ? "border-white/20 bg-white text-slate-900"
                    : "border-white/20 bg-white/10 text-slate-100 hover:bg-white/20"
                }`}
                href={link.href}
              >
                {link.label}
              </Link>
            );
          })}
        </nav>
      </header>
      <main id="main-content">{children}</main>
    </div>
  );
}
