"use client";

import { Button } from "./Button";
import { Modal } from "./Modal";

type Props = {
  isOpen: boolean;
  title: string;
  description: string;
  confirmLabel?: string;
  cancelLabel?: string;
  onConfirm: () => void;
  onClose: () => void;
};

export function ConfirmModal({
  isOpen,
  title,
  description,
  confirmLabel = "Confirmer",
  cancelLabel = "Annuler",
  onConfirm,
  onClose,
}: Props) {
  return (
    <Modal isOpen={isOpen} onClose={onClose} title={title}>
      <p className="mb-4 text-sm text-slate-600">{description}</p>
      <div className="flex justify-end gap-3">
        <Button variant="ghost" onClick={onClose}>
          {cancelLabel}
        </Button>
        <Button variant="danger" onClick={onConfirm}>
          {confirmLabel}
        </Button>
      </div>
    </Modal>
  );
}