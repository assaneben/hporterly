import { Router } from "express";

import { patientsQuerySchema, searchPatientsController } from "../controllers/patients.controller";
import { authMiddleware } from "../middleware/auth";
import { requireRole } from "../middleware/rbac";
import { validateQuery } from "../middleware/validate";

export const patientsRouter = Router();

patientsRouter.use(authMiddleware, requireRole("administrateur", "demandeur", "brancardier", "regulateur"));

patientsRouter.get("/", validateQuery(patientsQuerySchema), (req, res, next) => {
  searchPatientsController(req, res).catch(next);
});