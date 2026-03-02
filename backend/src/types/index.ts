import type { Request } from "express";

export const ROLES = ["demandeur", "brancardier", "regulateur", "administrateur"] as const;
export type Role = (typeof ROLES)[number];

export const TICKET_STATUSES = [
  "pending",
  "assigned",
  "in_progress",
  "arrived",
  "suspended",
  "completed",
  "canceled",
] as const;

export type TicketStatus = (typeof TICKET_STATUSES)[number];

export type AuthUser = {
  id: string;
  username: string;
  role: Role;
  porterId?: string;
};

export type RequestWithUser = Request & {
  user?: AuthUser;
};

export type DispatchRecommendation = {
  porter: {
    id: string;
    userId: string;
    status: string;
    skills: string[];
    completedMissionsToday: number;
    rating: number;
    user: {
      firstName: string;
      lastName: string;
      username: string;
    };
  };
  score: number;
};

export type PriorityRuleCondition = {
  field: string;
  operator: "is_checked" | "is_unchecked" | "equals" | "contains" | "contains_any";
  value?: string | string[];
};

export type PriorityRule = {
  id: string;
  level: 1 | 2 | 3 | 4;
  enabled?: boolean;
  combinator: "AND" | "OR";
  conditions: PriorityRuleCondition[];
  reason: string;
};

export type PriorityRuntimeConfig = {
  fallbackReason: string;
  levels: Record<number, { enabled: boolean }>;
  rules: PriorityRule[];
};

export type PriorityEvaluationResult = {
  priority: 1 | 2 | 3 | 4;
  reason: string;
  ruleId?: string;
  fallback?: boolean;
};