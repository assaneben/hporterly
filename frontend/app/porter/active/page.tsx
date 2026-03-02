"use client";

import { useEffect, useMemo, useState } from "react";

import { ActiveMission } from "@/components/porter/ActiveMission";
import { ReinforcementBanner } from "@/components/porter/ReinforcementBanner";
import { useToast } from "@/components/providers/ToastProvider";
import { EmptyState } from "@/components/ui/EmptyState";
import { useTickets } from "@/hooks/useTickets";
import { api } from "@/lib/api";
import { useAuthStore } from "@/stores/authStore";

const ACTIVE_STATUSES = ["assigned", "in_progress", "arrived", "suspended"] as const;

const PHASE_BY_STATUS: Record<string, number> = {
  assigned: 0,
  in_progress: 2,
  arrived: 3,
  suspended: 2,
  completed: 4,
};

const NEXT_STATUS: Record<string, string | null> = {
  assigned: "in_progress",
  in_progress: "arrived",
  arrived: "completed",
  suspended: "in_progress",
  completed: null,
};

export default function PorterActiveMissionPage() {
  const { pushToast } = useToast();
  const { tickets, fetchTickets, updateStatus } = useTickets();
  const user = useAuthStore((state) => state.user);
  const [loadingAction, setLoadingAction] = useState(false);

  useEffect(() => {
    fetchTickets().catch(() => undefined);
  }, [fetchTickets]);

  const activeTicket = useMemo(
    () =>
      tickets.find(
        (ticket) => ticket.porterId === user?.porter_id && ACTIVE_STATUSES.includes(ticket.status as (typeof ACTIVE_STATUSES)[number]),
      ),
    [tickets, user?.porter_id],
  );

  const nextStatus = activeTicket ? NEXT_STATUS[activeTicket.status] ?? null : null;
  const currentPhase = activeTicket ? PHASE_BY_STATUS[activeTicket.status] ?? 0 : 0;

  const performStatusUpdate = async (status: string, reasonCode?: string) => {
    if (!activeTicket) {
      return;
    }

    setLoadingAction(true);
    try {
      await updateStatus(activeTicket.id, status, reasonCode);
      await fetchTickets();
      pushToast({ title: "Mission mise a jour", type: "success" });
    } catch (error) {
      pushToast({
        title: "Erreur",
        message: error instanceof Error ? error.message : "Impossible de mettre a jour la mission.",
        type: "error",
      });
    } finally {
      setLoadingAction(false);
    }
  };

  if (!activeTicket) {
    return (
      <EmptyState
        title="Aucune mission active"
        description="Acceptez une mission dans la queue partagee pour commencer."
        className="border-porter-border bg-porter-surface text-[#EAF4F9]"
      />
    );
  }

  return (
    <section className="space-y-3">
      <ActiveMission
        advanceDisabled={loadingAction || !nextStatus}
        currentPhase={currentPhase}
        suspendDisabled={loadingAction || activeTicket.status === "suspended"}
        ticket={activeTicket}
        onAdvance={() => {
          if (!nextStatus) return;
          performStatusUpdate(nextStatus).catch(() => undefined);
        }}
        onSuspend={() => {
          performStatusUpdate("suspended", "urgent_interruption").catch(() => undefined);
        }}
      />
      <ReinforcementBanner
        onRequest={() => {
          if (!user?.porter_id) return;
          api
            .post(`/tickets/${activeTicket.id}/help`, {
              requested_porter_id: user.porter_id,
            })
            .then(() => {
              pushToast({ title: "Renfort demande", type: "success" });
            })
            .catch(() => {
              pushToast({ title: "Erreur", message: "Demande de renfort impossible.", type: "error" });
            });
        }}
      />
    </section>
  );
}
