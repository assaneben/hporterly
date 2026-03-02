"use client";

import { useEffect, useMemo } from "react";

import { MissionCard } from "@/components/porter/MissionCard";
import { EmptyState } from "@/components/ui/EmptyState";
import { useTickets } from "@/hooks/useTickets";
import { useAuthStore } from "@/stores/authStore";

export default function PorterMissionsPage() {
  const { tickets, fetchTickets } = useTickets();
  const user = useAuthStore((state) => state.user);

  useEffect(() => {
    fetchTickets().catch(() => undefined);
  }, [fetchTickets]);

  const mine = useMemo(
    () => tickets.filter((ticket) => ticket.porterId && ticket.porterId === user?.porter_id),
    [tickets, user?.porter_id],
  );

  const groups = {
    prepare: mine.filter((ticket) => ["assigned", "picked_up"].includes(ticket.status)),
    transport: mine.filter((ticket) => ticket.status === "in_progress"),
    suspended: mine.filter((ticket) => ticket.status === "suspended"),
    finalize: mine.filter((ticket) => ticket.status === "arrived"),
  };

  const renderGroup = (title: string, ticketsInGroup: typeof mine) => (
    <div className="space-y-2 rounded-xl border border-porter-border bg-porter-surface p-3">
      <h2 className="text-xs uppercase tracking-[0.14em] text-[#B9D9E6]">{title}</h2>
      {ticketsInGroup.length === 0 ? (
        <p className="text-sm text-[#B9D9E6]">Aucune mission dans cette section.</p>
      ) : (
        ticketsInGroup.map((ticket) => <MissionCard key={ticket.id} ticket={ticket} />)
      )}
    </div>
  );

  return (
    <section className="space-y-4">
      <h1 className="font-title text-2xl font-semibold">Mes missions</h1>

      {mine.length === 0 ? (
        <EmptyState
          title="Aucune mission assignee"
          description="Les missions que vous acceptez apparaitront ici."
          className="border-porter-border bg-porter-surface text-[#EAF4F9]"
        />
      ) : null}

      {renderGroup("A preparer", groups.prepare)}
      {renderGroup("En transport", groups.transport)}
      {renderGroup("Suspendues", groups.suspended)}
      {renderGroup("A finaliser", groups.finalize)}
    </section>
  );
}
