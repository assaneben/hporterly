import type { PriorityRuntimeConfig } from "@/lib/priority-engine";

const levelLabel: Record<number, string> = {
  1: "Urgence",
  2: "Prioritaire",
  3: "Standard",
  4: "Programme",
};

const levelTone: Record<number, string> = {
  1: "border-red-200 bg-red-50",
  2: "border-orange-200 bg-orange-50",
  3: "border-blue-200 bg-blue-50",
  4: "border-emerald-200 bg-emerald-50",
};

export function PriorityRuleEditor({ config }: { config: PriorityRuntimeConfig }) {
  return (
    <section className="space-y-4 rounded-xl border border-slate-200 bg-white p-4 shadow-sm">
      <h2 className="font-title text-lg font-semibold text-slate-900">Regles de priorite</h2>
      {[1, 2, 3, 4].map((level) => {
        const rules = config.rules.filter((rule) => rule.level === level);

        return (
          <div key={level} className={`rounded-lg border p-3 ${levelTone[level]}`}>
            <div className="mb-2 flex items-center justify-between gap-2">
              <h3 className="text-sm font-semibold text-slate-700">N{level} - {levelLabel[level]}</h3>
              <span className="rounded-full bg-white/80 px-2 py-0.5 text-xs text-slate-600">{rules.length} regle(s)</span>
            </div>
            <ul className="space-y-1 text-sm text-slate-700">
              {rules.map((rule) => (
                <li key={rule.id} className="rounded bg-white/70 px-2 py-1">
                  {rule.reason}
                </li>
              ))}
            </ul>
          </div>
        );
      })}
    </section>
  );
}