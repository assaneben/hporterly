import type { PrismaClient } from "@prisma/client";

export async function createNotification(
  prisma: PrismaClient,
  params: {
    userId: string;
    notificationType: string;
    title: string;
    message: string;
    priority?: string;
    data?: unknown;
    relatedTicketId?: string;
  },
) {
  return prisma.notification.create({
    data: {
      userId: params.userId,
      notificationType: params.notificationType,
      title: params.title,
      message: params.message,
      priority: params.priority ?? "normal",
      data: params.data as object | undefined,
      relatedTicketId: params.relatedTicketId,
    },
  });
}

export const DEFAULT_NOTIFICATION_PREFERENCES = {
  ticket_created: { enabled: true, sound: true },
  ticket_assigned: { enabled: true, sound: true },
  ticket_updated: { enabled: true, sound: false },
  ticket_completed: { enabled: true, sound: false },
  ticket_canceled: { enabled: true, sound: true },
  urgent_ticket: { enabled: true, sound: true },
  help_request_received: { enabled: true, sound: true },
  user_message: { enabled: true, sound: true },
  co_partner_added: { enabled: true, sound: true },
  co_partner_removed: { enabled: true, sound: true },
  mission_suspended: { enabled: true, sound: true },
  mission_resumed: { enabled: true, sound: false },
  rdv_activated: { enabled: true, sound: true },
  rdv_overdue: { enabled: true, sound: true },
};

export async function getOrCreateNotificationPreferences(prisma: PrismaClient, userId: string) {
  const existing = await prisma.notificationPreference.findUnique({ where: { userId } });

  if (existing) {
    return existing;
  }

  return prisma.notificationPreference.create({
    data: {
      userId,
      preferences: DEFAULT_NOTIFICATION_PREFERENCES,
      soundEnabled: true,
    },
  });
}