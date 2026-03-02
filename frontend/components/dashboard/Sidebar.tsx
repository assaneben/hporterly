"use client";

import { useMemo } from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";

import { Badge } from "@/components/ui/Badge";
import { cx } from "@/lib/utils";
import { useFilterStore } from "@/stores/filterStore";
import { useTicketStore } from "@/stores/ticketStore";

type Props = {
  counts?: Record<string, number>;
  className?: string;
  onNavigate?: () => void;
};

type FilterPreset = {
  status: string;
  priority: string;
  transportType: string;
};

type NavItem = {
  href: string;
  label: string;
  key: string;
  filters?: FilterPreset;
};

const clearFilters: FilterPreset = {
  status: "",
  priority: "",
  transportType: "",
};

const blocks: Array<{ title: string; items: NavItem[] }> = [
  {
    title: "Navigation priorites",
    items: [
      { href: "/dashboard", label: "Toutes les demandes", key: "all", filters: clearFilters },
      { href: "/dashboard", label: "N1 Urgence", key: "n1", filters: { status: "", priority: "1", transportType: "" } },
      { href: "/dashboard", label: "N2 Prioritaire", key: "n2", filters: { status: "", priority: "2", transportType: "" } },
      { href: "/dashboard", label: "N3 Standard", key: "n3", filters: { status: "", priority: "3", transportType: "" } },
      { href: "/dashboard", label: "N4 Programme", key: "n4", filters: { status: "", priority: "4", transportType: "" } },
    ],
  },
  {
    title: "Statuts",
    items: [
      { href: "/dashboard", label: "En attente", key: "pending", filters: { status: "pending", priority: "", transportType: "" } },
      { href: "/dashboard", label: "Assignees", key: "assigned", filters: { status: "assigned", priority: "", transportType: "" } },
      { href: "/dashboard", label: "En cours", key: "in_progress", filters: { status: "in_progress", priority: "", transportType: "" } },
      { href: "/dashboard", label: "Suspendues", key: "suspended", filters: { status: "suspended", priority: "", transportType: "" } },
      { href: "/dashboard", label: "Terminees", key: "completed", filters: { status: "completed", priority: "", transportType: "" } },
    ],
  },
  {
    title: "Types",
    items: [
      { href: "/dashboard", label: "Patient", key: "tp", filters: { status: "", priority: "", transportType: "PATIENT" } },
      { href: "/dashboard", label: "Materiel", key: "te", filters: { status: "", priority: "", transportType: "EQUIPMENT" } },
      { href: "/dashboard", label: "Prelevement", key: "ts", filters: { status: "", priority: "", transportType: "SPECIMEN" } },
    ],
  },
  {
    title: "Analyse",
    items: [{ href: "/dashboard/reports", label: "Rapports", key: "reports" }],
  },
];

export function Sidebar({ counts = {}, className, onNavigate }: Props) {
  const pathname = usePathname();
  const tickets = useTicketStore((state) => state.tickets);
  const status = useFilterStore((state) => state.status);
  const priority = useFilterStore((state) => state.priority);
  const transportType = useFilterStore((state) => state.transportType);
  const setFilter = useFilterStore((state) => state.setFilter);

  const computedCounts = useMemo<Record<string, number>>(
    () => ({
      all: tickets.length,
      n1: tickets.filter((ticket) => ticket.priority === 1).length,
      n2: tickets.filter((ticket) => ticket.priority === 2).length,
      n3: tickets.filter((ticket) => ticket.priority === 3).length,
      n4: tickets.filter((ticket) => ticket.priority === 4).length,
      pending: tickets.filter((ticket) => ticket.status === "pending").length,
      assigned: tickets.filter((ticket) => ticket.status === "assigned").length,
      in_progress: tickets.filter((ticket) => ticket.status === "in_progress").length,
      suspended: tickets.filter((ticket) => ticket.status === "suspended").length,
      completed: tickets.filter((ticket) => ticket.status === "completed").length,
      types_all: tickets.length,
      tp: tickets.filter((ticket) => ticket.transportType === "PATIENT").length,
      te: tickets.filter((ticket) => ticket.transportType === "EQUIPMENT").length,
      ts: tickets.filter((ticket) => ticket.transportType === "SPECIMEN").length,
      reports: 0,
    }),
    [tickets],
  );

  const mergedCounts = { ...computedCounts, ...counts };

  return (
    <aside
      className={cx(
        "h-full w-[260px] overflow-y-auto border-r border-slate-700/70 bg-gradient-to-b from-slate-800 to-slate-950 px-3 pb-6 pt-5 text-slate-300 hply-soft-scroll",
        className,
      )}
    >
      <div className="mb-6 rounded-lg border border-white/10 bg-white/5 p-3">
        <p className="font-title text-2xl font-bold text-white">HPly</p>
        <p className="mt-1 text-xs text-slate-400">Coordination des transports</p>
      </div>

      <nav className="space-y-5">
        {blocks.map((block) => (
          <div key={block.title}>
            <p className="mb-2 px-2 text-[11px] uppercase tracking-[0.14em] text-slate-500">{block.title}</p>
            <div className="space-y-1">
              {block.items.map((item) => {
                const active =
                  item.href === "/dashboard/reports"
                    ? pathname === item.href
                    : pathname === "/dashboard" &&
                      (item.filters
                        ? status === item.filters.status &&
                          priority === item.filters.priority &&
                          transportType === item.filters.transportType
                        : false);

                return (
                  <Link
                    key={`${item.key}-${item.label}`}
                    className={cx(
                      "group flex items-center justify-between rounded-md border-l-[3px] border-transparent px-3 py-2.5 text-sm transition",
                      active
                        ? "border-blue-400 bg-gradient-to-r from-blue-600/35 to-blue-400/15 text-white"
                        : "hover:bg-white/10 hover:text-slate-100",
                    )}
                    href={item.href}
                    onClick={() => {
                      if (item.filters) {
                        setFilter("status", item.filters.status);
                        setFilter("priority", item.filters.priority);
                        setFilter("transportType", item.filters.transportType);
                      }
                      onNavigate?.();
                    }}
                  >
                    <span>{item.label}</span>
                    {item.key === "n1" && (mergedCounts[item.key] ?? 0) > 0 ? (
                      <Badge variant="n1" pulse>
                        {String(mergedCounts[item.key])}
                      </Badge>
                    ) : (
                      <span className="text-[11px] text-slate-500 group-hover:text-slate-300">
                        {mergedCounts[item.key] ?? 0}
                      </span>
                    )}
                  </Link>
                );
              })}
            </div>
          </div>
        ))}
      </nav>
    </aside>
  );
}
