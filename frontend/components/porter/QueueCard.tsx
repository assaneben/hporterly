import { Badge } from "@/components/ui/Badge";
import { getPriorityBadgeVariant, getStatusLabel } from "@/lib/ticket-display";
import type { Ticket } from "@/lib/types";

type Props = {
  ticket: Ticket;
  actionLabel: string;
  actionDisabled?: boolean;
  onAction: (ticketId: string) => void;
};

export function QueueCard({ ticket, actionLabel, actionDisabled, onAction }: Props) {
  return (
    <article className="hply-lift rounded-xl border border-porter-border bg-porter-surface p-4 text-[#EAF4F9] shadow-card transition hover:bg-porter-surface-elev">
      <div className="mb-2 flex items-center justify-between gap-2">
        <Badge variant={getPriorityBadgeVariant(ticket.priority)} pulse={ticket.priority === 1}>
          {`N${ticket.priority}`}
        </Badge>
        <span className="text-[11px] uppercase tracking-[0.12em] text-[#B9D9E6]">{getStatusLabel(ticket.status)}</span>
      </div>

      <p className="font-title text-lg font-semibold leading-tight">{ticket.patientName}</p>
      <p className="mt-1 text-sm leading-relaxed text-[#B9D9E6]">
        {ticket.origin} -&gt; {ticket.destination}
      </p>

      <div className="mt-3 flex flex-wrap gap-1.5 text-[11px] font-medium">
        {ticket.needsO2 ? <span className="rounded-full bg-sky-500/20 px-2 py-1 text-[11px]">O2</span> : null}
        {ticket.isolation ? <span className="rounded-full bg-violet-500/20 px-2 py-1 text-[11px]">Isolement</span> : null}
        {ticket.patientMonitoring ? <span className="rounded-full bg-red-500/20 px-2 py-1 text-[11px]">Monit</span> : null}
      </div>

      <button
        data-testid="queue-card-action"
        className="mt-4 min-h-[60px] w-full rounded-full bg-gradient-to-r from-primary to-primary-dark px-4 py-2 text-sm font-semibold tracking-[0.01em] text-white shadow disabled:cursor-not-allowed disabled:opacity-60"
        disabled={actionDisabled}
        onClick={() => onAction(ticket.id)}
        type="button"
      >
        {actionLabel}
      </button>
    </article>
  );
}
