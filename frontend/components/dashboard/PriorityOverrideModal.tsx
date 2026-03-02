"use client";

import { useEffect, useState } from "react";

import { Button } from "@/components/ui/Button";
import { InlineFeedback } from "@/components/ui/InlineFeedback";
import { Input } from "@/components/ui/Input";
import { Modal } from "@/components/ui/Modal";
import { Select } from "@/components/ui/Select";

type Props = {
  isOpen: boolean;
  ticketId: string | null;
  currentPriority: number;
  onClose: () => void;
  onSubmit: (priority: number, reason: string) => void;
};

export function PriorityOverrideModal({ isOpen, ticketId, currentPriority, onClose, onSubmit }: Props) {
  const [priority, setPriority] = useState(String(currentPriority));
  const [reason, setReason] = useState("");
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!isOpen) {
      return;
    }

    setPriority(String(currentPriority));
    setReason("");
    setError(null);
  }, [currentPriority, isOpen]);

  return (
    <Modal isOpen={isOpen} onClose={onClose} title={`Override priorite (${ticketId ?? ""})`}>
      <div className="space-y-3">
        {error ? <InlineFeedback message={error} tone="error" /> : null}

        <Select label="Nouvelle priorite" value={priority} onChange={(event) => setPriority(event.target.value)}>
          <option value="1">N1 Urgence</option>
          <option value="2">N2 Prioritaire</option>
          <option value="3">N3 Standard</option>
          <option value="4">N4 Programme</option>
        </Select>
        <Input label="Raison" value={reason} onChange={(event) => setReason(event.target.value)} />
        <div className="flex justify-end gap-2">
          <Button variant="ghost" onClick={onClose}>
            Annuler
          </Button>
          <Button
            onClick={() => {
              if (!reason.trim()) {
                setError("Une justification est requise.");
                return;
              }
              setError(null);
              onSubmit(Number(priority), reason.trim());
            }}
          >
            Valider
          </Button>
        </div>
      </div>
    </Modal>
  );
}
