"use client";

import { useCallback } from "react";

import { api } from "@/lib/api";
import type { Porter } from "@/lib/types";
import { usePorterStore } from "@/stores/porterStore";

export function usePorters() {
  const porters = usePorterStore((state) => state.porters);
  const setPorters = usePorterStore((state) => state.setPorters);
  const updatePorter = usePorterStore((state) => state.updatePorter);

  const fetchPorters = useCallback(async () => {
    const data = await api.get<Porter[]>("/porters");
    setPorters(data);
  }, [setPorters]);

  const updateStatus = useCallback(
    async (porterId: string, status: Porter["status"], location?: string) => {
      const updated = await api.patch<Porter>(`/porters/${porterId}/status`, {
        status,
        location,
      });
      updatePorter(updated);
      return updated;
    },
    [updatePorter],
  );

  return {
    porters,
    fetchPorters,
    updateStatus,
  };
}