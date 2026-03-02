import type { Response } from "express";
import bcrypt from "bcryptjs";
import { z } from "zod";

import { prisma } from "../config/prisma";
import { logAudit } from "../services/audit.service";
import type { RequestWithUser } from "../types";

const porterIdSchema = z.object({ id: z.string().min(1) });

export const createPorterSchema = z.object({
  username: z.string().min(3),
  password: z.string().min(8),
  first_name: z.string().min(1),
  last_name: z.string().min(1),
  email: z.string().email().optional(),
  service: z.string().optional(),
  skills: z.array(z.string()).default([]),
});

export const porterStatusSchema = z.object({
  status: z.enum(["available", "busy", "break", "offline"]),
  location: z.string().optional(),
});

export const porterSkillsSchema = z.object({
  skills: z.array(z.string()),
});

export async function listPortersController(_req: RequestWithUser, res: Response): Promise<void> {
  const porters = await prisma.porter.findMany({
    include: {
      user: {
        select: {
          id: true,
          username: true,
          firstName: true,
          lastName: true,
          email: true,
          service: true,
          isActive: true,
        },
      },
    },
    orderBy: [{ status: "asc" }, { createdAt: "desc" }],
  });

  res.json(porters);
}

export async function createPorterController(req: RequestWithUser, res: Response): Promise<void> {
  const payload = createPorterSchema.parse(req.body);

  const passwordHash = await bcrypt.hash(payload.password, 10);

  const porter = await prisma.$transaction(async (tx) => {
    const user = await tx.user.create({
      data: {
        username: payload.username,
        passwordHash,
        role: "brancardier",
        firstName: payload.first_name,
        lastName: payload.last_name,
        email: payload.email,
        service: payload.service,
      },
    });

    return tx.porter.create({
      data: {
        userId: user.id,
        skills: payload.skills,
      },
      include: {
        user: {
          select: {
            id: true,
            username: true,
            firstName: true,
            lastName: true,
            email: true,
            service: true,
          },
        },
      },
    });
  });

  await logAudit(prisma, {
    req,
    action: "porter_created",
    entityType: "porter",
    entityId: porter.id,
    newValue: porter,
  });

  res.status(201).json(porter);
}

export async function updatePorterStatusController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = porterIdSchema.parse(req.params);
  const payload = porterStatusSchema.parse(req.body);

  const oldPorter = await prisma.porter.findUnique({ where: { id } });
  if (!oldPorter) {
    res.status(404).json({ message: "Brancardier introuvable" });
    return;
  }

  const porter = await prisma.porter.update({
    where: { id },
    data: {
      status: payload.status,
      currentLocation: payload.location,
    },
  });

  await logAudit(prisma, {
    req,
    action: "porter_status_updated",
    entityType: "porter",
    entityId: id,
    oldValue: oldPorter,
    newValue: porter,
  });

  res.json(porter);
}

export async function updatePorterSkillsController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = porterIdSchema.parse(req.params);
  const payload = porterSkillsSchema.parse(req.body);

  const oldPorter = await prisma.porter.findUnique({ where: { id } });
  if (!oldPorter) {
    res.status(404).json({ message: "Brancardier introuvable" });
    return;
  }

  const porter = await prisma.porter.update({
    where: { id },
    data: { skills: payload.skills },
  });

  await logAudit(prisma, {
    req,
    action: "porter_skills_updated",
    entityType: "porter",
    entityId: id,
    oldValue: oldPorter,
    newValue: porter,
  });

  res.json(porter);
}