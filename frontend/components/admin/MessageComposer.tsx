"use client";

import { useState } from "react";

import { Button } from "@/components/ui/Button";
import { InlineFeedback } from "@/components/ui/InlineFeedback";

export function MessageComposer({
  onSend,
}: {
  onSend: (message: string, targetType: string) => Promise<void>;
}) {
  const [message, setMessage] = useState("");
  const [targetType, setTargetType] = useState("all_porters");
  const [sending, setSending] = useState(false);
  const [feedback, setFeedback] = useState<{ tone: "success" | "error"; text: string } | null>(null);

  return (
    <section className="space-y-4 rounded-xl border border-slate-200 bg-white p-4 shadow-sm">
      <h2 className="font-title text-lg font-semibold text-slate-900">Composer un message</h2>

      {feedback ? <InlineFeedback message={feedback.text} tone={feedback.tone} /> : null}

      <select
        className="min-h-[44px] w-full rounded-md border border-slate-300 px-3 py-2 text-sm"
        value={targetType}
        onChange={(event) => setTargetType(event.target.value)}
      >
        <option value="all_admins">Tous les admins</option>
        <option value="all_porters">Tous les brancardiers</option>
        <option value="all_demandeurs">Tous les demandeurs</option>
      </select>
      <textarea
        className="min-h-36 w-full rounded-md border border-slate-300 p-3 text-sm"
        maxLength={1000}
        minLength={2}
        placeholder="Saisir votre message (2 a 1000 caracteres)"
        value={message}
        onChange={(event) => setMessage(event.target.value)}
      />
      <div className="flex justify-end">
        <Button
          disabled={sending}
          onClick={() => {
            if (message.trim().length < 2 || sending) return;
            setSending(true);
            setFeedback(null);
            onSend(message.trim(), targetType)
              .then(() => {
                setMessage("");
                setFeedback({ tone: "success", text: "Message envoye avec succes." });
              })
              .catch((error) => {
                setFeedback({
                  tone: "error",
                  text: error instanceof Error ? error.message : "Envoi du message impossible.",
                });
              })
              .finally(() => {
                setSending(false);
              });
          }}
        >
          {sending ? "Envoi..." : "Envoyer"}
        </Button>
      </div>
    </section>
  );
}
