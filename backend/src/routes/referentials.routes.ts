import { Router } from "express";

import {
  createEquipmentController,
  createServiceController,
  createSpecimenController,
  createTransportModeController,
  deleteEquipmentController,
  deleteServiceController,
  deleteSpecimenController,
  deleteTransportModeController,
  equipmentSchema,
  listActiveEquipmentController,
  listActiveServicesController,
  listActiveTransportModesController,
  listEquipmentController,
  listServicesController,
  listSpecimensController,
  listTransportModesController,
  serviceSchema,
  specimenSchema,
  transportModeSchema,
  updateEquipmentController,
  updateServiceController,
  updateSpecimenController,
  updateTransportModeController,
} from "../controllers/referentials.controller";
import { authMiddleware } from "../middleware/auth";
import { requireRole } from "../middleware/rbac";
import { validateBody } from "../middleware/validate";

export const referentialsRouter = Router();

referentialsRouter.use(authMiddleware);

referentialsRouter.get("/services", requireRole("administrateur", "demandeur", "brancardier", "regulateur"), (req, res, next) => {
  listServicesController(req, res).catch(next);
});
referentialsRouter.get(
  "/services/active",
  requireRole("administrateur", "demandeur", "brancardier", "regulateur"),
  (req, res, next) => {
    listActiveServicesController(req, res).catch(next);
  },
);
referentialsRouter.post("/services", requireRole("administrateur"), validateBody(serviceSchema), (req, res, next) => {
  createServiceController(req, res).catch(next);
});
referentialsRouter.patch(
  "/services/:id",
  requireRole("administrateur"),
  validateBody(serviceSchema.partial()),
  (req, res, next) => {
    updateServiceController(req, res).catch(next);
  },
);
referentialsRouter.delete("/services/:id", requireRole("administrateur"), (req, res, next) => {
  deleteServiceController(req, res).catch(next);
});

referentialsRouter.get("/equipment", requireRole("administrateur", "demandeur", "brancardier", "regulateur"), (req, res, next) => {
  listEquipmentController(req, res).catch(next);
});
referentialsRouter.get(
  "/equipment/active",
  requireRole("administrateur", "demandeur", "brancardier", "regulateur"),
  (req, res, next) => {
    listActiveEquipmentController(req, res).catch(next);
  },
);
referentialsRouter.post(
  "/equipment",
  requireRole("administrateur"),
  validateBody(equipmentSchema),
  (req, res, next) => {
    createEquipmentController(req, res).catch(next);
  },
);
referentialsRouter.patch(
  "/equipment/:id",
  requireRole("administrateur"),
  validateBody(equipmentSchema.partial()),
  (req, res, next) => {
    updateEquipmentController(req, res).catch(next);
  },
);
referentialsRouter.delete("/equipment/:id", requireRole("administrateur"), (req, res, next) => {
  deleteEquipmentController(req, res).catch(next);
});

referentialsRouter.get(
  "/transport-modes",
  requireRole("administrateur", "demandeur", "brancardier", "regulateur"),
  (req, res, next) => {
    listTransportModesController(req, res).catch(next);
  },
);
referentialsRouter.get(
  "/transport-modes/active",
  requireRole("administrateur", "demandeur", "brancardier", "regulateur"),
  (req, res, next) => {
    listActiveTransportModesController(req, res).catch(next);
  },
);
referentialsRouter.post(
  "/transport-modes",
  requireRole("administrateur"),
  validateBody(transportModeSchema),
  (req, res, next) => {
    createTransportModeController(req, res).catch(next);
  },
);
referentialsRouter.patch(
  "/transport-modes/:id",
  requireRole("administrateur"),
  validateBody(transportModeSchema.partial()),
  (req, res, next) => {
    updateTransportModeController(req, res).catch(next);
  },
);
referentialsRouter.delete("/transport-modes/:id", requireRole("administrateur"), (req, res, next) => {
  deleteTransportModeController(req, res).catch(next);
});

referentialsRouter.get(
  "/specimens",
  requireRole("administrateur", "demandeur", "brancardier", "regulateur"),
  (req, res, next) => {
    listSpecimensController(req, res).catch(next);
  },
);
referentialsRouter.post("/specimens", requireRole("administrateur"), validateBody(specimenSchema), (req, res, next) => {
  createSpecimenController(req, res).catch(next);
});
referentialsRouter.patch(
  "/specimens/:id",
  requireRole("administrateur"),
  validateBody(specimenSchema.partial()),
  (req, res, next) => {
    updateSpecimenController(req, res).catch(next);
  },
);
referentialsRouter.delete("/specimens/:id", requireRole("administrateur"), (req, res, next) => {
  deleteSpecimenController(req, res).catch(next);
});
