import { Badge } from "@/components/ui/Badge";
import { Card } from "@/components/ui/Card";
import { getPriorityBadgeVariant, getStatusBadgeVariant, getStatusLabel } from "@/lib/ticket-display";
import type { Ticket } from "@/lib/types";
import { formatDate } from "@/lib/utils";

type Props = {
  ticket: Ticket;
};

export function TicketDetails({ ticket }: Props) {
  return (
    <Card className="space-y-3" variant="dashboard">
      <div className="flex flex-wrap items-center gap-2">
        <h3 className="font-title text-lg font-semibold text-slate-900">Details ticket</h3>
        <Badge variant={getPriorityBadgeVariant(ticket.priority)}>{`N${ticket.priority}`}</Badge>
        <Badge variant={getStatusBadgeVariant(ticket.status)}>{getStatusLabel(ticket.status)}</Badge>
      </div>
      <p className="text-sm text-slate-700">Patient: {ticket.patientName}</p>
      <p className="text-sm text-slate-700">
        Trajet: {ticket.origin} → {ticket.destination}
      </p>
      <div className="flex flex-wrap gap-2">
        {ticket.needsO2 ? <Badge variant="n2">O2</Badge> : null}
        {ticket.needsPerfusion ? <Badge variant="n2">Perfusion</Badge> : null}
        {ticket.isolation ? <Badge variant="n3">Isolement</Badge> : null}
        {ticket.patientMonitoring ? <Badge variant="n1">Monitoring</Badge> : null}
        {ticket.needsTwoPorters ? <Badge variant="n2">2 brancardiers</Badge> : null}
      </div>
      <p className="text-xs text-slate-500">Cree le {formatDate(ticket.createdAt)}</p>
      <p className="text-xs text-slate-500">Mis a jour le {formatDate(ticket.updatedAt)}</p>
    </Card>
  );
}
