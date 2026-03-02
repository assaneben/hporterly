import { Router } from "express";

import {
  getPriorityDefaultsController,
  getPriorityRulesController,
  getPriorityRuntimeController,
  replaceConfigSchema,
  replacePriorityRulesController,
  restorePriorityRulesController,
} from "../controllers/priorityRules.controller";
import { authMiddleware } from "../middleware/auth";
import { requireRole } from "../middleware/rbac";
import { validateBody } from "../middleware/validate";

export const priorityRulesRouter = Router();

priorityRulesRouter.use(authMiddleware);

priorityRulesRouter.get("/", requireRole("administrateur"), (req, res, next) => {
  getPriorityRulesController(req, res).catch(next);
});

priorityRulesRouter.get("/runtime", requireRole("administrateur", "demandeur", "brancardier", "regulateur"), (req, res, next) => {
  getPriorityRuntimeController(req, res).catch(next);
});

priorityRulesRouter.get("/defaults", requireRole("administrateur"), getPriorityDefaultsController);

priorityRulesRouter.put(
  "/",
  requireRole("administrateur"),
  validateBody(replaceConfigSchema),
  (req, res, next) => {
    replacePriorityRulesController(req, res).catch(next);
  },
);

priorityRulesRouter.post("/restore-defaults", requireRole("administrateur"), (req, res, next) => {
  restorePriorityRulesController(req, res).catch(next);
});
