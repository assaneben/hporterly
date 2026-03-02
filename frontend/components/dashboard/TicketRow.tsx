import { Badge } from "@/components/ui/Badge";
import { Button } from "@/components/ui/Button";
import type { Ticket } from "@/lib/types";
import { formatDate } from "@/lib/utils";

type Props = {
  ticket: Ticket;
  onOpen: (ticket: Ticket) => void;
};

function getPriorityVariant(priority: number): "n1" | "n2" | "n3" | "n4" {
  if (priority === 1) return "n1";
  if (priority === 2) return "n2";
  if (priority === 3) return "n3";
  return "n4";
}

export function TicketRow({ ticket, onOpen }: Props) {
  return (
    <tr className="hover:bg-slate-50">
      <td className="px-3 py-3">
        <Badge variant={getPriorityVariant(ticket.priority)}>{`N${ticket.priority}`}</Badge>
      </td>
      <td className="px-3 py-3 text-sm">{ticket.id.slice(0, 8)}</td>
      <td className="px-3 py-3 text-sm">{ticket.patientName}</td>
      <td className="px-3 py-3 text-sm">{ticket.origin}</td>
      <td className="px-3 py-3 text-sm">{ticket.destination}</td>
      <td className="px-3 py-3 text-sm">{ticket.status}</td>
      <td className="px-3 py-3 text-sm">{ticket.porterId ? ticket.porterId.slice(0, 8) : "-"}</td>
      <td className="px-3 py-3 text-sm">{formatDate(ticket.createdAt)}</td>
      <td className="px-3 py-3">
        <Button className="px-3 py-2 text-xs" variant="ghost" onClick={() => onOpen(ticket)}>
          Actions
        </Button>
      </td>
    </tr>
  );
}