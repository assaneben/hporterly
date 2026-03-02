import { Badge } from "@/components/ui/Badge";
import { getPriorityBadgeVariant } from "@/lib/ticket-display";

export function PriorityBadge({ priority, reason }: { priority: number; reason: string }) {
  return (
    <div className="hply-lift rounded-lg border border-slate-200 bg-slate-50 p-3">
      <p className="mb-2 text-sm font-semibold text-slate-700">Priorite calculee</p>
      <div className="flex items-center gap-2">
        <Badge variant={getPriorityBadgeVariant(priority)}>{`N${priority}`}</Badge>
        <span className="text-sm text-slate-700">{reason}</span>
      </div>
    </div>
  );
}
