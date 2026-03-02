"use client";

import { useEffect, useMemo, useState } from "react";
import { useRouter } from "next/navigation";

import { QueueCard } from "@/components/porter/QueueCard";
import { useToast } from "@/components/providers/ToastProvider";
import { EmptyState } from "@/components/ui/EmptyState";
import { useTickets } from "@/hooks/useTickets";
import { api } from "@/lib/api";
import { useAuthStore } from "@/stores/authStore";

const tabs = [
  { key: "pending", label: "En attente" },
  { key: "assigned", label: "Assignees" },
  { key: "in_progress", label: "En cours" },
  { key: "suspended", label: "Suspendues" },
  { key: "completed", label: "Terminees" },
] as const;

const ACTIVE_MISSION_STATUSES = ["assigned", "in_progress", "arrived", "suspended"];

export default function PorterQueuePage() {
  const router = useRouter();
  const { pushToast } = useToast();
  const { tickets, fetchTickets } = useTickets();
  const user = useAuthStore((state) => state.user);
  const [activeTab, setActiveTab] = useState<(typeof tabs)[number]["key"]>("pending");
  const [typeFilter, setTypeFilter] = useState<"ALL" | "PATIENT" | "EQUIPMENT" | "SPECIMEN">("ALL");
  const [search, setSearch] = useState("");
  const [submittingTicketId, setSubmittingTicketId] = useState<string | null>(null);

  useEffect(() => {
    fetchTickets().catch(() => undefined);
  }, [fetchTickets]);

  const counts = useMemo(
    () =>
      tabs.reduce<Record<string, number>>((acc, tab) => {
        acc[tab.key] = tickets.filter((ticket) => ticket.status === tab.key).length;
        return acc;
      }, {}),
    [tickets],
  );

  const queue = useMemo(() => {
    return tickets.filter((ticket) => {
      if (ticket.status !== activeTab) return false;
      if (typeFilter !== "ALL" && ticket.transportType !== typeFilter) return false;

      if (!search.trim()) return true;

      return [ticket.patientName, ticket.origin, ticket.destination, ticket.id]
        .join(" ")
        .toLowerCase()
        .includes(search.trim().toLowerCase());
    });
  }, [activeTab, tickets, typeFilter, search]);

  const byPriority = useMemo(() => {
    return {
      p1: queue.filter((ticket) => ticket.priority === 1).length,
      p2: queue.filter((ticket) => ticket.priority === 2).length,
      p3: queue.filter((ticket) => ticket.priority === 3).length,
      p4: queue.filter((ticket) => ticket.priority === 4).length,
    };
  }, [queue]);

  const assignToMe = async (ticketId: string) => {
    setSubmittingTicketId(ticketId);
    try {
      await api.post(`/tickets/${ticketId}/assign`, {});
      await fetchTickets();
      pushToast({ title: "Mission assignee", type: "success" });
    } catch (error) {
      pushToast({
        title: "Erreur",
        message: error instanceof Error ? error.message : "Impossible d'accepter la mission.",
        type: "error",
      });
    } finally {
      setSubmittingTicketId(null);
    }
  };

  return (
    <section className="space-y-3.5">
      <h1 className="font-title text-2xl font-semibold tracking-tight">File partagee</h1>

      <div className="hply-lift rounded-xl border border-porter-border bg-porter-surface p-3">
        <input
          className="min-h-[60px] w-full rounded-lg border border-porter-border bg-porter-surface-elev px-3.5 text-sm text-[#EAF4F9] placeholder:text-[#B9D9E6]"
          placeholder="Rechercher patient, destination, ticket..."
          value={search}
          onChange={(event) => setSearch(event.target.value)}
        />
      </div>

      <div className="hply-soft-scroll flex gap-2 overflow-x-auto pb-1">
        {tabs.map((tab) => (
          <button
            key={tab.key}
            className={`min-h-[60px] rounded-lg border px-3 text-xs font-semibold tracking-[0.01em] transition ${
              activeTab === tab.key
                ? "border-primary bg-primary/20 text-white"
                : "border-porter-border bg-porter-surface text-[#B9D9E6] hover:-translate-y-px hover:bg-porter-surface-elev"
            }`}
            onClick={() => setActiveTab(tab.key)}
            type="button"
          >
            {tab.label} ({counts[tab.key] ?? 0})
          </button>
        ))}
      </div>

      <div className="flex flex-wrap gap-2">
        {[
          { value: "ALL", label: "Tous" },
          { value: "PATIENT", label: "Patient" },
          { value: "EQUIPMENT", label: "Materiel" },
          { value: "SPECIMEN", label: "Prelevement" },
        ].map((item) => (
          <button
            key={item.value}
            className={`rounded-full px-3 py-1.5 text-xs font-semibold transition ${
              typeFilter === item.value
                ? "bg-primary text-[#011C40]"
                : "bg-porter-surface-elev text-[#B9D9E6] hover:-translate-y-px hover:bg-porter-surface"
            }`}
            onClick={() => setTypeFilter(item.value as typeof typeFilter)}
            type="button"
          >
            {item.label}
          </button>
        ))}
      </div>

      {queue.map((ticket) => {
        const isMine = ticket.porterId === user?.porter_id;
        const canOpenActive = isMine && ACTIVE_MISSION_STATUSES.includes(ticket.status);

        return (
          <QueueCard
            key={ticket.id}
            actionDisabled={submittingTicketId === ticket.id}
            actionLabel={canOpenActive ? "Ouvrir mission" : "Accepter la mission"}
            ticket={ticket}
            onAction={(ticketId) => {
              if (canOpenActive) {
                router.push("/porter/active");
                return;
              }
              assignToMe(ticketId).catch(() => undefined);
            }}
          />
        );
      })}

      {queue.length === 0 ? (
        <EmptyState
          title="Aucun ticket disponible"
          description="Ajustez les filtres ou revenez dans quelques instants."
          className="border-porter-border bg-porter-surface text-[#EAF4F9]"
        />
      ) : null}

      <footer className="flex flex-wrap gap-2 rounded-lg border border-porter-border bg-porter-surface px-3 py-2 text-xs">
        <span className="rounded-full bg-red-500/20 px-2 py-1">N1 {byPriority.p1} Urgence</span>
        <span className="rounded-full bg-orange-500/20 px-2 py-1">N2 {byPriority.p2} Prioritaire</span>
        <span className="rounded-full bg-sky-500/20 px-2 py-1">N3 {byPriority.p3} Standard</span>
        <span className="rounded-full bg-emerald-500/20 px-2 py-1">N4 {byPriority.p4} Programme</span>
      </footer>
    </section>
  );
}
