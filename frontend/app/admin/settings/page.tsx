"use client";

import { useEffect, useState } from "react";

import { ReferentialEditor } from "@/components/admin/ReferentialEditor";
import { Button } from "@/components/ui/Button";
import { InlineFeedback } from "@/components/ui/InlineFeedback";
import { api } from "@/lib/api";

type ServiceItem = {
  id: string;
  name: string;
  siteName: string;
  buildingName: string;
  levelName: string;
};

type EquipmentItem = {
  id: string;
  label: string;
  sizes: string[];
};

type ModeItem = {
  id: string;
  label: string;
  sortOrder: number;
};

type SpecimenItem = {
  id: string;
  label: string;
};

export default function AdminSettingsPage() {
  const [services, setServices] = useState<ServiceItem[]>([]);
  const [equipment, setEquipment] = useState<EquipmentItem[]>([]);
  const [modes, setModes] = useState<ModeItem[]>([]);
  const [specimens, setSpecimens] = useState<SpecimenItem[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadReferentials = async () => {
    setLoading(true);
    setError(null);
    try {
      const [nextServices, nextEquipment, nextModes, nextSpecimens] = await Promise.all([
        api.get<ServiceItem[]>("/referentials/services"),
        api.get<EquipmentItem[]>("/referentials/equipment"),
        api.get<ModeItem[]>("/referentials/transport-modes"),
        api.get<SpecimenItem[]>("/referentials/specimens"),
      ]);

      setServices(nextServices);
      setEquipment(nextEquipment);
      setModes(nextModes);
      setSpecimens(nextSpecimens);
    } catch (fetchError) {
      setError(fetchError instanceof Error ? fetchError.message : "Impossible de charger les referentiels.");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadReferentials().catch(() => undefined);
  }, []);

  return (
    <div className="space-y-4">
      <div className="rounded-xl border border-slate-200 bg-white px-4 py-3 shadow-sm">
        <div className="flex flex-wrap items-center justify-between gap-3">
          <div>
            <h2 className="font-title text-xl font-semibold text-slate-900">Parametres referentiels</h2>
            <p className="text-sm text-slate-500">
              Gestion des services, equipements, modes de transport et specimens
            </p>
          </div>
          <Button disabled={loading} variant="ghost" onClick={() => loadReferentials().catch(() => undefined)}>
            {loading ? "Chargement..." : "Rafraichir"}
          </Button>
        </div>
      </div>

      {error ? <InlineFeedback message={error} tone="error" /> : null}

      <ReferentialEditor title="Services">
        <ul className="space-y-1 text-sm text-slate-700">
          {services.length === 0 ? <li className="text-slate-500">Aucun service disponible.</li> : null}
          {services.map((item) => (
            <li key={item.id}>
              {item.name} - {item.siteName} / {item.buildingName} / {item.levelName}
            </li>
          ))}
        </ul>
      </ReferentialEditor>

      <ReferentialEditor title="Equipement">
        <ul className="space-y-1 text-sm text-slate-700">
          {equipment.length === 0 ? <li className="text-slate-500">Aucun equipement disponible.</li> : null}
          {equipment.map((item) => (
            <li key={item.id}>
              {item.label} ({item.sizes.join(", ")})
            </li>
          ))}
        </ul>
      </ReferentialEditor>

      <ReferentialEditor title="Modes transport">
        <ul className="space-y-1 text-sm text-slate-700">
          {modes.length === 0 ? <li className="text-slate-500">Aucun mode configure.</li> : null}
          {modes.map((item) => (
            <li key={item.id}>
              #{item.sortOrder} - {item.label}
            </li>
          ))}
        </ul>
      </ReferentialEditor>

      <ReferentialEditor title="Specimens">
        <ul className="space-y-1 text-sm text-slate-700">
          {specimens.length === 0 ? <li className="text-slate-500">Aucun specimen configure.</li> : null}
          {specimens.map((item) => (
            <li key={item.id}>{item.label}</li>
          ))}
        </ul>
      </ReferentialEditor>
    </div>
  );
}
