"use client";

import { useEffect, useState } from "react";

import { useToast } from "@/components/providers/ToastProvider";
import { EmptyState } from "@/components/ui/EmptyState";
import { InlineFeedback } from "@/components/ui/InlineFeedback";
import { useNotifications } from "@/hooks/useNotifications";
import { api } from "@/lib/api";
import { formatDate } from "@/lib/utils";

export default function PorterMessagesPage() {
  const { pushToast } = useToast();
  const { notifications, fetchNotifications } = useNotifications();
  const [message, setMessage] = useState("");
  const [loading, setLoading] = useState(false);
  const [sending, setSending] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadNotifications = async () => {
    setLoading(true);
    setError(null);
    try {
      await fetchNotifications();
    } catch {
      setError("Impossible de charger les messages.");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadNotifications().catch(() => undefined);
  }, []);

  return (
    <section className="space-y-3">
      <h1 className="font-title text-2xl font-semibold">Messages</h1>

      {error ? <InlineFeedback message={error} tone="error" /> : null}

      <div className="space-y-2 rounded-xl border border-porter-border bg-porter-surface p-3">
        {loading ? (
          <p className="py-4 text-center text-sm text-[#B9D9E6]">Chargement des messages...</p>
        ) : notifications.length === 0 ? (
          <EmptyState
            title="Boite de reception vide"
            description="Les nouveaux messages apparaitront ici."
            className="border-porter-border bg-porter-surface-elev text-[#EAF4F9]"
          />
        ) : (
          notifications.map((item) => (
            <article key={item.id} className="rounded-lg border border-porter-border bg-porter-surface-elev p-3">
              <p className="text-[11px] uppercase tracking-wide text-[#B9D9E6]">{item.title}</p>
              <p className="mt-1 text-sm">{item.message}</p>
              <p className="mt-1 text-[11px] text-[#B9D9E6]">{formatDate(item.createdAt)}</p>
            </article>
          ))
        )}
      </div>

      <form
        className="flex gap-2"
        onSubmit={(event) => {
          event.preventDefault();
          if (message.trim().length < 2 || sending) return;

          setSending(true);
          api
            .post("/notifications/send-message", {
              message: message.trim(),
              channel: "general",
              target_type: "all_admins",
            })
            .then(() => {
              setMessage("");
              pushToast({ title: "Message envoye", type: "success" });
              return loadNotifications();
            })
            .catch(() => {
              pushToast({ title: "Erreur", message: "Envoi du message impossible.", type: "error" });
            })
            .finally(() => {
              setSending(false);
            });
        }}
      >
        <input
          className="min-h-[60px] flex-1 rounded-lg border border-porter-border bg-porter-surface-elev px-3 text-[#EAF4F9]"
          placeholder="Votre message..."
          value={message}
          onChange={(event) => setMessage(event.target.value)}
        />
        <button
          className="min-h-[60px] rounded-full bg-primary px-4 font-semibold text-[#011C40] disabled:opacity-60"
          disabled={sending}
          type="submit"
        >
          {sending ? "Envoi..." : "Envoyer"}
        </button>
      </form>
    </section>
  );
}
