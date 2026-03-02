import { Router } from "express";

import {
  addCoPartnerController,
  assignTicketController,
  cancelSchema,
  cancelTicketController,
  createTicketController,
  createTicketSchema,
  hardDeleteSchema,
  hardDeleteTicketController,
  helpRespondSchema,
  helpSchema,
  listTicketsController,
  listTicketsQuerySchema,
  overrideTicketPriorityController,
  pauseTicketController,
  prioritySchema,
  reassignTicketController,
  removeCoPartnerController,
  requestHelpController,
  respondHelpController,
  statusUpdateSchema,
  ticketRecommendationsController,
  unassignTicketController,
  updateEquipmentStatusController,
  updateTicketNotesController,
  notesSchema,
  getTicketController,
  assignSchema,
  coPartnerCreateSchema,
  equipmentStatusSchema,
  updateTicketStatusController,
} from "../controllers/tickets.controller";
import { evaluateTicketPriorityController } from "../controllers/priorityRules.controller";
import { authMiddleware } from "../middleware/auth";
import { requireRole } from "../middleware/rbac";
import { validateBody, validateQuery } from "../middleware/validate";

export const ticketsRouter = Router();

ticketsRouter.use(authMiddleware);

ticketsRouter.get(
  "/",
  requireRole("administrateur", "regulateur", "brancardier", "demandeur"),
  validateQuery(listTicketsQuerySchema),
  (req, res, next) => {
    listTicketsController(req, res).catch(next);
  },
);

ticketsRouter.post(
  "/",
  requireRole("administrateur", "demandeur"),
  validateBody(createTicketSchema),
  (req, res, next) => {
    createTicketController(req, res).catch(next);
  },
);

ticketsRouter.get(
  "/:id",
  requireRole("administrateur", "regulateur", "brancardier", "demandeur"),
  (req, res, next) => {
    getTicketController(req, res).catch(next);
  },
);

ticketsRouter.patch(
  "/:id/status",
  requireRole("administrateur", "brancardier"),
  validateBody(statusUpdateSchema),
  (req, res, next) => {
    updateTicketStatusController(req, res).catch(next);
  },
);

ticketsRouter.patch(
  "/:id/notes",
  requireRole("administrateur", "brancardier"),
  validateBody(notesSchema),
  (req, res, next) => {
    updateTicketNotesController(req, res).catch(next);
  },
);

ticketsRouter.patch(
  "/:id/priority",
  requireRole("administrateur"),
  validateBody(prioritySchema),
  (req, res, next) => {
    overrideTicketPriorityController(req, res).catch(next);
  },
);

ticketsRouter.post(
  "/:id/assign",
  requireRole("administrateur", "brancardier"),
  validateBody(assignSchema),
  (req, res, next) => {
    assignTicketController(req, res).catch(next);
  },
);

ticketsRouter.post(
  "/:id/reassign",
  requireRole("administrateur"),
  validateBody(assignSchema.required()),
  (req, res, next) => {
    reassignTicketController(req, res).catch(next);
  },
);

ticketsRouter.post("/:id/unassign", requireRole("administrateur"), (req, res, next) => {
  unassignTicketController(req, res).catch(next);
});

ticketsRouter.post(
  "/:id/pause",
  requireRole("administrateur", "brancardier"),
  validateBody(cancelSchema.partial()),
  (req, res, next) => {
    pauseTicketController(req, res).catch(next);
  },
);

ticketsRouter.post(
  "/:id/cancel",
  requireRole("administrateur"),
  validateBody(cancelSchema),
  (req, res, next) => {
    cancelTicketController(req, res).catch(next);
  },
);

ticketsRouter.post(
  "/:id/hard-delete",
  requireRole("administrateur"),
  validateBody(hardDeleteSchema),
  (req, res, next) => {
    hardDeleteTicketController(req, res).catch(next);
  },
);

ticketsRouter.get(
  "/:id/recommendations",
  requireRole("administrateur"),
  (req, res, next) => {
    ticketRecommendationsController(req, res).catch(next);
  },
);

ticketsRouter.post(
  "/:id/co-partners",
  requireRole("administrateur"),
  validateBody(coPartnerCreateSchema),
  (req, res, next) => {
    addCoPartnerController(req, res).catch(next);
  },
);

ticketsRouter.delete(
  "/:id/co-partners/:pid",
  requireRole("administrateur"),
  (req, res, next) => {
    removeCoPartnerController(req, res).catch(next);
  },
);

ticketsRouter.post(
  "/:id/help",
  requireRole("brancardier"),
  validateBody(helpSchema),
  (req, res, next) => {
    requestHelpController(req, res).catch(next);
  },
);

ticketsRouter.post(
  "/:id/help/respond",
  requireRole("brancardier"),
  validateBody(helpRespondSchema),
  (req, res, next) => {
    respondHelpController(req, res).catch(next);
  },
);

ticketsRouter.post(
  "/:id/evaluate-priority",
  requireRole("administrateur", "regulateur"),
  (req, res, next) => {
    evaluateTicketPriorityController(req, res).catch(next);
  },
);

ticketsRouter.patch(
  "/:id/equipment-status",
  requireRole("administrateur", "brancardier"),
  validateBody(equipmentStatusSchema),
  (req, res, next) => {
    updateEquipmentStatusController(req, res).catch(next);
  },
);
