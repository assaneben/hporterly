"use client";

import { useEffect, useState } from "react";

import { PorterCard } from "@/components/admin/PorterCard";
import { Button } from "@/components/ui/Button";
import { EmptyState } from "@/components/ui/EmptyState";
import { InlineFeedback } from "@/components/ui/InlineFeedback";
import { usePorters } from "@/hooks/usePorters";

export default function AdminPortersPage() {
  const { porters, fetchPorters } = usePorters();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadPorters = async () => {
    setLoading(true);
    setError(null);
    try {
      await fetchPorters();
    } catch (fetchError) {
      setError(fetchError instanceof Error ? fetchError.message : "Impossible de charger les brancardiers.");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadPorters().catch(() => undefined);
  }, []);

  return (
    <section className="space-y-4">
      <div className="rounded-xl border border-slate-200 bg-white px-4 py-3 shadow-sm">
        <div className="flex flex-wrap items-center justify-between gap-3">
          <div>
            <h2 className="font-title text-xl font-semibold text-slate-900">Gestion brancardiers</h2>
            <p className="text-sm leading-relaxed text-slate-500">
              Suivi des statuts et competences des equipes terrain
            </p>
          </div>
          <Button disabled={loading} variant="ghost" onClick={() => loadPorters().catch(() => undefined)}>
            {loading ? "Chargement..." : "Rafraichir"}
          </Button>
        </div>
      </div>

      {error ? <InlineFeedback message={error} tone="error" /> : null}

      {loading ? <p className="text-sm text-slate-500">Chargement des brancardiers...</p> : null}

      {!loading && porters.length === 0 ? (
        <EmptyState
          title="Aucun brancardier trouve"
          description="Ajoutez des profils brancardiers pour commencer la planification."
        />
      ) : null}

      <div className="grid gap-4 md:grid-cols-2">
        {porters.map((porter) => (
          <PorterCard key={porter.id} porter={porter} />
        ))}
      </div>
    </section>
  );
}
