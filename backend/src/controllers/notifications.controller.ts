import type { Prisma, User } from "@prisma/client";
import type { Response } from "express";
import { z } from "zod";

import { prisma } from "../config/prisma";
import {
  createNotification,
  getOrCreateNotificationPreferences,
} from "../services/notification.service";
import type { RequestWithUser } from "../types";

export const notificationQuerySchema = z.object({
  limit: z.coerce.number().int().positive().max(200).default(50),
  notification_type: z.string().optional(),
});

export const markReadSchema = z.object({
  notification_ids: z.array(z.string().min(1)).min(1),
});

export const updatePreferencesSchema = z.object({
  preferences: z.record(z.any()).optional(),
  sound_enabled: z.boolean().optional(),
});

export const sendMessageSchema = z.object({
  message: z.string().min(2).max(1000),
  channel: z.string().optional(),
  target_type: z.enum([
    "admin",
    "porter",
    "demandeur",
    "all_admins",
    "all_porters",
    "all_demandeurs",
  ]),
  target_user_id: z.string().optional(),
  target_porter_id: z.string().optional(),
});

function withRecipientFilter(
  payload: z.infer<typeof sendMessageSchema>,
): { role?: string; userId?: string; porterId?: string } {
  switch (payload.target_type) {
    case "admin":
    case "demandeur":
      return {
        role: payload.target_type === "admin" ? "administrateur" : "demandeur",
        userId: payload.target_user_id,
      };
    case "porter":
      return {
        role: "brancardier",
        porterId: payload.target_porter_id,
        userId: payload.target_user_id,
      };
    case "all_admins":
      return { role: "administrateur" };
    case "all_porters":
      return { role: "brancardier" };
    case "all_demandeurs":
      return { role: "demandeur" };
    default:
      return {};
  }
}

export async function listNotificationsController(req: RequestWithUser, res: Response): Promise<void> {
  const query = notificationQuerySchema.parse(req.query);

  const notifications = await prisma.notification.findMany({
    where: {
      userId: req.user!.id,
      notificationType: query.notification_type,
    },
    orderBy: { createdAt: "desc" },
    take: query.limit,
  });

  res.json(notifications);
}

export async function unreadCountController(req: RequestWithUser, res: Response): Promise<void> {
  const count = await prisma.notification.count({
    where: {
      userId: req.user!.id,
      isRead: false,
    },
  });

  res.json({ count });
}

export async function markReadController(req: RequestWithUser, res: Response): Promise<void> {
  const payload = markReadSchema.parse(req.body);

  await prisma.notification.updateMany({
    where: {
      id: { in: payload.notification_ids },
      userId: req.user!.id,
    },
    data: {
      isRead: true,
    },
  });

  res.status(204).send();
}

export async function markAllReadController(req: RequestWithUser, res: Response): Promise<void> {
  await prisma.notification.updateMany({
    where: {
      userId: req.user!.id,
      isRead: false,
    },
    data: {
      isRead: true,
    },
  });

  res.status(204).send();
}

export async function getPreferencesController(req: RequestWithUser, res: Response): Promise<void> {
  const prefs = await getOrCreateNotificationPreferences(prisma, req.user!.id);
  res.json(prefs);
}

export async function updatePreferencesController(req: RequestWithUser, res: Response): Promise<void> {
  const payload = updatePreferencesSchema.parse(req.body);

  const prefs = await getOrCreateNotificationPreferences(prisma, req.user!.id);
  const updated = await prisma.notificationPreference.update({
    where: { id: prefs.id },
    data: {
      preferences:
        (payload.preferences ??
          ((prefs.preferences as Record<string, unknown> | null) ?? {})) as Prisma.InputJsonValue,
      soundEnabled: payload.sound_enabled ?? prefs.soundEnabled,
    },
  });

  res.json(updated);
}

export async function deleteNotificationController(req: RequestWithUser, res: Response): Promise<void> {
  const id = z.string().min(1).parse(req.params.id);
  await prisma.notification.delete({ where: { id } });
  res.status(204).send();
}

export async function messageRecipientsController(_req: RequestWithUser, res: Response): Promise<void> {
  const [admins, porters, demandeurs] = await Promise.all([
    prisma.user.findMany({
      where: { role: "administrateur", isActive: true },
      select: { id: true, username: true, firstName: true, lastName: true },
    }),
    prisma.porter.findMany({
      include: { user: { select: { id: true, username: true, firstName: true, lastName: true } } },
    }),
    prisma.user.findMany({
      where: { role: "demandeur", isActive: true },
      select: { id: true, username: true, firstName: true, lastName: true },
    }),
  ]);

  res.json({ admins, porters, demandeurs });
}

export async function sendMessageController(req: RequestWithUser, res: Response): Promise<void> {
  const payload = sendMessageSchema.parse(req.body);

  const recipientFilter = withRecipientFilter(payload);

  const users = (await prisma.user.findMany({
    where: {
      isActive: true,
      role: recipientFilter.role,
      id: recipientFilter.userId,
      porter: recipientFilter.porterId ? { id: recipientFilter.porterId } : undefined,
    },
    include: { porter: true },
  })) as User[];

  if (users.length === 0) {
    res.status(404).json({ message: "Aucun destinataire" });
    return;
  }

  await Promise.all(
    users.map((user) =>
      createNotification(prisma, {
        userId: user.id,
        notificationType: "user_message",
        title: `Message ${payload.channel ?? "general"}`,
        message: payload.message,
        priority: payload.channel === "urgent" ? "high" : "normal",
        data: {
          channel: payload.channel ?? "general",
          sender: req.user,
          target_type: payload.target_type,
        },
      }),
    ),
  );

  res.status(201).json({ sent: users.length });
}