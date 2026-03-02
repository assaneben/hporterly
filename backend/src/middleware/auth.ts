import type { NextFunction, Response } from "express";
import jwt from "jsonwebtoken";

import { env } from "../config/env";
import type { AuthUser, RequestWithUser } from "../types";

export function authMiddleware(req: RequestWithUser, res: Response, next: NextFunction): void {
  const authHeader = req.headers.authorization;

  if (!authHeader || !authHeader.startsWith("Bearer ")) {
    res.status(401).json({ message: "Authentification requise" });
    return;
  }

  const token = authHeader.slice("Bearer ".length);

  try {
    const payload = jwt.verify(token, env.JWT_SECRET) as AuthUser;
    req.user = payload;
    next();
  } catch {
    res.status(401).json({ message: "Token invalide" });
  }
}