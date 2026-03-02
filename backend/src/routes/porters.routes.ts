import { Router } from "express";

import {
  createPorterController,
  createPorterSchema,
  listPortersController,
  porterSkillsSchema,
  porterStatusSchema,
  updatePorterSkillsController,
  updatePorterStatusController,
} from "../controllers/porters.controller";
import { authMiddleware } from "../middleware/auth";
import { requireRole } from "../middleware/rbac";
import { validateBody } from "../middleware/validate";

export const portersRouter = Router();

portersRouter.use(authMiddleware);

portersRouter.get("/", requireRole("administrateur"), (req, res, next) => {
  listPortersController(req, res).catch(next);
});

portersRouter.post(
  "/",
  requireRole("administrateur"),
  validateBody(createPorterSchema),
  (req, res, next) => {
    createPorterController(req, res).catch(next);
  },
);

portersRouter.patch(
  "/:id/status",
  requireRole("administrateur", "brancardier"),
  validateBody(porterStatusSchema),
  (req, res, next) => {
    updatePorterStatusController(req, res).catch(next);
  },
);

portersRouter.patch(
  "/:id/skills",
  requireRole("administrateur"),
  validateBody(porterSkillsSchema),
  (req, res, next) => {
    updatePorterSkillsController(req, res).catch(next);
  },
);