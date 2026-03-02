import { Badge } from "@/components/ui/Badge";
import { getPriorityBadgeVariant, getStatusLabel } from "@/lib/ticket-display";
import type { Ticket } from "@/lib/types";

export function MissionCard({ ticket }: { ticket: Ticket }) {
  return (
    <article className="rounded-xl border border-porter-border bg-porter-surface-elev p-4 text-[#EAF4F9]">
      <div className="mb-2 flex items-center justify-between gap-2">
        <p className="font-title text-base font-semibold">{ticket.patientName}</p>
        <Badge variant={getPriorityBadgeVariant(ticket.priority)}>{`N${ticket.priority}`}</Badge>
      </div>
      <p className="text-sm text-[#B9D9E6]">
        {ticket.origin} → {ticket.destination}
      </p>
      <p className="mt-3 text-xs uppercase tracking-wide text-[#B9D9E6]">Statut: {getStatusLabel(ticket.status)}</p>
    </article>
  );
}
