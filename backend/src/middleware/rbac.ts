import type { NextFunction, Response } from "express";

import type { RequestWithUser, Role } from "../types";

export function requireRole(...roles: Role[]) {
  return (req: RequestWithUser, res: Response, next: NextFunction): void => {
    if (!req.user) {
      res.status(401).json({ message: "Authentification requise" });
      return;
    }

    if (!roles.includes(req.user.role)) {
      res.status(403).json({ message: "Acces refuse" });
      return;
    }

    next();
  };
}