"use client";

import { useEffect, useRef } from "react";

export function usePolling(task: () => Promise<void> | void, intervalMs: number, enabled = true): void {
  const savedTask = useRef(task);

  useEffect(() => {
    savedTask.current = task;
  }, [task]);

  useEffect(() => {
    if (!enabled) {
      return;
    }

    const run = () => {
      Promise.resolve(savedTask.current()).catch(() => undefined);
    };

    run();
    const id = setInterval(run, intervalMs);
    return () => clearInterval(id);
  }, [enabled, intervalMs]);
}