export type DispatchPorter = {
  id: string;
  status: string;
  skills: string[];
  completedMissionsToday: number;
  rating: number;
};

export type DispatchTicket = {
  priority: number;
  mode: string;
  needsO2: boolean;
};

export function calculateScore(porter: DispatchPorter, ticket: DispatchTicket): number {
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

export function getRecommendations(
  ticket: DispatchTicket,
  porters: DispatchPorter[],
): Array<{ porter: DispatchPorter; score: number }> {
  return porters
    .map((porter) => ({ porter, score: calculateScore(porter, ticket) }))
    .filter((item) => item.score > 0)
    .sort((a, b) => b.score - a.score)
    .slice(0, 3);
}