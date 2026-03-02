import { beforeEach, describe, expect, it } from "vitest";

import { enqueueOutbox, getRetryDelay, loadOutbox, processOutbox } from "./outbox";

class LocalStorageMock {
  private store = new Map<string, string>();

  clear() {
    this.store.clear();
  }

  getItem(key: string) {
    return this.store.get(key) ?? null;
  }

  setItem(key: string, value: string) {
    this.store.set(key, value);
  }

  removeItem(key: string) {
    this.store.delete(key);
  }
}

describe("outbox", () => {
  beforeEach(() => {
    const localStorage = new LocalStorageMock();
    (globalThis as { window?: unknown }).window = { localStorage };
  });

  it("applique le backoff exponentiel", () => {
    expect(getRetryDelay(1)).toBe(5_000);
    expect(getRetryDelay(2)).toBe(10_000);
    expect(getRetryDelay(3)).toBe(20_000);
  });

  it("retire un item apres succes sync", async () => {
    enqueueOutbox({
      id: "1",
      type: "create_ticket",
      path: "/tickets",
      method: "POST",
      body: { ok: true },
      created_at: new Date().toISOString(),
      owner_user_id: "u1",
    });

    const initial = loadOutbox();
    expect(initial.items.length).toBe(1);

    const result = await processOutbox({
      ownerUserId: "u1",
      runRequest: async () => undefined,
    });

    expect(result.items.length).toBe(0);
  });
});