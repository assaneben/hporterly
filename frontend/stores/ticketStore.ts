import { create } from "zustand";

import type { Ticket } from "@/lib/types";

type TicketState = {
  tickets: Ticket[];
  loading: boolean;
  setTickets: (tickets: Ticket[]) => void;
  upsertTicket: (ticket: Ticket) => void;
  removeTicket: (ticketId: string) => void;
  setLoading: (value: boolean) => void;
};

export const useTicketStore = create<TicketState>((set, get) => ({
  tickets: [],
  loading: false,
  setTickets: (tickets) => set({ tickets }),
  upsertTicket: (ticket) => {
    const current = get().tickets;
    const index = current.findIndex((item) => item.id === ticket.id);

    if (index === -1) {
      set({ tickets: [ticket, ...current] });
      return;
    }

    const next = [...current];
    next[index] = ticket;
    set({ tickets: next });
  },
  removeTicket: (ticketId) => set({ tickets: get().tickets.filter((ticket) => ticket.id !== ticketId) }),
  setLoading: (value) => set({ loading: value }),
}));