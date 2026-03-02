import type { NextFunction, Request, Response } from "express";
import type { ZodSchema } from "zod";

export function validateBody<T>(schema: ZodSchema<T>) {
  return (req: Request, res: Response, next: NextFunction): void => {
    const parsed = schema.safeParse(req.body);

    if (!parsed.success) {
      res.status(422).json({ message: "Validation invalide", errors: parsed.error.flatten() });
      return;
    }

    req.body = parsed.data;
    next();
  };
}

export function validateQuery<T>(schema: ZodSchema<T>) {
  return (req: Request, res: Response, next: NextFunction): void => {
    const parsed = schema.safeParse(req.query);

    if (!parsed.success) {
      res.status(422).json({ message: "Query invalide", errors: parsed.error.flatten() });
      return;
    }

    req.query = parsed.data as Request["query"];
    next();
  };
}

export function validateParams<T>(schema: ZodSchema<T>) {
  return (req: Request, res: Response, next: NextFunction): void => {
    const parsed = schema.safeParse(req.params);

    if (!parsed.success) {
      res.status(422).json({ message: "Parametres invalides", errors: parsed.error.flatten() });
      return;
    }

    req.params = parsed.data as Request["params"];
    next();
  };
}