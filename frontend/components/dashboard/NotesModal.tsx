"use client";

import { useEffect, useState } from "react";

import { Button } from "@/components/ui/Button";
import { Modal } from "@/components/ui/Modal";

type Props = {
  isOpen: boolean;
  initialNotes?: string;
  onClose: () => void;
  onSave: (notes: string) => void;
};

export function NotesModal({ isOpen, initialNotes, onClose, onSave }: Props) {
  const [notes, setNotes] = useState(initialNotes ?? "");

  useEffect(() => {
    if (!isOpen) {
      return;
    }
    setNotes(initialNotes ?? "");
  }, [initialNotes, isOpen]);

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="Modifier les notes">
      <textarea
        className="min-h-40 w-full rounded-md border border-slate-300 p-3"
        maxLength={5000}
        value={notes}
        onChange={(event) => setNotes(event.target.value)}
      />
      <p className="mt-2 text-xs text-slate-500">{notes.length}/5000 caracteres</p>
      <div className="mt-3 flex justify-end gap-2">
        <Button variant="ghost" onClick={onClose}>
          Annuler
        </Button>
        <Button onClick={() => onSave(notes.trim())}>Enregistrer</Button>
      </div>
    </Modal>
  );
}
