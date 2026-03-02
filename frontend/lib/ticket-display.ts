export type PriorityBadgeVariant = "n1" | "n2" | "n3" | "n4";
export type StatusBadgeVariant =
  | "pending"
  | "assigned"
  | "in_progress"
  | "completed"
  | "canceled"
  | "paused";

export function getPriorityBadgeVariant(priority: number): PriorityBadgeVariant {
  if (priority === 1) return "n1";
  if (priority === 2) return "n2";
  if (priority === 3) return "n3";
  return "n4";
}

export function getStatusBadgeVariant(status: string): StatusBadgeVariant {
  if (status === "assigned") return "assigned";
  if (status === "in_progress" || status === "arrived") return "in_progress";
  if (status === "completed") return "completed";
  if (status === "canceled") return "canceled";
  if (status === "suspended" || status === "paused") return "paused";
  return "pending";
}

export function getStatusLabel(status: string): string {
  switch (status) {
    case "pending":
      return "En attente";
    case "assigned":
      return "Assigne";
    case "in_progress":
      return "En cours";
    case "arrived":
      return "Arrive";
    case "suspended":
      return "Suspendu";
    case "completed":
      return "Termine";
    case "canceled":
      return "Annule";
    default:
      return status;
  }
}
