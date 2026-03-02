"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";

import { Button } from "@/components/ui/Button";
import { Card } from "@/components/ui/Card";
import { Input } from "@/components/ui/Input";
import { useToast } from "@/components/providers/ToastProvider";
import { getRoleHome, login } from "@/lib/auth";
import { useAuthStore } from "@/stores/authStore";

const demoAccounts = ["admin", "marie.durand", "jean.martin", "regulateur"];

export default function LoginPage() {
  const router = useRouter();
  const { pushToast } = useToast();
  const setSession = useAuthStore((state) => state.setSession);

  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const submit = async (event: React.FormEvent) => {
    event.preventDefault();
    setLoading(true);
    setError(null);

    try {
      const result = await login(username, password);
      setSession(result.token, result.user);
      pushToast({ title: "Connexion reussie", type: "success" });
      router.replace(getRoleHome(result.user.role));
    } catch {
      setError("Identifiants invalides");
      pushToast({ title: "Echec de connexion", message: "Identifiants invalides", type: "error" });
    } finally {
      setLoading(false);
    }
  };

  return (
    <main id="main-content" className="relative flex min-h-screen items-center justify-center overflow-hidden p-4 sm:p-6">
      <div className="pointer-events-none absolute inset-0 bg-[radial-gradient(circle_at_15%_20%,rgba(84,172,191,0.26),transparent_38%),radial-gradient(circle_at_80%_84%,rgba(16,185,129,0.16),transparent_35%)]" />

      <Card className="hply-fade-in hply-glass relative w-full max-w-[440px] rounded-xl px-7 py-8 sm:px-8 sm:py-9" variant="glass">
        <div className="mb-8 text-center">
          <p className="mx-auto mb-3 w-fit rounded-full border border-white/25 bg-white/10 px-3 py-1 text-[11px] uppercase tracking-[0.18em] text-slate-100">
            Plateforme HPly
          </p>
          <h1 className="font-title text-3xl font-bold leading-tight text-white sm:text-[2.1rem]">HPorterly</h1>
          <p className="mx-auto mt-2 max-w-[34ch] text-sm leading-relaxed text-slate-200">
            Connexion securisee a la plateforme de transport interne
          </p>
        </div>

        <form className="space-y-[18px]" onSubmit={submit}>
          <Input
            label="Nom d'utilisateur"
            labelClassName="text-slate-100"
            required
            value={username}
            onChange={(event) => setUsername(event.target.value)}
            className="border-white/20 bg-white/90 text-[15px]"
          />
          <Input
            label="Mot de passe"
            labelClassName="text-slate-100"
            required
            type="password"
            value={password}
            onChange={(event) => setPassword(event.target.value)}
            className="border-white/20 bg-white/90 text-[15px]"
          />
          <Button className="min-h-[46px]" disabled={loading} fullWidth type="submit">
            {loading ? "Connexion..." : "Se connecter"}
          </Button>
        </form>

        {error ? <p className="mt-3 text-center text-sm font-medium text-red-200">{error}</p> : null}
        <p className="mt-2 text-center text-xs text-slate-300">Session de demonstration securisee</p>

        <div className="mt-6 border-t border-white/15 pt-4">
          <p className="text-xs font-semibold uppercase tracking-[0.11em] text-slate-300">Comptes demo</p>
          <div className="mt-2 flex flex-wrap gap-2">
            {demoAccounts.map((account) => (
              <button
                key={account}
                className="rounded-full border border-white/20 bg-white/10 px-3 py-1 text-xs font-medium text-slate-100 transition hover:-translate-y-px hover:bg-white/20"
                onClick={() => {
                  setUsername(account);
                  setPassword("password123");
                }}
                type="button"
              >
                {account}
              </button>
            ))}
          </div>
        </div>
      </Card>
    </main>
  );
}
