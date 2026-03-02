"use client";

import { useEffect, useState } from "react";

import { InlineFeedback } from "@/components/ui/InlineFeedback";
import { api } from "@/lib/api";

export default function PorterSettingsPage() {
  const [soundEnabled, setSoundEnabled] = useState(true);
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setLoading(true);
    api
      .get<{ soundEnabled: boolean }>("/notifications/preferences")
      .then((value) => setSoundEnabled(value.soundEnabled))
      .catch(() => {
        setError("Impossible de charger les preferences.");
      })
      .finally(() => {
        setLoading(false);
      });
  }, []);

  return (
    <section className="space-y-3">
      <h1 className="font-title text-2xl font-semibold">Reglages</h1>
      {error ? <InlineFeedback message={error} tone="error" /> : null}
      <div className="rounded-xl border border-porter-border bg-porter-surface p-4">
        {loading ? <p className="text-sm text-[#B9D9E6]">Chargement des preferences...</p> : null}
        <label className="flex min-h-[60px] items-center justify-between gap-3 rounded-lg border border-porter-border bg-porter-surface-elev px-3">
          <span>Activer les sons de notification</span>
          <input
            type="checkbox"
            checked={soundEnabled}
            disabled={saving}
            onChange={(event) => {
              const value = event.target.checked;
              const previous = soundEnabled;
              setSoundEnabled(value);
              setSaving(true);
              api
                .patch("/notifications/preferences", { sound_enabled: value })
                .catch(() => {
                  setSoundEnabled(previous);
                  setError("Impossible de mettre a jour la preference.");
                })
                .finally(() => {
                  setSaving(false);
                });
            }}
          />
        </label>
      </div>
    </section>
  );
}
