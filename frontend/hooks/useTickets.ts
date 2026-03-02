"use client";

import { useCallback } from "react";

import { api } from "@/lib/api";
import type { Ticket } from "@/lib/types";
import { useFilterStore } from "@/stores/filterStore";
import { useTicketStore } from "@/stores/ticketStore";

export function useTickets() {
  const tickets = useTicketStore((state) => state.tickets);
  const loading = useTicketStore((state) => state.loading);
  const setTickets = useTicketStore((state) => state.setTickets);
  const setLoading = useTicketStore((state) => state.setLoading);
  const upsertTicket = useTicketStore((state) => state.upsertTicket);

  const statusFilter = useFilterStore((state) => state.status);
  const priorityFilter = useFilterStore((state) => state.priority);
  const transportTypeFilter = useFilterStore((state) => state.transportType);

  const fetchTickets = useCallback(async () => {
    setLoading(true);

    try {
      const search = new URLSearchParams();
      if (statusFilter) search.set("status", statusFilter);
      if (priorityFilter) search.set("priority", priorityFilter);
      if (transportTypeFilter) search.set("transport_type", transportTypeFilter);

      const data = await api.get<Ticket[]>(`/tickets?${search.toString()}`);
      setTickets(data);
    } finally {
      setLoading(false);
    }
  }, [priorityFilter, setLoading, setTickets, statusFilter, transportTypeFilter]);

  const updateStatus = useCallback(
    async (ticketId: string, status: string, reason_code?: string, comment?: string) => {
      const updated = await api.patch<Ticket>(`/tickets/${ticketId}/status`, {
        status,
        reason_code,
        comment,
      });
      upsertTicket(updated);
      return updated;
    },
    [upsertTicket],
  );

  return {
    tickets,
    loading,
    fetchTickets,
    updateStatus,
  };
}
