"use client";

import { useMemo } from "react";

import {
  buildPriorityContext,
  evaluatePriority,
  getDefaultPriorityConfig,
  type PriorityRuntimeConfig,
} from "@/lib/priority-engine";

export function usePriorityEngine(
  values: Parameters<typeof buildPriorityContext>[0],
  config?: PriorityRuntimeConfig,
) {
  return useMemo(() => {
    const context = buildPriorityContext(values);
    const runtime = config ?? getDefaultPriorityConfig();
    const result = evaluatePriority(context, runtime);

    return {
      context,
      result,
    };
  }, [config, values]);
}