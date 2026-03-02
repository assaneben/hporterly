import { describe, expect, it } from "vitest";

import { evaluatePriorityContext, getDefaultPriorityConfig } from "./priority.service";

describe("priority.service", () => {
  it("renvoie N1 si situation vitale", () => {
    const result = evaluatePriorityContext(
      {
        vital_emergency: true,
      },
      getDefaultPriorityConfig(),
    );

    expect(result.priority).toBe(1);
    expect(result.reason).toContain("Situation vitale");
  });

  it("renvoie fallback N4 si aucun match", () => {
    const result = evaluatePriorityContext(
      {
        patient_stable: true,
        any_precaution: false,
        destination: "consultation",
      },
      {
        ...getDefaultPriorityConfig(),
        levels: {
          1: { enabled: false },
          2: { enabled: false },
          3: { enabled: false },
          4: { enabled: false },
        },
      },
    );

    expect(result.priority).toBe(4);
    expect(result.fallback).toBe(true);
  });
});