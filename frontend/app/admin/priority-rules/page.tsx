"use client";

import { useEffect, useState } from "react";

import { PriorityRuleEditor } from "@/components/admin/PriorityRuleEditor";
import { Button } from "@/components/ui/Button";
import { InlineFeedback } from "@/components/ui/InlineFeedback";
import { api } from "@/lib/api";
import { getDefaultPriorityConfig, type PriorityRuntimeConfig } from "@/lib/priority-engine";

export default function AdminPriorityRulesPage() {
  const [config, setConfig] = useState<PriorityRuntimeConfig>(getDefaultPriorityConfig());
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadRules = async () => {
    setLoading(true);
    setError(null);
    try {
      const runtime = await api.get<PriorityRuntimeConfig>("/priority-rules/runtime");
      setConfig(runtime);
    } catch (fetchError) {
      setConfig(getDefaultPriorityConfig());
      setError(fetchError instanceof Error ? fetchError.message : "Impossible de charger les regles.");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadRules().catch(() => undefined);
  }, []);

  return (
    <section className="space-y-4">
      <div className="rounded-xl border border-slate-200 bg-white px-4 py-3 shadow-sm">
        <div className="flex flex-wrap items-center justify-between gap-3">
          <div>
            <h2 className="font-title text-xl font-semibold text-slate-900">Moteur de priorite</h2>
            <p className="text-sm text-slate-500">Visualisation des regles N1 a N4 actives</p>
          </div>
          <Button disabled={loading} variant="ghost" onClick={() => loadRules().catch(() => undefined)}>
            {loading ? "Chargement..." : "Rafraichir"}
          </Button>
        </div>
      </div>

      {error ? <InlineFeedback message={error} tone="error" /> : null}

      <PriorityRuleEditor config={config} />
    </section>
  );
}
