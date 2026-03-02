import type { Response } from "express";
import { z } from "zod";

import { prisma } from "../config/prisma";
import type { RequestWithUser } from "../types";

export const patientsQuerySchema = z.object({
  search: z.string().optional(),
});

export async function searchPatientsController(req: RequestWithUser, res: Response): Promise<void> {
  const query = patientsQuerySchema.parse(req.query);

  const patients = await prisma.patient.findMany({
    where: query.search
      ? {
          OR: [
            { firstName: { contains: query.search, mode: "insensitive" } },
            { lastName: { contains: query.search, mode: "insensitive" } },
            { id: { contains: query.search, mode: "insensitive" } },
          ],
        }
      : undefined,
    orderBy: [{ lastName: "asc" }, { firstName: "asc" }],
    take: 50,
  });

  res.json(patients);
}