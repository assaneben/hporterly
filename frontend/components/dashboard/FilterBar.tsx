"use client";

import { Input } from "@/components/ui/Input";
import { Select } from "@/components/ui/Select";
import { useFilterStore } from "@/stores/filterStore";

const chips = [
  { key: "status", value: "pending", label: "Non assigne" },
  { key: "priority", value: "1", label: "Retard critique" },
  { key: "transportType", value: "PATIENT", label: "Patients" },
  { key: "transportType", value: "EQUIPMENT", label: "Materiel" },
  { key: "transportType", value: "SPECIMEN", label: "Prelevements" },
] as const;

export function FilterBar() {
  const search = useFilterStore((state) => state.search);
  const status = useFilterStore((state) => state.status);
  const priority = useFilterStore((state) => state.priority);
  const transportType = useFilterStore((state) => state.transportType);
  const setFilter = useFilterStore((state) => state.setFilter);
  const reset = useFilterStore((state) => state.reset);

  return (
    <div className="hply-fade-in hply-fade-in-delay-1 space-y-3 rounded-xl border border-dash-border bg-white p-4 shadow-sm">
      <div className="flex items-center justify-between gap-2">
        <p className="text-xs font-semibold uppercase tracking-[0.12em] text-slate-500">Filtres avances</p>
        <button
          className="rounded-md border border-slate-200 px-2.5 py-1 text-xs font-semibold text-slate-600 transition hover:-translate-y-px hover:bg-slate-50"
          onClick={() => reset()}
          type="button"
        >
          Reinitialiser
        </button>
      </div>

      <div className="grid gap-3 md:grid-cols-4">
        <Input
          label="Recherche"
          placeholder="Rechercher patient, service, id..."
          value={search}
          onChange={(event) => setFilter("search", event.target.value)}
        />
        <Select label="Statut" value={status} onChange={(event) => setFilter("status", event.target.value)}>
          <option value="">Tous</option>
          <option value="pending">En attente</option>
          <option value="assigned">Assigne</option>
          <option value="in_progress">En cours</option>
          <option value="suspended">Suspendu</option>
          <option value="completed">Termine</option>
        </Select>
        <Select label="Priorite" value={priority} onChange={(event) => setFilter("priority", event.target.value)}>
          <option value="">Toutes</option>
          <option value="1">N1 Urgence</option>
          <option value="2">N2 Prioritaire</option>
          <option value="3">N3 Standard</option>
          <option value="4">N4 Programme</option>
        </Select>
        <Select
          label="Type"
          value={transportType}
          onChange={(event) => setFilter("transportType", event.target.value)}
        >
          <option value="">Tous</option>
          <option value="PATIENT">Patient</option>
          <option value="EQUIPMENT">Materiel</option>
          <option value="SPECIMEN">Prelevement</option>
        </Select>
      </div>

      <div className="flex flex-wrap gap-2">
        {chips.map((chip) => {
          const active =
            (chip.key === "status" && status === chip.value) ||
            (chip.key === "priority" && priority === chip.value) ||
            (chip.key === "transportType" && transportType === chip.value);

          return (
            <button
              key={`${chip.key}-${chip.value}`}
              className={`rounded-full border px-3 py-1.5 text-xs font-semibold transition ${
                active
                  ? "border-blue-300 bg-blue-50 text-blue-700"
                  : "border-slate-200 bg-slate-50 text-slate-600 hover:-translate-y-px hover:bg-slate-100"
              }`}
              onClick={() => setFilter(chip.key, active ? "" : chip.value)}
              type="button"
            >
              {chip.label}
            </button>
          );
        })}
      </div>
    </div>
  );
}
