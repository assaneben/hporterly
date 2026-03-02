import type { Response } from "express";
import bcrypt from "bcryptjs";
import jwt from "jsonwebtoken";
import { z } from "zod";

import { env } from "../config/env";
import { prisma } from "../config/prisma";
import type { RequestWithUser } from "../types";

export const loginSchema = z.object({
  username: z.string().min(1),
  password: z.string().min(1),
});

export async function loginController(req: RequestWithUser, res: Response): Promise<void> {
  const { username, password } = req.body as z.infer<typeof loginSchema>;

  const user = await prisma.user.findUnique({
    where: { username },
    include: { porter: true },
  });

  if (!user || !user.isActive) {
    res.status(401).json({ message: "Identifiants invalides" });
    return;
  }

  const passwordOk = await bcrypt.compare(password, user.passwordHash);

  if (!passwordOk) {
    res.status(401).json({ message: "Identifiants invalides" });
    return;
  }

  const tokenPayload = {
    id: user.id,
    username: user.username,
    role: user.role,
    porterId: user.porter?.id,
  };

  const token = jwt.sign(tokenPayload, env.JWT_SECRET, { expiresIn: "12h" });

  res.json({
    token,
    user: {
      id: user.id,
      username: user.username,
      role: user.role,
      first_name: user.firstName,
      last_name: user.lastName,
      email: user.email,
      service: user.service,
      porter_id: user.porter?.id,
    },
  });
}