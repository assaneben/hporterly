import type { Response } from "express";
import { z } from "zod";

import { prisma } from "../config/prisma";
import { logAudit } from "../services/audit.service";
import { getRecommendations } from "../services/dispatch.service";
import {
  evaluatePriorityContext,
  getPriorityRuntimeConfig,
} from "../services/priority.service";
import { canTransition, normalizeStatus } from "../services/ticket-status.service";
import type { RequestWithUser } from "../types";

const idSchema = z.object({ id: z.string().min(1) });
const coPartnerSchema = z.object({ id: z.string().min(1), pid: z.string().min(1) });

export const listTicketsQuerySchema = z.object({
  status: z.string().optional(),
  priority: z.coerce.number().int().min(1).max(4).optional(),
  transport_type: z.string().optional(),
});

export const createTicketSchema = z.object({
  patient_id: z.string().min(1).optional(),
  patient_name: z.string().min(1),
  patient_first_name: z.string().optional(),
  patient_last_name: z.string().optional(),
  patient_dob: z.string().datetime().optional(),
  patient_sex: z.string().optional(),
  patient_ipp: z.string().optional(),
  origin: z.string().min(1),
  destination: z.string().min(1),
  priority: z.number().int().min(1).max(4).optional(),
  mode: z.string().optional(),
  notes: z.string().max(5000).optional(),
  scheduled_time: z.string().datetime().optional(),
  transport_type: z.enum(["PATIENT", "EQUIPMENT", "SPECIMEN"]).default("PATIENT"),
  transport_subtype: z.string().default("TP-BRANC"),
  activation_minutes_before: z.number().int().optional(),
  needs_o2: z.boolean().optional(),
  needs_perfusion: z.boolean().optional(),
  isolation: z.boolean().optional(),
  patient_agitated: z.boolean().optional(),
  patient_monitoring: z.boolean().optional(),
  needs_two_porters: z.boolean().optional(),
  patient_contentious: z.boolean().optional(),
  patient_over_120kg: z.boolean().optional(),
  patient_bariatric: z.boolean().optional(),
  patient_psychiatry: z.boolean().optional(),
  patient_dialysis: z.boolean().optional(),
  patient_icu: z.boolean().optional(),
  motif: z.string().optional(),
  equipment_recipient_patient_id: z.string().optional(),
  equipment_recipient_patient_name: z.string().optional(),
  equipment_size: z.string().optional(),
  equipment_return_service: z.string().optional(),
  laboratory_name: z.string().optional(),
  specimen_types: z.array(z.string()).optional(),
  notes_for_reception: z.string().optional(),
  ide_accompanying_confirmed: z.boolean().optional(),
  patient_stable: z.boolean().optional(),
});

export const statusUpdateSchema = z.object({
  status: z.string().min(1),
  comment: z.string().optional(),
  reason_code: z.string().optional(),
});

export const notesSchema = z.object({
  notes: z.string().max(5000),
});

export const prioritySchema = z.object({
  priority: z.number().int().min(1).max(4),
  reason: z.string().min(2),
});

export const assignSchema = z.object({
  porter_id: z.string().min(1).optional(),
});

export const cancelSchema = z.object({
  reason_code: z.string().default("other"),
  comment: z.string().optional(),
});

export const hardDeleteSchema = z.object({
  reason: z.string().optional(),
});

export const coPartnerCreateSchema = z.object({
  porter_id: z.string().min(1),
});

export const helpSchema = z.object({
  requested_porter_id: z.string().min(1),
});

export const helpRespondSchema = z.object({
  help_request_id: z.string().min(1),
  status: z.enum(["accepted", "rejected"]),
});

export const equipmentStatusSchema = z.object({
  equipment_delivered: z.boolean().optional(),
  equipment_label_returned: z.boolean().optional(),
});

function normalizeText(value: string): string {
  return value
    .normalize("NFD")
    .replace(/[\u0300-\u036f]/g, "")
    .trim()
    .toLowerCase();
}

function buildPriorityContextFromBody(data: z.infer<typeof createTicketSchema>) {
  const origin = data.origin ?? "";
  const destination = data.destination ?? "";

  const context = {
    vital_emergency: data.priority === 1,
    intubated_ventilated: Boolean(data.patient_icu),
    patient_monitoring: Boolean(data.patient_monitoring),
    needs_o2: Boolean(data.needs_o2),
    needs_perfusion: Boolean(data.needs_perfusion),
    isolation: Boolean(data.isolation),
    patient_agitated: Boolean(data.patient_agitated),
    ide_accompanying_confirmed: Boolean(data.ide_accompanying_confirmed),
    patient_bariatric: Boolean(data.patient_bariatric),
    patient_dialysis: Boolean(data.patient_dialysis),
    patient_psychiatry: Boolean(data.patient_psychiatry),
    patient_contentious: Boolean(data.patient_contentious),
    patient_over_120kg: Boolean(data.patient_over_120kg),
    patient_stable: Boolean(data.patient_stable ?? !data.patient_monitoring),
    destination,
    transport_type: data.transport_type,
    scheduled_time_set: Boolean(data.scheduled_time),
    inter_service_transfer: normalizeText(origin) !== normalizeText(destination),
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

function shouldBeVisibleToPorters(
  priority: number,
  scheduledTimeIso?: string,
  activationMinutesBefore?: number,
): boolean {
  if (priority !== 4 || !scheduledTimeIso || !activationMinutesBefore) {
    return true;
  }

  const scheduledTime = new Date(scheduledTimeIso).getTime();
  const activationTime = scheduledTime - activationMinutesBefore * 60_000;

  return Date.now() >= activationTime;
}

export async function listTicketsController(req: RequestWithUser, res: Response): Promise<void> {
  const parsed = listTicketsQuerySchema.parse(req.query);

  const where: Record<string, unknown> = {
    isArchived: false,
  };

  if (parsed.status) where.status = normalizeStatus(parsed.status) ?? parsed.status;
  if (parsed.priority) where.priority = parsed.priority;
  if (parsed.transport_type) where.transportType = parsed.transport_type;

  if (req.user?.role === "demandeur") {
    where.requesterId = req.user.id;
  }

  if (req.user?.role === "brancardier") {
    where.OR = [{ isVisibleToPorters: true }, { porterId: req.user.porterId }];
  }

  const tickets = await prisma.ticket.findMany({
    where,
    orderBy: [{ priority: "asc" }, { createdAt: "desc" }],
    include: {
      assignments: {
        where: { isActive: true },
      },
      requester: {
        select: { id: true, firstName: true, lastName: true, username: true },
      },
    },
  });

  res.json(tickets);
}

export async function createTicketController(req: RequestWithUser, res: Response): Promise<void> {
  const payload = createTicketSchema.parse(req.body);

  const runtimeConfig = await getPriorityRuntimeConfig(prisma);
  const calculated = evaluatePriorityContext(buildPriorityContextFromBody(payload), runtimeConfig);
  const finalPriority = payload.priority ?? calculated.priority;

  const ticket = await prisma.ticket.create({
    data: {
      patientId: payload.patient_id ?? `TMP-${Date.now()}`,
      patientName: payload.patient_name,
      patientFirstName: payload.patient_first_name,
      patientLastName: payload.patient_last_name,
      patientDob: payload.patient_dob ? new Date(payload.patient_dob) : undefined,
      patientSex: payload.patient_sex,
      patientIpp: payload.patient_ipp,
      origin: payload.origin,
      destination: payload.destination,
      priority: finalPriority,
      mode: payload.mode ?? "Brancard",
      notes: payload.notes,
      scheduledTime: payload.scheduled_time ? new Date(payload.scheduled_time) : undefined,
      transportType: payload.transport_type,
      transportSubtype: payload.transport_subtype,
      activationMinutesBefore: payload.activation_minutes_before,
      isVisibleToPorters: shouldBeVisibleToPorters(
        finalPriority,
        payload.scheduled_time,
        payload.activation_minutes_before,
      ),
      needsO2: payload.needs_o2 ?? false,
      needsPerfusion: payload.needs_perfusion ?? false,
      isolation: payload.isolation ?? false,
      patientAgitated: payload.patient_agitated ?? false,
      patientMonitoring: payload.patient_monitoring ?? false,
      needsTwoPorters: payload.needs_two_porters ?? false,
      patientContentious: payload.patient_contentious ?? false,
      patientOver120kg: payload.patient_over_120kg ?? false,
      patientBariatric: payload.patient_bariatric ?? false,
      patientPsychiatry: payload.patient_psychiatry ?? false,
      patientDialysis: payload.patient_dialysis ?? false,
      patientIcu: payload.patient_icu ?? false,
      motif: payload.motif,
      equipmentRecipientPatientId: payload.equipment_recipient_patient_id,
      equipmentRecipientPatientName: payload.equipment_recipient_patient_name,
      equipmentSize: payload.equipment_size,
      equipmentReturnService: payload.equipment_return_service,
      laboratoryName: payload.laboratory_name,
      specimenTypes: payload.specimen_types ?? [],
      notesForReception: payload.notes_for_reception,
      requesterId: req.user!.id,
    },
  });

  await logAudit(prisma, {
    req,
    action: "ticket_created",
    entityType: "ticket",
    entityId: ticket.id,
    newValue: ticket,
  });

  res.status(201).json({
    ticket,
    calculated_priority: calculated,
  });
}

export async function getTicketController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);

  const ticket = await prisma.ticket.findUnique({
    where: { id },
    include: {
      assignments: {
        where: { isActive: true },
        include: {
          porter: {
            include: {
              user: {
                select: {
                  firstName: true,
                  lastName: true,
                  username: true,
                },
              },
            },
          },
        },
      },
      requester: {
        select: {
          id: true,
          username: true,
          firstName: true,
          lastName: true,
        },
      },
    },
  });

  if (!ticket) {
    res.status(404).json({ message: "Ticket introuvable" });
    return;
  }

  if (req.user?.role === "demandeur" && ticket.requesterId !== req.user.id) {
    res.status(403).json({ message: "Acces refuse" });
    return;
  }

  res.json(ticket);
}

export async function updateTicketStatusController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);
  const { status, reason_code, comment } = statusUpdateSchema.parse(req.body);

  const targetStatus = normalizeStatus(status);
  if (!targetStatus) {
    res.status(422).json({ message: "Statut invalide" });
    return;
  }

  const ticket = await prisma.ticket.findUnique({ where: { id } });
  if (!ticket) {
    res.status(404).json({ message: "Ticket introuvable" });
    return;
  }

  if (!canTransition(ticket.status, targetStatus)) {
    res.status(409).json({ message: "Transition de statut invalide" });
    return;
  }

  if (targetStatus === "canceled" && !reason_code) {
    res.status(422).json({ message: "reason_code requis pour annulation" });
    return;
  }

  if (targetStatus === "suspended" && !reason_code) {
    res.status(422).json({ message: "reason_code requis pour suspension" });
    return;
  }

  const updated = await prisma.ticket.update({
    where: { id },
    data: {
      status: targetStatus,
      completedAt: targetStatus === "completed" ? new Date() : ticket.completedAt,
      notes: comment ? `${ticket.notes ?? ""}\n[${new Date().toISOString()}] ${comment}`.trim() : ticket.notes,
    },
  });

  await logAudit(prisma, {
    req,
    action: "ticket_status_updated",
    entityType: "ticket",
    entityId: id,
    oldValue: ticket,
    newValue: updated,
  });

  res.json(updated);
}

export async function updateTicketNotesController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);
  const payload = notesSchema.parse(req.body);

  const ticket = await prisma.ticket.update({
    where: { id },
    data: { notes: payload.notes },
  });

  await logAudit(prisma, {
    req,
    action: "ticket_notes_updated",
    entityType: "ticket",
    entityId: id,
    newValue: { notes: payload.notes },
  });

  res.json(ticket);
}

export async function overrideTicketPriorityController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);
  const payload = prioritySchema.parse(req.body);

  const oldTicket = await prisma.ticket.findUnique({ where: { id } });
  if (!oldTicket) {
    res.status(404).json({ message: "Ticket introuvable" });
    return;
  }

  const ticket = await prisma.ticket.update({
    where: { id },
    data: {
      priority: payload.priority,
      notes: `${oldTicket.notes ?? ""}\n[Override priorite] ${payload.reason}`.trim(),
    },
  });

  await logAudit(prisma, {
    req,
    action: "ticket_priority_overridden",
    entityType: "ticket",
    entityId: id,
    oldValue: { priority: oldTicket.priority },
    newValue: { priority: payload.priority, reason: payload.reason },
  });

  res.json(ticket);
}

export async function assignTicketController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);
  const payload = assignSchema.parse(req.body);

  const ticket = await prisma.ticket.findUnique({ where: { id } });
  if (!ticket) {
    res.status(404).json({ message: "Ticket introuvable" });
    return;
  }

  let porterId = payload.porter_id;

  if (!porterId && req.user?.role === "brancardier") {
    porterId = req.user.porterId;
  }

  if (!porterId) {
    res.status(422).json({ message: "porter_id requis" });
    return;
  }

  const porter = await prisma.porter.findUnique({ where: { id: porterId } });
  if (!porter) {
    res.status(404).json({ message: "Brancardier introuvable" });
    return;
  }

  await prisma.$transaction(async (tx) => {
    await tx.ticketAssignment.updateMany({
      where: { ticketId: id, role: "primary", isActive: true },
      data: { isActive: false, removedAt: new Date() },
    });

    await tx.ticket.update({
      where: { id },
      data: { porterId, status: "assigned" },
    });

    await tx.ticketAssignment.create({
      data: {
        ticketId: id,
        porterId,
        role: "primary",
      },
    });
  });

  const updated = await prisma.ticket.findUnique({ where: { id } });

  await logAudit(prisma, {
    req,
    action: "ticket_assigned",
    entityType: "ticket",
    entityId: id,
    oldValue: ticket,
    newValue: updated,
  });

  res.json(updated);
}

export async function reassignTicketController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);
  const payload = assignSchema.required().parse(req.body);

  const existing = await prisma.ticket.findUnique({ where: { id } });
  if (!existing) {
    res.status(404).json({ message: "Ticket introuvable" });
    return;
  }

  await prisma.$transaction(async (tx) => {
    await tx.ticketAssignment.updateMany({
      where: { ticketId: id, role: "primary", isActive: true },
      data: { isActive: false, removedAt: new Date() },
    });

    await tx.ticket.update({
      where: { id },
      data: {
        porterId: payload.porter_id,
        status: "assigned",
      },
    });

    await tx.ticketAssignment.create({
      data: {
        ticketId: id,
        porterId: payload.porter_id,
        role: "primary",
      },
    });
  });

  const updated = await prisma.ticket.findUnique({ where: { id } });

  await logAudit(prisma, {
    req,
    action: "ticket_reassigned",
    entityType: "ticket",
    entityId: id,
    oldValue: existing,
    newValue: updated,
  });

  res.json(updated);
}

export async function unassignTicketController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);

  const ticket = await prisma.ticket.findUnique({ where: { id } });
  if (!ticket) {
    res.status(404).json({ message: "Ticket introuvable" });
    return;
  }

  await prisma.$transaction(async (tx) => {
    await tx.ticketAssignment.updateMany({
      where: { ticketId: id, isActive: true },
      data: { isActive: false, removedAt: new Date() },
    });

    await tx.ticket.update({
      where: { id },
      data: {
        porterId: null,
        status: "pending",
      },
    });
  });

  const updated = await prisma.ticket.findUnique({ where: { id } });
  res.json(updated);
}

export async function pauseTicketController(req: RequestWithUser, res: Response): Promise<void> {
  req.body = {
    status: "suspended",
    reason_code: req.body?.reason_code ?? "urgent_interruption",
    comment: req.body?.comment,
  };

  await updateTicketStatusController(req, res);
}

export async function cancelTicketController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);
  const payload = cancelSchema.parse(req.body);

  const ticket = await prisma.ticket.findUnique({ where: { id } });
  if (!ticket) {
    res.status(404).json({ message: "Ticket introuvable" });
    return;
  }

  const updated = await prisma.ticket.update({
    where: { id },
    data: {
      status: "canceled",
      notes: `${ticket.notes ?? ""}\n[Annulation:${payload.reason_code}] ${payload.comment ?? ""}`.trim(),
    },
  });

  await logAudit(prisma, {
    req,
    action: "ticket_canceled",
    entityType: "ticket",
    entityId: id,
    oldValue: ticket,
    newValue: updated,
  });

  res.json(updated);
}

export async function hardDeleteTicketController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);
  hardDeleteSchema.parse(req.body);

  const exists = await prisma.ticket.findUnique({ where: { id } });
  if (!exists) {
    res.status(404).json({ message: "Ticket introuvable" });
    return;
  }

  await prisma.$transaction(async (tx) => {
    await tx.helpRequest.deleteMany({ where: { ticketId: id } });
    await tx.ticketAssignment.deleteMany({ where: { ticketId: id } });
    await tx.notification.deleteMany({ where: { relatedTicketId: id } });
    await tx.ticket.delete({ where: { id } });
  });

  await logAudit(prisma, {
    req,
    action: "ticket_hard_deleted",
    entityType: "ticket",
    entityId: id,
    oldValue: exists,
  });

  res.status(204).send();
}

export async function ticketRecommendationsController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);

  const ticket = await prisma.ticket.findUnique({ where: { id } });
  if (!ticket) {
    res.status(404).json({ message: "Ticket introuvable" });
    return;
  }

  const porters = await prisma.porter.findMany({
    include: {
      user: {
        select: { firstName: true, lastName: true, username: true },
      },
    },
  });

  const recommendations = getRecommendations(ticket, porters);
  res.json(recommendations);
}

export async function addCoPartnerController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);
  const payload = coPartnerCreateSchema.parse(req.body);

  const ticket = await prisma.ticket.findUnique({ where: { id } });
  if (!ticket) {
    res.status(404).json({ message: "Ticket introuvable" });
    return;
  }

  const assignment = await prisma.ticketAssignment.create({
    data: {
      ticketId: id,
      porterId: payload.porter_id,
      role: "co_partner",
    },
  });

  await logAudit(prisma, {
    req,
    action: "co_partner_added",
    entityType: "ticket_assignment",
    entityId: assignment.id,
    newValue: assignment,
  });

  res.status(201).json(assignment);
}

export async function removeCoPartnerController(req: RequestWithUser, res: Response): Promise<void> {
  const { id, pid } = coPartnerSchema.parse(req.params);

  const assignment = await prisma.ticketAssignment.findFirst({
    where: {
      ticketId: id,
      porterId: pid,
      role: "co_partner",
      isActive: true,
    },
  });

  if (!assignment) {
    res.status(404).json({ message: "Co-partenaire introuvable" });
    return;
  }

  const updated = await prisma.ticketAssignment.update({
    where: { id: assignment.id },
    data: { isActive: false, removedAt: new Date() },
  });

  await logAudit(prisma, {
    req,
    action: "co_partner_removed",
    entityType: "ticket_assignment",
    entityId: assignment.id,
    oldValue: assignment,
    newValue: updated,
  });

  res.json(updated);
}

export async function requestHelpController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);
  const payload = helpSchema.parse(req.body);

  const ticket = await prisma.ticket.findUnique({ where: { id } });
  if (!ticket) {
    res.status(404).json({ message: "Ticket introuvable" });
    return;
  }

  if (!req.user?.porterId) {
    res.status(422).json({ message: "Profil brancardier requis" });
    return;
  }

  const help = await prisma.helpRequest.create({
    data: {
      ticketId: id,
      requestingPorterId: req.user.porterId,
      requestedPorterId: payload.requested_porter_id,
      status: "pending",
    },
  });

  await prisma.ticket.update({
    where: { id },
    data: {
      helpRequested: true,
      helpPorterId: payload.requested_porter_id,
      helpStatus: "pending",
      helpRequestedAt: new Date(),
    },
  });

  res.status(201).json(help);
}

export async function respondHelpController(req: RequestWithUser, res: Response): Promise<void> {
  const payload = helpRespondSchema.parse(req.body);

  const helpRequest = await prisma.helpRequest.findUnique({ where: { id: payload.help_request_id } });
  if (!helpRequest) {
    res.status(404).json({ message: "Demande de renfort introuvable" });
    return;
  }

  const updatedHelp = await prisma.helpRequest.update({
    where: { id: helpRequest.id },
    data: {
      status: payload.status,
      respondedAt: new Date(),
    },
  });

  await prisma.ticket.update({
    where: { id: helpRequest.ticketId },
    data: {
      helpStatus: payload.status,
      helpRequested: payload.status === "accepted",
    },
  });

  res.json(updatedHelp);
}

export async function updateEquipmentStatusController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);
  const payload = equipmentStatusSchema.parse(req.body);

  const updated = await prisma.ticket.update({
    where: { id },
    data: {
      equipmentDelivered: payload.equipment_delivered,
      equipmentLabelReturned: payload.equipment_label_returned,
    },
  });

  res.json(updated);
}