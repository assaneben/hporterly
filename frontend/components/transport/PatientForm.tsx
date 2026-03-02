import { Checkbox } from "@/components/ui/Checkbox";
import { Input } from "@/components/ui/Input";
import { Select } from "@/components/ui/Select";

type Values = Record<string, string | boolean | undefined>;

export function PatientForm({ values, onChange }: { values: Values; onChange: (name: string, value: string | boolean) => void }) {
  return (
    <div className="grid gap-3 md:grid-cols-2">
      <Input label="Nom complet" value={String(values.patient_name ?? "")} onChange={(event) => onChange("patient_name", event.target.value)} required />
      <Input label="IPP" value={String(values.patient_ipp ?? "")} onChange={(event) => onChange("patient_ipp", event.target.value)} />
      <Input label="Service de depart" value={String(values.origin ?? "")} onChange={(event) => onChange("origin", event.target.value)} required />
      <Input label="Service de destination" value={String(values.destination ?? "")} onChange={(event) => onChange("destination", event.target.value)} required />
      <Select label="Mode de transport" value={String(values.mode ?? "Brancard")} onChange={(event) => onChange("mode", event.target.value)}>
        <option>A pied</option>
        <option>Avec assistance</option>
        <option>Fauteuil</option>
        <option>Brancard</option>
        <option>Lit</option>
      </Select>
      <Input label="Motif" value={String(values.motif ?? "")} onChange={(event) => onChange("motif", event.target.value)} />
      <Checkbox label="Situation vitale engagee" checked={Boolean(values.vital_emergency)} onChange={(event) => onChange("vital_emergency", event.target.checked)} />
      <Checkbox label="Intube/ventile" checked={Boolean(values.intubated_ventilated)} onChange={(event) => onChange("intubated_ventilated", event.target.checked)} />
      <Checkbox label="Sous monitoring" checked={Boolean(values.patient_monitoring)} onChange={(event) => onChange("patient_monitoring", event.target.checked)} />
      <Checkbox label="Oxygene requis" checked={Boolean(values.needs_o2)} onChange={(event) => onChange("needs_o2", event.target.checked)} />
      <Checkbox label="Perfusion active" checked={Boolean(values.needs_perfusion)} onChange={(event) => onChange("needs_perfusion", event.target.checked)} />
      <Checkbox label="Isolement requis" checked={Boolean(values.isolation)} onChange={(event) => onChange("isolation", event.target.checked)} />
      <Checkbox label="Patient agite" checked={Boolean(values.patient_agitated)} onChange={(event) => onChange("patient_agitated", event.target.checked)} />
      <Checkbox label="Accompagnant IDE confirme" checked={Boolean(values.ide_accompanying_confirmed)} onChange={(event) => onChange("ide_accompanying_confirmed", event.target.checked)} />
      <Checkbox label="Patient contentionne" checked={Boolean(values.patient_contentious)} onChange={(event) => onChange("patient_contentious", event.target.checked)} />
      <Checkbox label="Patient bariatrique" checked={Boolean(values.patient_bariatric)} onChange={(event) => onChange("patient_bariatric", event.target.checked)} />
      <Checkbox label="Patient >= 120kg" checked={Boolean(values.patient_over_120kg)} onChange={(event) => onChange("patient_over_120kg", event.target.checked)} />
      <Checkbox label="Patient en psychiatrie" checked={Boolean(values.patient_psychiatry)} onChange={(event) => onChange("patient_psychiatry", event.target.checked)} />
      <Checkbox label="Patient en dialyse" checked={Boolean(values.patient_dialysis)} onChange={(event) => onChange("patient_dialysis", event.target.checked)} />
      <Checkbox label="Necessite 2 brancardiers" checked={Boolean(values.needs_two_porters)} onChange={(event) => onChange("needs_two_porters", event.target.checked)} />
    </div>
  );
}