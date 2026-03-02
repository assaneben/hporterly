export const API_BASE_URL = process.env.NEXT_PUBLIC_API_BASE_URL ?? "http://localhost:4000/api";

export const STORAGE_KEYS = {
  authToken: "hporterly_token_v1",
  authUser: "hporterly_user_v1",
  outbox: "hporterly_outbox_v1",
} as const;

export const SLA_MINUTES = 45;

export const ROLE_HOME: Record<string, string> = {
  administrateur: "/dashboard",
  regulateur: "/dashboard",
  brancardier: "/porter",
  demandeur: "/transport/new",
};

export const PRIORITY_LABELS: Record<number, string> = {
  1: "N1 Urgence",
  2: "N2 Prioritaire",
  3: "N3 Standard",
  4: "N4 Programme",
};