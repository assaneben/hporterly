import { Router } from "express";

import {
  deleteNotificationController,
  getPreferencesController,
  listNotificationsController,
  markAllReadController,
  markReadController,
  markReadSchema,
  messageRecipientsController,
  notificationQuerySchema,
  sendMessageController,
  sendMessageSchema,
  unreadCountController,
  updatePreferencesController,
  updatePreferencesSchema,
} from "../controllers/notifications.controller";
import { authMiddleware } from "../middleware/auth";
import { requireRole } from "../middleware/rbac";
import { validateBody, validateQuery } from "../middleware/validate";

export const notificationsRouter = Router();

notificationsRouter.use(authMiddleware);

notificationsRouter.get(
  "/",
  requireRole("administrateur", "demandeur", "brancardier", "regulateur"),
  validateQuery(notificationQuerySchema),
  (req, res, next) => {
    listNotificationsController(req, res).catch(next);
  },
);

notificationsRouter.get("/unread-count", requireRole("administrateur", "demandeur", "brancardier", "regulateur"), (req, res, next) => {
  unreadCountController(req, res).catch(next);
});

notificationsRouter.post(
  "/mark-read",
  requireRole("administrateur", "demandeur", "brancardier", "regulateur"),
  validateBody(markReadSchema),
  (req, res, next) => {
    markReadController(req, res).catch(next);
  },
);

notificationsRouter.post("/mark-all-read", requireRole("administrateur", "demandeur", "brancardier", "regulateur"), (req, res, next) => {
  markAllReadController(req, res).catch(next);
});

notificationsRouter.get(
  "/preferences",
  requireRole("administrateur", "demandeur", "brancardier", "regulateur"),
  (req, res, next) => {
    getPreferencesController(req, res).catch(next);
  },
);

notificationsRouter.patch(
  "/preferences",
  requireRole("administrateur", "demandeur", "brancardier", "regulateur"),
  validateBody(updatePreferencesSchema),
  (req, res, next) => {
    updatePreferencesController(req, res).catch(next);
  },
);

notificationsRouter.delete(
  "/:id",
  requireRole("administrateur", "demandeur", "brancardier", "regulateur"),
  (req, res, next) => {
    deleteNotificationController(req, res).catch(next);
  },
);

notificationsRouter.get("/message-recipients", requireRole("administrateur", "demandeur", "brancardier"), (req, res, next) => {
  messageRecipientsController(req, res).catch(next);
});

notificationsRouter.post(
  "/send-message",
  requireRole("administrateur", "demandeur", "brancardier"),
  validateBody(sendMessageSchema),
  (req, res, next) => {
    sendMessageController(req, res).catch(next);
  },
);