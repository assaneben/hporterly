"use client";

import { useEffect, useState } from "react";

import { DashboardHeader } from "@/components/dashboard/DashboardHeader";
import { ReportsPanel } from "@/components/dashboard/ReportsPanel";
import { InlineFeedback } from "@/components/ui/InlineFeedback";
import { SkeletonLoader } from "@/components/ui/SkeletonLoader";
import { useTickets } from "@/hooks/useTickets";

export default function DashboardReportsPage() {
  const { tickets, loading, fetchTickets } = useTickets();
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    setError(null);
    fetchTickets().catch((fetchError) => {
      setError(fetchError instanceof Error ? fetchError.message : "Impossible de charger les rapports.");
    });
  }, [fetchTickets]);

  return (
    <main id="main-content" className="space-y-4">
      <DashboardHeader />
      {error ? <InlineFeedback message={error} tone="error" /> : null}
      {loading && tickets.length === 0 ? (
        <div className="rounded-xl border border-slate-200 bg-white p-4">
          <SkeletonLoader lines={8} />
        </div>
      ) : (
        <ReportsPanel tickets={tickets} />
      )}
    </main>
  );
}
