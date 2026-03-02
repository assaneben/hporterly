import type { Response } from "express";
import bcrypt from "bcryptjs";
import { z } from "zod";

import { prisma } from "../config/prisma";
import { logAudit } from "../services/audit.service";
import type { RequestWithUser } from "../types";

const idSchema = z.object({ id: z.string().min(1) });

export const createUserSchema = z.object({
  username: z.string().min(3),
  password: z.string().min(8),
  role: z.enum(["demandeur", "brancardier", "regulateur", "administrateur"]),
  first_name: z.string().min(1),
  last_name: z.string().min(1),
  email: z.string().email().optional(),
  service: z.string().optional(),
  skills: z.array(z.string()).default([]),
});

export const updateUserSchema = z.object({
  first_name: z.string().min(1).optional(),
  last_name: z.string().min(1).optional(),
  email: z.string().email().optional(),
  service: z.string().optional(),
  role: z.enum(["demandeur", "brancardier", "regulateur", "administrateur"]).optional(),
  is_active: z.boolean().optional(),
});

export const hardDeleteUserSchema = z.object({
  confirmation: z.string().optional(),
});

export async function listUsersController(_req: RequestWithUser, res: Response): Promise<void> {
  const users = await prisma.user.findMany({
    include: { porter: true },
    orderBy: { createdAt: "desc" },
  });
  res.json(users);
}

export async function createUserController(req: RequestWithUser, res: Response): Promise<void> {
  const payload = createUserSchema.parse(req.body);
  const passwordHash = await bcrypt.hash(payload.password, 10);

  const created = await prisma.$transaction(async (tx) => {
    const user = await tx.user.create({
      data: {
        username: payload.username,
        passwordHash,
        role: payload.role,
        firstName: payload.first_name,
        lastName: payload.last_name,
        email: payload.email,
        service: payload.service,
      },
    });

    if (payload.role === "brancardier") {
      await tx.porter.create({
        data: {
          userId: user.id,
          skills: payload.skills,
        },
      });
    }

    return tx.user.findUnique({ where: { id: user.id }, include: { porter: true } });
  });

  await logAudit(prisma, {
    req,
    action: "user_created",
    entityType: "user",
    entityId: created!.id,
    newValue: created,
  });

  res.status(201).json(created);
}

export async function updateUserController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);
  const payload = updateUserSchema.parse(req.body);

  const oldUser = await prisma.user.findUnique({ where: { id }, include: { porter: true } });
  if (!oldUser) {
    res.status(404).json({ message: "Utilisateur introuvable" });
    return;
  }

  const updated = await prisma.user.update({
    where: { id },
    data: {
      firstName: payload.first_name,
      lastName: payload.last_name,
      email: payload.email,
      service: payload.service,
      role: payload.role,
      isActive: payload.is_active,
    },
    include: { porter: true },
  });

  await logAudit(prisma, {
    req,
    action: "user_updated",
    entityType: "user",
    entityId: id,
    oldValue: oldUser,
    newValue: updated,
  });

  res.json(updated);
}

export async function deleteUserController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);

  const user = await prisma.user.findUnique({ where: { id } });
  if (!user) {
    res.status(404).json({ message: "Utilisateur introuvable" });
    return;
  }

  const updated = await prisma.user.update({
    where: { id },
    data: { isActive: false },
  });

  await logAudit(prisma, {
    req,
    action: "user_soft_deleted",
    entityType: "user",
    entityId: id,
    oldValue: user,
    newValue: updated,
  });

  res.json(updated);
}

export async function hardDeleteUserController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);
  hardDeleteUserSchema.parse(req.body);

  const user = await prisma.user.findUnique({ where: { id }, include: { porter: true } });
  if (!user) {
    res.status(404).json({ message: "Utilisateur introuvable" });
    return;
  }

  await prisma.$transaction(async (tx) => {
    await tx.auditLog.deleteMany({ where: { userId: id } });
    await tx.notificationPreference.deleteMany({ where: { userId: id } });
    await tx.notification.deleteMany({ where: { userId: id } });

    if (user.porter) {
      await tx.ticketAssignment.deleteMany({ where: { porterId: user.porter.id } });
      await tx.porter.delete({ where: { id: user.porter.id } });
    }

    await tx.user.delete({ where: { id } });
  });

  await logAudit(prisma, {
    req,
    action: "user_hard_deleted",
    entityType: "user",
    entityId: id,
    oldValue: user,
  });

  res.status(204).send();
}