import { Badge } from "@/components/ui/Badge";
import { Checkbox } from "@/components/ui/Checkbox";
import { Select } from "@/components/ui/Select";
import type { Porter } from "@/lib/types";

const skills = ["BRANCARD", "FAUTEUIL", "LIT", "O2", "URG"];

const statusLabel: Record<string, string> = {
  available: "Disponible",
  busy: "En mission",
  break: "En pause",
  offline: "Hors service",
};

export function PorterCard({ porter }: { porter: Porter }) {
  return (
    <article className="space-y-4 rounded-xl border border-slate-200 bg-white p-[18px] shadow-sm">
      <div className="flex items-start justify-between gap-3">
        <div>
          <p className="font-title text-lg font-semibold leading-tight text-slate-900">
            {porter.user?.firstName} {porter.user?.lastName}
          </p>
          <p className="mt-0.5 text-xs text-slate-500">{porter.user?.username}</p>
        </div>
        <Badge variant={porter.status === "available" ? "completed" : porter.status === "offline" ? "canceled" : "paused"}>
          {statusLabel[porter.status] ?? porter.status}
        </Badge>
      </div>

      <Select label="Statut" value={porter.status} disabled>
        <option value={porter.status}>{statusLabel[porter.status] ?? porter.status}</option>
      </Select>

      <div className="grid grid-cols-2 gap-2 rounded-lg border border-slate-200 bg-slate-50 p-2">
        {skills.map((skill) => (
          <Checkbox key={skill} checked={porter.skills.includes(skill)} label={skill} readOnly />
        ))}
      </div>
    </article>
  );
}
