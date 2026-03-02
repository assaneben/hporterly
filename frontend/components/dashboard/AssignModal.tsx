"use client";

import { useEffect, useState } from "react";

import { EmptyState } from "@/components/ui/EmptyState";
import { InlineFeedback } from "@/components/ui/InlineFeedback";
import { Modal } from "@/components/ui/Modal";
import { api } from "@/lib/api";
import type { Porter } from "@/lib/types";
import { Button } from "@/components/ui/Button";

type Recommendation = {
  porter: Porter;
  score: number;
};

type Props = {
  ticketId: string | null;
  isOpen: boolean;
  onClose: () => void;
  onAssigned: (porterId: string) => void;
};

export function AssignModal({ ticketId, isOpen, onClose, onAssigned }: Props) {
  const [items, setItems] = useState<Recommendation[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!ticketId || !isOpen) {
      setItems([]);
      setError(null);
      return;
    }

    setLoading(true);
    setError(null);
    api
      .get<Recommendation[]>(`/tickets/${ticketId}/recommendations`)
      .then(setItems)
      .catch(() => {
        setItems([]);
        setError("Impossible de charger les recommandations.");
      })
      .finally(() => {
        setLoading(false);
      });
  }, [isOpen, ticketId]);

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="Assigner un brancardier">
      <div className="space-y-2">
        {error ? <InlineFeedback message={error} tone="error" /> : null}

        {loading ? <p className="text-sm text-slate-500">Chargement des recommandations...</p> : null}

        {!loading && !error && items.length === 0 ? (
          <EmptyState
            title="Aucune recommandation"
            description="Aucun brancardier disponible pour ce ticket."
            className="py-5"
          />
        ) : null}

        {!loading
          ? items.map((item) => (
              <button
                key={item.porter.id}
                data-testid="assign-option"
                className="flex w-full items-center justify-between rounded border border-slate-200 px-3 py-2 text-left hover:bg-slate-50"
                onClick={() => onAssigned(item.porter.id)}
                type="button"
              >
                <span>
                  {item.porter.user?.firstName} {item.porter.user?.lastName}
                </span>
                <span className="text-sm text-slate-500">Score {item.score}</span>
              </button>
            ))
          : null}
      </div>
      <div className="mt-4 flex justify-end">
        <Button variant="ghost" onClick={onClose}>
          Fermer
        </Button>
      </div>
    </Modal>
  );
}
