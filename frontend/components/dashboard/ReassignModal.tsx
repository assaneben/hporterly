"use client";

import { useEffect, useState } from "react";

import { Button } from "@/components/ui/Button";
import { EmptyState } from "@/components/ui/EmptyState";
import { InlineFeedback } from "@/components/ui/InlineFeedback";
import { Modal } from "@/components/ui/Modal";
import { api } from "@/lib/api";
import type { Porter } from "@/lib/types";

type Props = {
  ticketId: string | null;
  isOpen: boolean;
  onClose: () => void;
  onReassigned: (porterId: string) => void;
};

export function ReassignModal({ ticketId, isOpen, onClose, onReassigned }: Props) {
  const [porters, setPorters] = useState<Porter[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!isOpen || !ticketId) {
      setPorters([]);
      setError(null);
      return;
    }

    setLoading(true);
    setError(null);

    api
      .get<Porter[]>("/porters")
      .then(setPorters)
      .catch(() => {
        setPorters([]);
        setError("Impossible de charger les brancardiers.");
      })
      .finally(() => {
        setLoading(false);
      });
  }, [isOpen, ticketId]);

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="Reassigner le ticket">
      <div className="space-y-2">
        {error ? <InlineFeedback message={error} tone="error" /> : null}

        {loading ? <p className="text-sm text-slate-500">Chargement de la liste...</p> : null}

        {!loading && !error && porters.length === 0 ? (
          <EmptyState title="Aucun brancardier disponible" description="Veuillez reessayer plus tard." className="py-5" />
        ) : null}

        {!loading
          ? porters.map((porter) => (
              <button
                key={porter.id}
                data-testid="reassign-option"
                className="w-full rounded border border-slate-200 px-3 py-2 text-left hover:bg-slate-50"
                onClick={() => onReassigned(porter.id)}
                type="button"
              >
                {porter.user?.firstName} {porter.user?.lastName}
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
