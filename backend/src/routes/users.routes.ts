import { Router } from "express";

import {
  createUserController,
  createUserSchema,
  deleteUserController,
  hardDeleteUserController,
  hardDeleteUserSchema,
  listUsersController,
  updateUserController,
  updateUserSchema,
} from "../controllers/users.controller";
import { authMiddleware } from "../middleware/auth";
import { requireRole } from "../middleware/rbac";
import { validateBody } from "../middleware/validate";

export const usersRouter = Router();

usersRouter.use(authMiddleware, requireRole("administrateur"));

usersRouter.get("/", (req, res, next) => {
  listUsersController(req, res).catch(next);
});

usersRouter.post("/", validateBody(createUserSchema), (req, res, next) => {
  createUserController(req, res).catch(next);
});

usersRouter.patch("/:id", validateBody(updateUserSchema), (req, res, next) => {
  updateUserController(req, res).catch(next);
});

usersRouter.delete("/:id", (req, res, next) => {
  deleteUserController(req, res).catch(next);
});

usersRouter.post("/:id/hard-delete", validateBody(hardDeleteUserSchema), (req, res, next) => {
  hardDeleteUserController(req, res).catch(next);
});