import { Router } from "express";

import { loginController, loginSchema } from "../controllers/auth.controller";
import { validateBody } from "../middleware/validate";

export const authRouter = Router();

authRouter.post("/login", validateBody(loginSchema), (req, res, next) => {
  loginController(req, res).catch(next);
});