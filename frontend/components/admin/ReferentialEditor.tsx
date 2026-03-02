import { Card } from "@/components/ui/Card";

export function ReferentialEditor({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <Card variant="dashboard" className="space-y-3 rounded-xl border border-slate-200 bg-white p-4 shadow-sm">
      <h2 className="font-title text-lg font-semibold text-slate-900">{title}</h2>
      <div className="text-sm leading-relaxed text-slate-700">{children}</div>
    </Card>
  );
}