import { Input } from "@/components/ui/Input";

type Values = Record<string, string | boolean | undefined>;

export function EquipmentForm({ values, onChange }: { values: Values; onChange: (name: string, value: string | boolean) => void }) {
  return (
    <div className="grid gap-3 md:grid-cols-2">
      <Input label="Type d'equipement" value={String(values.transport_subtype ?? "MAT-GENERIQUE")} onChange={(event) => onChange("transport_subtype", event.target.value)} />
      <Input label="Patient destinataire" value={String(values.equipment_recipient_patient_name ?? "")} onChange={(event) => onChange("equipment_recipient_patient_name", event.target.value)} />
      <Input label="Service de depart" value={String(values.origin ?? "")} onChange={(event) => onChange("origin", event.target.value)} />
      <Input label="Service de destination" value={String(values.destination ?? "")} onChange={(event) => onChange("destination", event.target.value)} />
      <Input label="Taille" value={String(values.equipment_size ?? "")} onChange={(event) => onChange("equipment_size", event.target.value)} />
      <Input label="Note libre" value={String(values.notes ?? "")} onChange={(event) => onChange("notes", event.target.value)} />
    </div>
  );
}