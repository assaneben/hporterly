import cors from "cors";
import express from "express";
import helmet from "helmet";
import morgan from "morgan";

import { env } from "./config/env";
import { errorHandler } from "./middleware/errorHandler";
import { authRouter } from "./routes/auth.routes";
import { healthRouter } from "./routes/health.routes";
import { notificationsRouter } from "./routes/notifications.routes";
import { patientsRouter } from "./routes/patients.routes";
import { portersRouter } from "./routes/porters.routes";
import { priorityRulesRouter } from "./routes/priorityRules.routes";
import { referentialsRouter } from "./routes/referentials.routes";
import { ticketsRouter } from "./routes/tickets.routes";
import { usersRouter } from "./routes/users.routes";

export function createApp() {
  const app = express();

  app.use(helmet());
  app.use(
    cors({
      origin: env.CORS_ORIGIN,
      credentials: true,
    }),
  );
  app.use(express.json({ limit: "1mb" }));
  app.use(morgan("dev"));

  app.use("/api/health", healthRouter);
  app.use("/api/auth", authRouter);
  app.use("/api/tickets", ticketsRouter);
  app.use("/api/porters", portersRouter);
  app.use("/api/users", usersRouter);
  app.use("/api/referentials", referentialsRouter);
  app.use("/api/priority-rules", priorityRulesRouter);
  app.use("/api/notifications", notificationsRouter);
  app.use("/api/patients", patientsRouter);

  app.use(errorHandler);

  return app;
}