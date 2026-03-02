import { cx } from "@/lib/utils";

const items = [
  { value: "PATIENT", label: "Patient", description: "Transport d'un patient" },
  { value: "EQUIPMENT", label: "Materiel", description: "Transport de materiel medical" },
  { value: "SPECIMEN", label: "Prelevement", description: "Transport de prelevement biologique" },
] as const;

export function TypeSelector({
  value,
  onChange,
}: {
  value: "PATIENT" | "EQUIPMENT" | "SPECIMEN";
  onChange: (value: "PATIENT" | "EQUIPMENT" | "SPECIMEN") => void;
}) {
  return (
    <div className="grid gap-3 md:grid-cols-3">
      {items.map((item) => (
        <button
          key={item.value}
          className={cx(
            "hply-lift rounded-xl border border-slate-200 bg-slate-50 p-4 text-left text-slate-900 transition hover:border-sky-300 hover:bg-sky-50",
            value === item.value && "border-sky-400 bg-sky-50 shadow-sm",
          )}
          onClick={() => onChange(item.value)}
          type="button"
        >
          <p className="font-title text-lg font-semibold">{item.label}</p>
          <p className="text-sm text-slate-600">{item.description}</p>
        </button>
      ))}
    </div>
  );
}
