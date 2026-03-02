import { STORAGE_KEYS } from "./constants";

export type OutboxMethod = "POST" | "PATCH" | "DELETE";

export type OutboxItem = {
  id: string;
  type: string;
  path: string;
  method: OutboxMethod;
  body: Record<string, unknown>;
  meta?: { optimistic_id?: string };
  created_at: string;
  owner_user_id: string;
  retries: number;
  next_attempt_at: number;
  last_error?: string;
  queue_only_on_network?: boolean;
};

export type OutboxPayload = {
  version: 1;
  items: OutboxItem[];
};

const BASE_DELAY = 5_000;
const MAX_DELAY = 300_000;
const MAX_RETRIES = 5;

const RETRIABLE_CODES = new Set([408, 425, 429, 500, 502, 503, 504]);
const NETWORK_ERRORS = ["failed to fetch", "networkerror", "load failed"];

function getEmptyOutbox(): OutboxPayload {
  return { version: 1, items: [] };
}

export function loadOutbox(): OutboxPayload {
  if (typeof window === "undefined") {
    return getEmptyOutbox();
  }

  const raw = window.localStorage.getItem(STORAGE_KEYS.outbox);
  if (!raw) {
    return getEmptyOutbox();
  }

  try {
    const parsed = JSON.parse(raw) as OutboxPayload;
    if (parsed.version !== 1 || !Array.isArray(parsed.items)) {
      return getEmptyOutbox();
    }
    return parsed;
  } catch {
    return getEmptyOutbox();
  }
}

export function saveOutbox(payload: OutboxPayload): void {
  if (typeof window === "undefined") {
    return;
  }

  window.localStorage.setItem(STORAGE_KEYS.outbox, JSON.stringify(payload));
}

export function getRetryDelay(retries: number): number {
  if (retries <= 0) {
    return BASE_DELAY;
  }

  return Math.min(BASE_DELAY * 2 ** (retries - 1), MAX_DELAY);
}

export function shouldRetry(status: number | undefined, errorMessage: string | undefined): boolean {
  if (status && RETRIABLE_CODES.has(status)) {
    return true;
  }

  if (!errorMessage) {
    return false;
  }

  const normalized = errorMessage.toLowerCase();
  return NETWORK_ERRORS.some((fragment) => normalized.includes(fragment));
}

export function enqueueOutbox(item: Omit<OutboxItem, "retries" | "next_attempt_at">): OutboxPayload {
  const current = loadOutbox();
  current.items.push({
    ...item,
    retries: 0,
    next_attempt_at: Date.now(),
  });
  saveOutbox(current);
  return current;
}

export async function processOutbox(params: {
  ownerUserId: string;
  runRequest: (item: OutboxItem) => Promise<void>;
}): Promise<OutboxPayload> {
  const current = loadOutbox();
  const nextItems: OutboxItem[] = [];

  for (const item of current.items) {
    if (item.owner_user_id !== params.ownerUserId) {
      nextItems.push(item);
      continue;
    }

    if (Date.now() < item.next_attempt_at) {
      nextItems.push(item);
      continue;
    }

    try {
      await params.runRequest(item);
    } catch (error) {
      const status = typeof error === "object" && error && "status" in error ? Number((error as { status?: number }).status) : undefined;
      const message = error instanceof Error ? error.message : String(error);
      const retry = shouldRetry(status, message) && item.retries < MAX_RETRIES;

      if (retry) {
        const retries = item.retries + 1;
        nextItems.push({
          ...item,
          retries,
          next_attempt_at: Date.now() + getRetryDelay(retries),
          last_error: message,
        });
      }
    }
  }

  const next = { version: 1 as const, items: nextItems };
  saveOutbox(next);
  return next;
}