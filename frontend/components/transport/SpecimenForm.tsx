import { Input } from "@/components/ui/Input";

type Values = Record<string, string | boolean | undefined>;

export function SpecimenForm({ values, onChange }: { values: Values; onChange: (name: string, value: string | boolean) => void }) {
  return (
    <div className="grid gap-3 md:grid-cols-2">
      <Input
        label="Type de prelevement"
        value={String(values.specimen_types ?? "Prise de sang")}
        onChange={(event) => onChange("specimen_types", event.target.value)}
      />
      <Input label="Laboratoire" value={String(values.laboratory_name ?? "")} onChange={(event) => onChange("laboratory_name", event.target.value)} />
      <Input label="Service de depart" value={String(values.origin ?? "")} onChange={(event) => onChange("origin", event.target.value)} />
      <Input label="Service de destination" value={String(values.destination ?? "")} onChange={(event) => onChange("destination", event.target.value)} />
      <Input label="Notes reception" value={String(values.notes_for_reception ?? "")} onChange={(event) => onChange("notes_for_reception", event.target.value)} />
      <Input label="Note libre" value={String(values.notes ?? "")} onChange={(event) => onChange("notes", event.target.value)} />
    </div>
  );
}