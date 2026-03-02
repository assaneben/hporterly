"use client";

import { useEffect, useMemo, useState } from "react";

import { AssignModal } from "@/components/dashboard/AssignModal";
import { DashboardHeader } from "@/components/dashboard/DashboardHeader";
import { FilterBar } from "@/components/dashboard/FilterBar";
import { KpiCards } from "@/components/dashboard/KpiCards";
import { NotesModal } from "@/components/dashboard/NotesModal";
import { PriorityOverrideModal } from "@/components/dashboard/PriorityOverrideModal";
import { ReassignModal } from "@/components/dashboard/ReassignModal";
import { ReportsPanel } from "@/components/dashboard/ReportsPanel";
import { TicketDetails } from "@/components/dashboard/TicketDetails";
import { useToast } from "@/components/providers/ToastProvider";
import { Badge } from "@/components/ui/Badge";
import { Button } from "@/components/ui/Button";
import { DataTable } from "@/components/ui/DataTable";
import { EmptyState } from "@/components/ui/EmptyState";
import { InlineFeedback } from "@/components/ui/InlineFeedback";
import { SkeletonLoader } from "@/components/ui/SkeletonLoader";
import { useTickets } from "@/hooks/useTickets";
import { api } from "@/lib/api";
import { getPriorityBadgeVariant, getStatusBadgeVariant, getStatusLabel } from "@/lib/ticket-display";
import type { Ticket } from "@/lib/types";
import { formatDate } from "@/lib/utils";
import { useFilterStore } from "@/stores/filterStore";

export default function DashboardPage() {
  const { pushToast } = useToast();
  const { tickets, loading, fetchTickets, updateStatus } = useTickets();

  const search = useFilterStore((state) => state.search).trim().toLowerCase();
  const statusFilter = useFilterStore((state) => state.status);
  const priorityFilter = useFilterStore((state) => state.priority);
  const transportTypeFilter = useFilterStore((state) => state.transportType);

  const [selectedTicket, setSelectedTicket] = useState<Ticket | null>(null);
  const [assignOpen, setAssignOpen] = useState(false);
  const [reassignOpen, setReassignOpen] = useState(false);
  const [priorityOpen, setPriorityOpen] = useState(false);
  const [notesOpen, setNotesOpen] = useState(false);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [actionKey, setActionKey] = useState<string | null>(null);

  useEffect(() => {
    setLoadError(null);
    fetchTickets().catch((error) => {
      const message = error instanceof Error ? error.message : "Impossible de charger les tickets.";
      setLoadError(message);
      pushToast({ title: "Erreur", message, type: "error" });
    });
  }, [fetchTickets, pushToast]);

  useEffect(() => {
    if (!selectedTicket) {
      return;
    }
    const refreshed = tickets.find((item) => item.id === selectedTicket.id) ?? null;
    setSelectedTicket(refreshed);
  }, [selectedTicket, tickets]);

  const filtered = useMemo(
    () =>
      tickets.filter((ticket) => {
        if (statusFilter && ticket.status !== statusFilter) {
          return false;
        }

        if (priorityFilter && String(ticket.priority) !== priorityFilter) {
          return false;
        }

        if (transportTypeFilter && ticket.transportType !== transportTypeFilter) {
          return false;
        }

        if (!search) {
          return true;
        }

        return [ticket.patientName, ticket.origin, ticket.destination, ticket.status, ticket.id]
          .join(" ")
          .toLowerCase()
          .includes(search);
      }),
    [tickets, statusFilter, priorityFilter, transportTypeFilter, search],
  );

  const stats = useMemo(() => {
    return {
      total: filtered.length,
      inProgress: filtered.filter((ticket) => ticket.status === "in_progress").length,
      completed: filtered.filter((ticket) => ticket.status === "completed").length,
      canceled: filtered.filter((ticket) => ticket.status === "canceled").length,
    };
  }, [filtered]);

  const runAction = async (key: string, action: () => Promise<void>, successTitle: string) => {
    setActionKey(key);
    try {
      await action();
      await fetchTickets();
      pushToast({ title: successTitle, type: "success" });
    } catch (error) {
      pushToast({
        title: "Erreur",
        message: error instanceof Error ? error.message : "Action impossible",
        type: "error",
      });
    } finally {
      setActionKey(null);
    }
  };

  const columns = useMemo(
    () => [
      {
        id: "priority",
        header: "Priorite",
        width: "120px",
        sortable: true,
        sortValue: (ticket: Ticket) => ticket.priority,
        cell: (ticket: Ticket) => (
          <Badge variant={getPriorityBadgeVariant(ticket.priority)} pulse={ticket.priority === 1}>
            {`N${ticket.priority}`}
          </Badge>
        ),
      },
      {
        id: "request",
        header: "N.Demande",
        width: "180px",
        sortable: true,
        sortValue: (ticket: Ticket) => ticket.id,
        cell: (ticket: Ticket) => <span className="font-mono text-xs tracking-wide">{ticket.id.slice(0, 8)}</span>,
      },
      {
        id: "patient",
        header: "Patient",
        width: "200px",
        sortable: true,
        sortValue: (ticket: Ticket) => ticket.patientName,
        cell: (ticket: Ticket) => ticket.patientName,
      },
      {
        id: "origin",
        header: "Depart",
        width: "150px",
        cell: (ticket: Ticket) => ticket.origin,
      },
      {
        id: "destination",
        header: "Destination",
        width: "150px",
        cell: (ticket: Ticket) => ticket.destination,
      },
      {
        id: "status",
        header: "Statut",
        width: "120px",
        cell: (ticket: Ticket) => <Badge variant={getStatusBadgeVariant(ticket.status)}>{getStatusLabel(ticket.status)}</Badge>,
      },
      {
        id: "porter",
        header: "Brancardier",
        width: "140px",
        cell: (ticket: Ticket) =>
          ticket.porterId ? <span className="font-mono text-xs tracking-wide">{ticket.porterId.slice(0, 8)}</span> : "-",
      },
      {
        id: "time",
        header: "Temps",
        width: "100px",
        sortable: true,
        sortValue: (ticket: Ticket) => ticket.createdAt,
        cell: (ticket: Ticket) => formatDate(ticket.createdAt),
      },
      {
        id: "actions",
        header: "Actions",
        width: "120px",
        cell: (ticket: Ticket) => (
          <div className="flex gap-1.5">
            <Button className="min-h-[34px] px-2.5 py-1 text-xs" variant="ghost" onClick={() => setSelectedTicket(ticket)}>
              Voir
            </Button>
            {ticket.status === "pending" ? (
              <Button
                data-testid="table-assign-button"
                className="min-h-[34px] px-2.5 py-1 text-xs"
                onClick={() => {
                  setSelectedTicket(ticket);
                  setAssignOpen(true);
                }}
              >
                Assigner
              </Button>
            ) : null}
          </div>
        ),
      },
    ],
    [],
  );

  if (loading && tickets.length === 0) {
    return (
      <main id="main-content" className="space-y-4 pb-4">
        <DashboardHeader />
        <div className="rounded-xl border border-slate-200 bg-white p-4">
          <SkeletonLoader lines={6} />
        </div>
      </main>
    );
  }

  return (
    <main id="main-content" className="space-y-4 pb-4">
      <DashboardHeader />

      {loadError ? <InlineFeedback message={loadError} tone="error" /> : null}

      <div className="hply-fade-in hply-fade-in-delay-1">
        <KpiCards
        total={stats.total}
        inProgress={stats.inProgress}
        completed={stats.completed}
        canceled={stats.canceled}
        />
      </div>

      <FilterBar />

      <div className="hply-fade-in hply-fade-in-delay-2">
        <DataTable title="Table des tickets" data={filtered} columns={columns} rowKey={(ticket) => ticket.id} />
      </div>

      {filtered.length === 0 ? (
        <EmptyState
          title="Aucun ticket pour ces criteres"
          description="Modifiez les filtres ou creez une nouvelle demande."
        />
      ) : null}

      {selectedTicket ? (
        <div className="grid gap-3 lg:grid-cols-2">
          <TicketDetails ticket={selectedTicket} />
          <div className="space-y-3 rounded-lg border border-slate-200 bg-white p-4">
            <h3 className="font-title text-lg font-semibold text-slate-900">Actions rapides</h3>
            <div className="flex flex-wrap gap-2">
              <Button
                data-testid="quick-reassign-button"
                disabled={Boolean(actionKey)}
                variant="ghost"
                onClick={() => setReassignOpen(true)}
              >
                Reassigner
              </Button>
              <Button disabled={Boolean(actionKey)} variant="ghost" onClick={() => setPriorityOpen(true)}>
                Override priorite
              </Button>
              <Button disabled={Boolean(actionKey)} variant="ghost" onClick={() => setNotesOpen(true)}>
                Modifier notes
              </Button>
              <Button
                data-testid="quick-start-button"
                disabled={Boolean(actionKey)}
                variant="ghost"
                onClick={() =>
                  runAction(
                    `start-${selectedTicket.id}`,
                    async () => {
                      await updateStatus(selectedTicket.id, "in_progress");
                    },
                    "Mission demarree",
                  )
                }
              >
                Demarrer
              </Button>
              <Button
                data-testid="quick-suspend-button"
                disabled={Boolean(actionKey)}
                variant="ghost"
                onClick={() =>
                  runAction(
                    `suspend-${selectedTicket.id}`,
                    async () => {
                      await updateStatus(selectedTicket.id, "suspended", "urgent_interruption");
                    },
                    "Mission suspendue",
                  )
                }
              >
                Suspendre
              </Button>
              <Button
                data-testid="quick-complete-button"
                disabled={Boolean(actionKey)}
                variant="ghost"
                onClick={() =>
                  runAction(
                    `complete-${selectedTicket.id}`,
                    async () => {
                      await updateStatus(selectedTicket.id, "completed");
                    },
                    "Mission completee",
                  )
                }
              >
                Completer
              </Button>
            </div>
          </div>
        </div>
      ) : null}

      <ReportsPanel tickets={filtered} />

      <AssignModal
        isOpen={assignOpen}
        ticketId={selectedTicket?.id ?? null}
        onClose={() => setAssignOpen(false)}
        onAssigned={(porterId) => {
          if (!selectedTicket) return;
          runAction(
            `assign-${selectedTicket.id}`,
            async () => {
              await api.post(`/tickets/${selectedTicket.id}/assign`, { porter_id: porterId });
              setAssignOpen(false);
            },
            "Ticket assigne",
          ).catch(() => undefined);
        }}
      />

      <ReassignModal
        isOpen={reassignOpen}
        ticketId={selectedTicket?.id ?? null}
        onClose={() => setReassignOpen(false)}
        onReassigned={(porterId) => {
          if (!selectedTicket) return;
          runAction(
            `reassign-${selectedTicket.id}`,
            async () => {
              await api.post(`/tickets/${selectedTicket.id}/reassign`, { porter_id: porterId });
              setReassignOpen(false);
            },
            "Ticket reassign",
          ).catch(() => undefined);
        }}
      />

      <PriorityOverrideModal
        isOpen={priorityOpen}
        ticketId={selectedTicket?.id ?? null}
        currentPriority={selectedTicket?.priority ?? 4}
        onClose={() => setPriorityOpen(false)}
        onSubmit={(priority, reason) => {
          if (!selectedTicket) return;
          runAction(
            `priority-${selectedTicket.id}`,
            async () => {
              await api.patch(`/tickets/${selectedTicket.id}/priority`, { priority, reason });
              setPriorityOpen(false);
            },
            "Priorite mise a jour",
          ).catch(() => undefined);
        }}
      />

      <NotesModal
        isOpen={notesOpen}
        initialNotes={selectedTicket?.notes}
        onClose={() => setNotesOpen(false)}
        onSave={(notes) => {
          if (!selectedTicket) return;
          runAction(
            `notes-${selectedTicket.id}`,
            async () => {
              await api.patch(`/tickets/${selectedTicket.id}/notes`, { notes });
              setNotesOpen(false);
            },
            "Notes enregistrees",
          ).catch(() => undefined);
        }}
      />
    </main>
  );
}
