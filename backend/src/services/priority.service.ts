import type { PrismaClient, Ticket } from "@prisma/client";

import type {
  PriorityEvaluationResult,
  PriorityRule,
  PriorityRuleCondition,
  PriorityRuntimeConfig,
} from "../types";

type PriorityContext = Record<string, boolean | string | null | undefined>;

const PRIORITY_FIELDS_FOR_PRECAUTION = [
  "vital_emergency",
  "intubated_ventilated",
  "needs_o2",
  "needs_perfusion",
  "isolation",
  "patient_monitoring",
  "patient_agitated",
  "patient_bariatric",
  "patient_dialysis",
  "patient_psychiatry",
  "patient_contentious",
  "patient_over_120kg",
] as const;

const DEFAULT_RULES: PriorityRule[] = [
  {
    id: "n1-vital",
    level: 1,
    combinator: "AND",
    enabled: true,
    conditions: [{ field: "vital_emergency", operator: "is_checked" }],
    reason: "Situation vitale engagee",
  },
  {
    id: "n1-intubated",
    level: 1,
    combinator: "AND",
    enabled: true,
    conditions: [{ field: "intubated_ventilated", operator: "is_checked" }],
    reason: "Patient intube/ventile",
  },
  {
    id: "n1-monitoring-urgences",
    level: 1,
    combinator: "AND",
    enabled: true,
    conditions: [
      { field: "patient_monitoring", operator: "is_checked" },
      { field: "destination", operator: "contains", value: "urgences" },
    ],
    reason: "Monitoring actif - destination Urgences",
  },
  {
    id: "n2-monitoring",
    level: 2,
    combinator: "AND",
    enabled: true,
    conditions: [{ field: "patient_monitoring", operator: "is_checked" }],
    reason: "Monitoring actif - IDE requis",
  },
  {
    id: "n2-destination-bloc",
    level: 2,
    combinator: "OR",
    enabled: true,
    conditions: [
      { field: "destination", operator: "contains", value: "bloc operatoire" },
      { field: "destination", operator: "contains", value: "usc" },
      { field: "destination", operator: "contains", value: "sspi" },
      { field: "destination", operator: "contains", value: "salle de reveil" },
    ],
    reason: "Destination : {destination}",
  },
  {
    id: "n2-perfusion-critical",
    level: 2,
    combinator: "AND",
    enabled: true,
    conditions: [
      { field: "needs_perfusion", operator: "is_checked" },
      { field: "destination", operator: "contains_any", value: ["bloc", "usc"] },
    ],
    reason: "Perfusion active - destination critique",
  },
  {
    id: "n2-dialysis",
    level: 2,
    combinator: "AND",
    enabled: true,
    conditions: [{ field: "patient_dialysis", operator: "is_checked" }],
    reason: "Patient en dialyse",
  },
  {
    id: "n2-agitated-no-escort",
    level: 2,
    combinator: "AND",
    enabled: true,
    conditions: [
      { field: "patient_agitated", operator: "is_checked" },
      { field: "ide_accompanying_confirmed", operator: "is_unchecked" },
    ],
    reason: "Patient agite sans accompagnant",
  },
  {
    id: "n3-o2-only",
    level: 3,
    combinator: "AND",
    enabled: true,
    conditions: [{ field: "needs_o2", operator: "is_checked" }],
    reason: "Oxygene requis",
  },
  {
    id: "n3-perfusion-standard",
    level: 3,
    combinator: "AND",
    enabled: true,
    conditions: [{ field: "needs_perfusion", operator: "is_checked" }],
    reason: "Perfusion active",
  },
  {
    id: "n3-isolation",
    level: 3,
    combinator: "AND",
    enabled: true,
    conditions: [{ field: "isolation", operator: "is_checked" }],
    reason: "Precaution isolement requise",
  },
  {
    id: "n3-restrained",
    level: 3,
    combinator: "AND",
    enabled: true,
    conditions: [{ field: "patient_contentious", operator: "is_checked" }],
    reason: "Patient contentionne",
  },
  {
    id: "n3-bariatric",
    level: 3,
    combinator: "OR",
    enabled: true,
    conditions: [
      { field: "patient_bariatric", operator: "is_checked" },
      { field: "patient_over_120kg", operator: "is_checked" },
    ],
    reason: "Materiel adapte requis",
  },
  {
    id: "n3-psychiatry",
    level: 3,
    combinator: "AND",
    enabled: true,
    conditions: [{ field: "patient_psychiatry", operator: "is_checked" }],
    reason: "Patient en psychiatrie",
  },
  {
    id: "n3-destination-imaging",
    level: 3,
    combinator: "OR",
    enabled: true,
    conditions: [
      { field: "destination", operator: "contains", value: "radiologie" },
      { field: "destination", operator: "contains", value: "echo" },
      { field: "destination", operator: "contains", value: "scanner" },
      { field: "destination", operator: "contains", value: "kine" },
    ],
    reason: "Destination : {destination}",
  },
  {
    id: "n3-interservice",
    level: 3,
    combinator: "AND",
    enabled: true,
    conditions: [
      { field: "inter_service_transfer", operator: "is_checked" },
      { field: "patient_stable", operator: "is_checked" },
    ],
    reason: "Transfert inter-service",
  },
  {
    id: "n4-stable",
    level: 4,
    combinator: "AND",
    enabled: true,
    conditions: [
      { field: "patient_stable", operator: "is_checked" },
      { field: "any_precaution", operator: "is_unchecked" },
    ],
    reason: "Patient stable - aucune precaution",
  },
  {
    id: "n4-consultation",
    level: 4,
    combinator: "OR",
    enabled: true,
    conditions: [
      { field: "destination", operator: "contains", value: "consultation" },
      { field: "destination", operator: "contains", value: "hdj" },
      { field: "destination", operator: "contains", value: "sortie" },
    ],
    reason: "Destination : {destination}",
  },
  {
    id: "n4-rdv",
    level: 4,
    combinator: "AND",
    enabled: true,
    conditions: [{ field: "scheduled_time_set", operator: "is_checked" }],
    reason: "RDV programme",
  },
];

const DEFAULT_CONFIG: PriorityRuntimeConfig = {
  fallbackReason: "Patient stable - aucune precaution",
  levels: {
    1: { enabled: true },
    2: { enabled: true },
    3: { enabled: true },
    4: { enabled: true },
  },
  rules: DEFAULT_RULES,
};

function normalizeText(value: unknown): string {
  return String(value ?? "")
    .normalize("NFD")
    .replace(/[\u0300-\u036f]/g, "")
    .trim()
    .toLowerCase();
}

function isNetworkBoolean(value: unknown): boolean {
  return value === true || value === "true" || value === 1;
}

function evaluateCondition(context: PriorityContext, condition: PriorityRuleCondition): boolean {
  const fieldValue = context[condition.field];

  if (condition.operator === "is_checked") {
    return isNetworkBoolean(fieldValue);
  }

  if (condition.operator === "is_unchecked") {
    return !isNetworkBoolean(fieldValue);
  }

  const normalizedValue = normalizeText(fieldValue);

  if (condition.operator === "equals") {
    return normalizedValue === normalizeText(condition.value);
  }

  if (condition.operator === "contains") {
    return normalizedValue.includes(normalizeText(condition.value));
  }

  if (condition.operator === "contains_any") {
    const values = Array.isArray(condition.value) ? condition.value : [];
    return values.some((item) => normalizedValue.includes(normalizeText(item)));
  }

  return false;
}

function interpolateReason(reason: string, context: PriorityContext): string {
  return reason.replace(/\{([a-zA-Z0-9_]+)\}/g, (_match, field) => String(context[field] ?? ""));
}

export function evaluatePriorityContext(
  context: PriorityContext,
  config: PriorityRuntimeConfig = DEFAULT_CONFIG,
): PriorityEvaluationResult {
  for (const level of [1, 2, 3, 4] as const) {
    if (!config.levels[level]?.enabled) {
      continue;
    }

    const levelRules = config.rules.filter((rule) => rule.level === level && rule.enabled !== false);

    for (const rule of levelRules) {
      const matches = rule.conditions.map((condition) => evaluateCondition(context, condition));

      const isMatch = rule.combinator === "AND" ? matches.every(Boolean) : matches.some(Boolean);

      if (isMatch) {
        return {
          priority: level,
          reason: interpolateReason(rule.reason, context),
          ruleId: rule.id,
        };
      }
    }
  }

  return {
    priority: 4,
    reason: config.fallbackReason,
    fallback: true,
  };
}

export function buildPriorityContextFromTicket(ticket: Ticket): PriorityContext {
  const destination = ticket.destination ?? "";
  const origin = ticket.origin ?? "";

  const context: PriorityContext = {
    vital_emergency: ticket.priority === 1 && ticket.patientIcu,
    intubated_ventilated: ticket.patientIcu,
    patient_monitoring: ticket.patientMonitoring,
    needs_o2: ticket.needsO2,
    needs_perfusion: ticket.needsPerfusion,
    isolation: ticket.isolation,
    patient_agitated: ticket.patientAgitated,
    ide_accompanying_confirmed: false,
    patient_bariatric: ticket.patientBariatric,
    patient_dialysis: ticket.patientDialysis,
    patient_psychiatry: ticket.patientPsychiatry,
    patient_contentious: ticket.patientContentious,
    patient_over_120kg: ticket.patientOver120kg,
    patient_stable: !ticket.patientMonitoring && !ticket.patientIcu,
    destination,
    transport_type: ticket.transportType,
    scheduled_time_set: Boolean(ticket.scheduledTime),
    inter_service_transfer: normalizeText(origin) !== normalizeText(destination),
  };

  context.any_precaution = PRIORITY_FIELDS_FOR_PRECAUTION.some((field) => isNetworkBoolean(context[field]));

  return context;
}

function isRuntimeConfig(value: unknown): value is PriorityRuntimeConfig {
  if (!value || typeof value !== "object") {
    return false;
  }

  const candidate = value as PriorityRuntimeConfig;
  return Boolean(candidate.rules) && Array.isArray(candidate.rules) && Boolean(candidate.levels);
}

export function getDefaultPriorityConfig(): PriorityRuntimeConfig {
  return JSON.parse(JSON.stringify(DEFAULT_CONFIG)) as PriorityRuntimeConfig;
}

export async function getPriorityRuntimeConfig(prisma: PrismaClient): Promise<PriorityRuntimeConfig> {
  const config = await prisma.priorityRulesConfig.findFirst({
    where: { isActive: true },
    orderBy: { updatedAt: "desc" },
  });

  if (!config || !isRuntimeConfig(config.rulesJson)) {
    return getDefaultPriorityConfig();
  }

  return config.rulesJson;
}

export async function replacePriorityConfig(
  prisma: PrismaClient,
  rulesJson: PriorityRuntimeConfig,
  updatedBy?: string,
): Promise<PriorityRuntimeConfig> {
  await prisma.priorityRulesConfig.updateMany({
    data: { isActive: false },
    where: { isActive: true },
  });

  const created = await prisma.priorityRulesConfig.create({
    data: {
      rulesJson,
      isActive: true,
      updatedBy,
    },
  });

  return created.rulesJson as PriorityRuntimeConfig;
}

export async function restoreDefaultPriorityConfig(
  prisma: PrismaClient,
  updatedBy?: string,
): Promise<PriorityRuntimeConfig> {
  return replacePriorityConfig(prisma, getDefaultPriorityConfig(), updatedBy);
}