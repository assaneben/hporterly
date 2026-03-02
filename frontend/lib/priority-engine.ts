export type PriorityOperator = "is_checked" | "is_unchecked" | "equals" | "contains" | "contains_any";

export type PriorityCondition = {
  field: string;
  operator: PriorityOperator;
  value?: string | string[];
};

export type PriorityRule = {
  id: string;
  level: 1 | 2 | 3 | 4;
  enabled?: boolean;
  combinator: "AND" | "OR";
  conditions: PriorityCondition[];
  reason: string;
};

export type PriorityRuntimeConfig = {
  fallbackReason: string;
  levels: Record<number, { enabled: boolean }>;
  rules: PriorityRule[];
};

export type PriorityEvaluation = {
  priority: 1 | 2 | 3 | 4;
  reason: string;
  ruleId?: string;
  fallback?: boolean;
};

const DEFAULT_CONFIG: PriorityRuntimeConfig = {
  fallbackReason: "Patient stable - aucune precaution",
  levels: {
    1: { enabled: true },
    2: { enabled: true },
    3: { enabled: true },
    4: { enabled: true },
  },
  rules: [
    { id: "n1-vital", level: 1, enabled: true, combinator: "AND", conditions: [{ field: "vital_emergency", operator: "is_checked" }], reason: "Situation vitale engagee" },
    { id: "n1-intubated", level: 1, enabled: true, combinator: "AND", conditions: [{ field: "intubated_ventilated", operator: "is_checked" }], reason: "Patient intube/ventile" },
    {
      id: "n1-monitoring-urgences",
      level: 1,
      enabled: true,
      combinator: "AND",
      conditions: [
        { field: "patient_monitoring", operator: "is_checked" },
        { field: "destination", operator: "contains", value: "urgences" },
      ],
      reason: "Monitoring actif - destination Urgences",
    },
    { id: "n2-monitoring", level: 2, enabled: true, combinator: "AND", conditions: [{ field: "patient_monitoring", operator: "is_checked" }], reason: "Monitoring actif - IDE requis" },
    {
      id: "n2-destination-bloc",
      level: 2,
      enabled: true,
      combinator: "OR",
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
      enabled: true,
      combinator: "AND",
      conditions: [
        { field: "needs_perfusion", operator: "is_checked" },
        { field: "destination", operator: "contains_any", value: ["bloc", "usc"] },
      ],
      reason: "Perfusion active - destination critique",
    },
    { id: "n2-dialysis", level: 2, enabled: true, combinator: "AND", conditions: [{ field: "patient_dialysis", operator: "is_checked" }], reason: "Patient en dialyse" },
    {
      id: "n2-agitated-no-escort",
      level: 2,
      enabled: true,
      combinator: "AND",
      conditions: [
        { field: "patient_agitated", operator: "is_checked" },
        { field: "ide_accompanying_confirmed", operator: "is_unchecked" },
      ],
      reason: "Patient agite sans accompagnant",
    },
    { id: "n3-o2-only", level: 3, enabled: true, combinator: "AND", conditions: [{ field: "needs_o2", operator: "is_checked" }], reason: "Oxygene requis" },
    { id: "n3-perfusion-standard", level: 3, enabled: true, combinator: "AND", conditions: [{ field: "needs_perfusion", operator: "is_checked" }], reason: "Perfusion active" },
    { id: "n3-isolation", level: 3, enabled: true, combinator: "AND", conditions: [{ field: "isolation", operator: "is_checked" }], reason: "Precaution isolement requise" },
    { id: "n3-restrained", level: 3, enabled: true, combinator: "AND", conditions: [{ field: "patient_contentious", operator: "is_checked" }], reason: "Patient contentionne" },
    {
      id: "n3-bariatric",
      level: 3,
      enabled: true,
      combinator: "OR",
      conditions: [
        { field: "patient_bariatric", operator: "is_checked" },
        { field: "patient_over_120kg", operator: "is_checked" },
      ],
      reason: "Materiel adapte requis",
    },
    { id: "n3-psychiatry", level: 3, enabled: true, combinator: "AND", conditions: [{ field: "patient_psychiatry", operator: "is_checked" }], reason: "Patient en psychiatrie" },
    {
      id: "n3-destination-imaging",
      level: 3,
      enabled: true,
      combinator: "OR",
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
      enabled: true,
      combinator: "AND",
      conditions: [
        { field: "inter_service_transfer", operator: "is_checked" },
        { field: "patient_stable", operator: "is_checked" },
      ],
      reason: "Transfert inter-service",
    },
    {
      id: "n4-stable",
      level: 4,
      enabled: true,
      combinator: "AND",
      conditions: [
        { field: "patient_stable", operator: "is_checked" },
        { field: "any_precaution", operator: "is_unchecked" },
      ],
      reason: "Patient stable - aucune precaution",
    },
    {
      id: "n4-consultation",
      level: 4,
      enabled: true,
      combinator: "OR",
      conditions: [
        { field: "destination", operator: "contains", value: "consultation" },
        { field: "destination", operator: "contains", value: "hdj" },
        { field: "destination", operator: "contains", value: "sortie" },
      ],
      reason: "Destination : {destination}",
    },
    { id: "n4-rdv", level: 4, enabled: true, combinator: "AND", conditions: [{ field: "scheduled_time_set", operator: "is_checked" }], reason: "RDV programme" },
  ],
};

function normalize(value: unknown): string {
  return String(value ?? "")
    .normalize("NFD")
    .replace(/[\u0300-\u036f]/g, "")
    .trim()
    .toLowerCase();
}

function evaluateCondition(context: Record<string, unknown>, condition: PriorityCondition): boolean {
  const current = context[condition.field];

  if (condition.operator === "is_checked") {
    return Boolean(current);
  }

  if (condition.operator === "is_unchecked") {
    return !current;
  }

  const currentNormalized = normalize(current);

  if (condition.operator === "equals") {
    return currentNormalized === normalize(condition.value);
  }

  if (condition.operator === "contains") {
    return currentNormalized.includes(normalize(condition.value));
  }

  if (condition.operator === "contains_any") {
    const values = Array.isArray(condition.value) ? condition.value : [];
    return values.some((item) => currentNormalized.includes(normalize(item)));
  }

  return false;
}

function interpolate(reason: string, context: Record<string, unknown>): string {
  return reason.replace(/\{([a-zA-Z0-9_]+)\}/g, (_, key: string) => String(context[key] ?? ""));
}

export function evaluatePriority(
  context: Record<string, unknown>,
  config: PriorityRuntimeConfig = DEFAULT_CONFIG,
): PriorityEvaluation {
  for (const level of [1, 2, 3, 4] as const) {
    if (!config.levels[level]?.enabled) continue;

    const rules = config.rules.filter((rule) => rule.level === level && rule.enabled !== false);

    for (const rule of rules) {
      const matches = rule.conditions.map((condition) => evaluateCondition(context, condition));
      const isMatch = rule.combinator === "AND" ? matches.every(Boolean) : matches.some(Boolean);

      if (isMatch) {
        return {
          priority: level,
          reason: interpolate(rule.reason, context),
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

export function getDefaultPriorityConfig(): PriorityRuntimeConfig {
  return JSON.parse(JSON.stringify(DEFAULT_CONFIG)) as PriorityRuntimeConfig;
}

export function buildPriorityContext(values: {
  origin?: string;
  destination?: string;
  scheduled_time?: string;
  vital_emergency?: boolean;
  intubated_ventilated?: boolean;
  patient_monitoring?: boolean;
  needs_o2?: boolean;
  needs_perfusion?: boolean;
  isolation?: boolean;
  patient_agitated?: boolean;
  ide_accompanying_confirmed?: boolean;
  patient_bariatric?: boolean;
  patient_dialysis?: boolean;
  patient_psychiatry?: boolean;
  patient_contentious?: boolean;
  patient_over_120kg?: boolean;
  patient_stable?: boolean;
  transport_type?: string;
}): Record<string, unknown> {
  const context = {
    vital_emergency: values.vital_emergency ?? false,
    intubated_ventilated: values.intubated_ventilated ?? false,
    patient_monitoring: values.patient_monitoring ?? false,
    needs_o2: values.needs_o2 ?? false,
    needs_perfusion: values.needs_perfusion ?? false,
    isolation: values.isolation ?? false,
    patient_agitated: values.patient_agitated ?? false,
    ide_accompanying_confirmed: values.ide_accompanying_confirmed ?? false,
    patient_bariatric: values.patient_bariatric ?? false,
    patient_dialysis: values.patient_dialysis ?? false,
    patient_psychiatry: values.patient_psychiatry ?? false,
    patient_contentious: values.patient_contentious ?? false,
    patient_over_120kg: values.patient_over_120kg ?? false,
    patient_stable: values.patient_stable ?? true,
    destination: values.destination ?? "",
    transport_type: values.transport_type ?? "PATIENT",
    scheduled_time_set: Boolean(values.scheduled_time),
    inter_service_transfer: normalize(values.origin) !== normalize(values.destination),
    any_precaution: false,
  };

  context.any_precaution = [
    context.vital_emergency,
    context.intubated_ventilated,
    context.needs_o2,
    context.needs_perfusion,
    context.isolation,
    context.patient_monitoring,
    context.patient_agitated,
    context.patient_bariatric,
    context.patient_dialysis,
    context.patient_psychiatry,
    context.patient_contentious,
    context.patient_over_120kg,
  ].some(Boolean);

  return context;
}