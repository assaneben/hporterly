"use client";

import { useEffect, useMemo, useState } from "react";

import { useToast } from "@/components/providers/ToastProvider";
import { api } from "@/lib/api";
import { useAuthStore } from "@/stores/authStore";

const labels = {
  available: "Disponible",
  busy: "En mission",
  break: "En pause",
  offline: "Hors service",
} as const;

const indicatorClass: Record<keyof typeof labels, string> = {
  available: "bg-emerald-400",
  busy: "bg-amber-400",
  break: "bg-slate-400",
  offline: "bg-red-400",
};

const STORAGE_KEY = "hporterly_porter_status_v1";

function parseStatus(value: string | null): keyof typeof labels {
  if (value === "available" || value === "busy" || value === "break" || value === "offline") {
    return value;
  }
  return "available";
}

export function StatusToggle() {
  const { pushToast } = useToast();
  const user = useAuthStore((state) => state.user);
  const [status, setStatus] = useState<keyof typeof labels>("available");
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    const stored = typeof window !== "undefined" ? window.localStorage.getItem(STORAGE_KEY) : null;
    setStatus(parseStatus(stored));
  }, []);

  const title = useMemo(() => labels[status], [status]);

  const updateStatus = async (nextStatus: keyof typeof labels) => {
    if (nextStatus === status || saving) {
      return;
    }

    const previous = status;
    setStatus(nextStatus);
    setSaving(true);

    try {
      if (user?.porter_id) {
        await api.patch(`/porters/${user.porter_id}/status`, { status: nextStatus });
      }
      window.localStorage.setItem(STORAGE_KEY, nextStatus);
      pushToast({ title: "Statut mis a jour", message: labels[nextStatus], type: "success" });
    } catch {
      setStatus(previous);
      pushToast({ title: "Erreur", message: "Impossible de mettre a jour le statut.", type: "error" });
    } finally {
      setSaving(false);
    }
  };

  return (
    <label className="flex min-h-[44px] items-center gap-2 rounded-full border border-porter-border bg-porter-surface-elev px-3 text-sm text-[#EAF4F9]">
      <span className={`h-2.5 w-2.5 rounded-full ${indicatorClass[status]} animate-msgPulse`} />
      <span className="hidden text-xs text-[#B9D9E6] sm:inline">{saving ? "Mise a jour..." : title}</span>
      <select
        className="rounded-md border-0 bg-transparent text-sm text-[#EAF4F9] outline-none disabled:opacity-60"
        disabled={saving}
        value={status}
        onChange={(event) => updateStatus(event.target.value as keyof typeof labels)}
      >
        {Object.entries(labels).map(([value, label]) => (
          <option key={value} value={value} className="bg-porter-surface text-[#EAF4F9]">
            {label}
          </option>
        ))}
      </select>
    </label>
  );
}
