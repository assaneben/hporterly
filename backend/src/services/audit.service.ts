import type { PrismaClient } from "@prisma/client";
import type { RequestWithUser } from "../types";

export async function logAudit(
  prisma: PrismaClient,
  params: {
    req: RequestWithUser;
    action: string;
    entityType: string;
    entityId: string;
    oldValue?: unknown;
    newValue?: unknown;
  },
): Promise<void> {
  if (!params.req.user?.id) {
    return;
  }

  await prisma.auditLog.create({
    data: {
      userId: params.req.user.id,
      action: params.action,
      entityType: params.entityType,
      entityId: params.entityId,
      oldValue: params.oldValue as object | undefined,
      newValue: params.newValue as object | undefined,
      ipAddress: params.req.ip,
      userAgent: params.req.headers["user-agent"],
    },
  });
}