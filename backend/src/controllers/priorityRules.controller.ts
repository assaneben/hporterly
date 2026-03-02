import type { Response } from "express";
import { z } from "zod";

import { prisma } from "../config/prisma";
import {
  buildPriorityContextFromTicket,
  evaluatePriorityContext,
  getDefaultPriorityConfig,
  getPriorityRuntimeConfig,
  replacePriorityConfig,
  restoreDefaultPriorityConfig,
} from "../services/priority.service";
import type { PriorityRuntimeConfig, RequestWithUser } from "../types";

const runtimeConfigSchema = z.object({
  fallbackReason: z.string(),
  levels: z.record(z.object({ enabled: z.boolean() })),
  rules: z.array(
    z.object({
      id: z.string(),
      level: z.number().int().min(1).max(4),
      enabled: z.boolean().optional(),
      combinator: z.enum(["AND", "OR"]),
      conditions: z.array(
        z.object({
          field: z.string(),
          operator: z.enum(["is_checked", "is_unchecked", "equals", "contains", "contains_any"]),
          value: z.union([z.string(), z.array(z.string())]).optional(),
        }),
      ),
      reason: z.string(),
    }),
  ),
});

export const replaceConfigSchema = z.object({
  rules_json: runtimeConfigSchema,
});

export async function getPriorityRulesController(_req: RequestWithUser, res: Response): Promise<void> {
  const configs = await prisma.priorityRulesConfig.findMany({
    orderBy: { updatedAt: "desc" },
    take: 10,
  });

  res.json(configs);
}

export async function getPriorityRuntimeController(_req: RequestWithUser, res: Response): Promise<void> {
  const runtime = await getPriorityRuntimeConfig(prisma);
  res.json(runtime);
}

export function getPriorityDefaultsController(_req: RequestWithUser, res: Response): void {
  res.json(getDefaultPriorityConfig());
}

export async function replacePriorityRulesController(req: RequestWithUser, res: Response): Promise<void> {
  const payload = replaceConfigSchema.parse(req.body);

  const replaced = await replacePriorityConfig(prisma, payload.rules_json as unknown as PriorityRuntimeConfig, req.user?.id);
  res.json(replaced);
}

export async function restorePriorityRulesController(req: RequestWithUser, res: Response): Promise<void> {
  const restored = await restoreDefaultPriorityConfig(prisma, req.user?.id);
  res.json(restored);
}

export async function evaluateTicketPriorityController(req: RequestWithUser, res: Response): Promise<void> {
  const id = z.string().min(1).parse(req.params.id);

  const ticket = await prisma.ticket.findUnique({ where: { id } });
  if (!ticket) {
    res.status(404).json({ message: "Ticket introuvable" });
    return;
  }

  const runtime = await getPriorityRuntimeConfig(prisma);
  const context = buildPriorityContextFromTicket(ticket);
  const evaluation = evaluatePriorityContext(context, runtime);

  res.json({
    ticket_id: ticket.id,
    context,
    evaluation,
  });
}