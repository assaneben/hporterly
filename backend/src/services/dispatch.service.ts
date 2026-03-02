import type { Ticket } from "@prisma/client";

import type { DispatchRecommendation } from "../types";

type DispatchPorter = DispatchRecommendation["porter"];

export function calculateScore(porter: DispatchPorter, ticket: Ticket): number {
  if (porter.status !== "available") return 0;
  if (ticket.needsO2 && !porter.skills.includes("O2")) return 0;
  if (ticket.priority === 1 && !porter.skills.includes("URG")) return 0;
  if (ticket.mode === "Lit" && !porter.skills.includes("LIT")) return 0;
  if (ticket.mode === "Fauteuil" && !porter.skills.includes("FAUTEUIL")) return 0;

  let score = 100;
  score -= porter.completedMissionsToday * 2;

  if (ticket.priority === 1 && porter.skills.includes("URG")) {
    score += 10;
  }

  score += Math.floor((porter.rating - 3.0) * 2);
  return Math.max(0, Math.min(100, score));
}

export function getRecommendations(ticket: Ticket, porters: DispatchPorter[]): DispatchRecommendation[] {
  return porters
    .map((porter) => ({ porter, score: calculateScore(porter, ticket) }))
    .filter((item) => item.score > 0)
    .sort((a, b) => b.score - a.score)
    .slice(0, 3);
}