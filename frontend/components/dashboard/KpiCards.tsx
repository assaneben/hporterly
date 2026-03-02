import { Card } from "@/components/ui/Card";

type Props = {
  total: number;
  inProgress: number;
  completed: number;
  canceled: number;
};

export function KpiCards({ total, inProgress, completed, canceled }: Props) {
  const sla = total === 0 ? 100 : Math.max(0, Math.round(((completed + canceled) / total) * 100));

  const items = [
    { label: "Total demandes", value: total, tone: "from-sky-50 to-blue-50" },
    { label: "En cours", value: inProgress, tone: "from-blue-50 to-indigo-50" },
    { label: "Terminees", value: completed, tone: "from-emerald-50 to-green-50" },
    { label: "Annulees", value: canceled, tone: "from-rose-50 to-red-50" },
    { label: "SLA (indicatif)", value: `${sla}%`, tone: "from-amber-50 to-yellow-50" },
  ];

  return (
    <div className="grid gap-3 md:grid-cols-3 xl:grid-cols-5">
      {items.map((item) => (
        <Card key={item.label} className={`hply-fade-in hply-lift bg-gradient-to-br ${item.tone} p-4`} variant="dashboard">
          <p className="text-[11px] uppercase tracking-[0.12em] text-slate-500">{item.label}</p>
          <p className="mt-1.5 font-title text-3xl font-semibold leading-none text-slate-900">{item.value}</p>
          <p className="mt-2 text-xs text-slate-500">Mise a jour temps reel</p>
        </Card>
      ))}
    </div>
  );
}
