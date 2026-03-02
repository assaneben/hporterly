import type { TicketStatus } from "../types";

const NORMALIZATION_MAP: Record<string, TicketStatus> = {
  pending: "pending",
  assigned: "assigned",
  in_progress: "in_progress",
  arrived: "arrived",
  suspended: "suspended",
  paused: "suspended",
  completed: "completed",
  canceled: "canceled",
  picked_up: "in_progress",
  desaffectee: "pending",
  reassignee: "assigned",
};

const STATUS_TRANSITIONS: Record<TicketStatus, TicketStatus[]> = {
  pending: ["assigned", "canceled"],
  assigned: ["in_progress", "suspended", "pending", "canceled"],
  in_progress: ["arrived", "suspended", "canceled"],
  arrived: ["completed", "suspended", "canceled"],
  suspended: ["assigned", "in_progress", "pending", "canceled"],
  completed: [],
  canceled: [],
};

export function normalizeStatus(status: string): TicketStatus | null {
  return NORMALIZATION_MAP[status] ?? null;
}

export function canTransition(from: string, to: string): boolean {
  const normalizedFrom = normalizeStatus(from);
  const normalizedTo = normalizeStatus(to);

  if (!normalizedFrom || !normalizedTo) {
    return false;
  }

  return STATUS_TRANSITIONS[normalizedFrom].includes(normalizedTo);
}