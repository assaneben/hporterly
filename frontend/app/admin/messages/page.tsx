"use client";

import { MessageComposer } from "@/components/admin/MessageComposer";
import { api } from "@/lib/api";

export default function AdminMessagesPage() {
  return (
    <section className="space-y-4">
      <div className="rounded-xl border border-slate-200 bg-white px-4 py-3 shadow-sm">
        <h2 className="font-title text-xl font-semibold text-slate-900">Messagerie interne</h2>
        <p className="text-sm text-slate-500">Diffusion d'informations vers les equipes</p>
      </div>
      <MessageComposer
        onSend={async (message, targetType) => {
          await api.post("/notifications/send-message", {
            message,
            channel: "general",
            target_type: targetType,
          });
        }}
      />
    </section>
  );
}
