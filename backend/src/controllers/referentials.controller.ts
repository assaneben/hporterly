import type { Response } from "express";
import { z } from "zod";

import { prisma } from "../config/prisma";
import type { RequestWithUser } from "../types";

const idSchema = z.object({ id: z.string().min(1) });

export const serviceSchema = z.object({
  name: z.string().min(1),
  building: z.string().optional(),
  floor: z.string().optional(),
  site_id: z.string().min(1),
  site_name: z.string().min(1),
  building_id: z.string().min(1),
  building_name: z.string().min(1),
  level_id: z.string().min(1),
  level_name: z.string().min(1),
  zone_id: z.string().min(1),
  zone_name: z.string().min(1),
  subzone_id: z.string().optional(),
  subzone_name: z.string().optional(),
  is_active: z.boolean().optional(),
});

export const equipmentSchema = z.object({
  label: z.string().min(1),
  sizes: z.array(z.string()).default([]),
  required_fields: z.record(z.any()).default({}),
  is_active: z.boolean().optional(),
});

export const transportModeSchema = z.object({
  label: z.string().min(1),
  sort_order: z.number().int().optional(),
  is_active: z.boolean().optional(),
});

export const specimenSchema = z.object({
  label: z.string().min(1),
  is_active: z.boolean().optional(),
});

export async function listServicesController(_req: RequestWithUser, res: Response): Promise<void> {
  res.json(await prisma.referentialService.findMany({ orderBy: { name: "asc" } }));
}

export async function listActiveServicesController(_req: RequestWithUser, res: Response): Promise<void> {
  res.json(
    await prisma.referentialService.findMany({
      where: { isActive: true },
      orderBy: { name: "asc" },
    }),
  );
}

export async function createServiceController(req: RequestWithUser, res: Response): Promise<void> {
  const payload = serviceSchema.parse(req.body);
  const item = await prisma.referentialService.create({
    data: {
      name: payload.name,
      building: payload.building,
      floor: payload.floor,
      siteId: payload.site_id,
      siteName: payload.site_name,
      buildingId: payload.building_id,
      buildingName: payload.building_name,
      levelId: payload.level_id,
      levelName: payload.level_name,
      zoneId: payload.zone_id,
      zoneName: payload.zone_name,
      subzoneId: payload.subzone_id,
      subzoneName: payload.subzone_name,
      isActive: payload.is_active ?? true,
    },
  });
  res.status(201).json(item);
}

export async function updateServiceController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);
  const payload = serviceSchema.partial().parse(req.body);

  const item = await prisma.referentialService.update({
    where: { id },
    data: {
      name: payload.name,
      building: payload.building,
      floor: payload.floor,
      siteId: payload.site_id,
      siteName: payload.site_name,
      buildingId: payload.building_id,
      buildingName: payload.building_name,
      levelId: payload.level_id,
      levelName: payload.level_name,
      zoneId: payload.zone_id,
      zoneName: payload.zone_name,
      subzoneId: payload.subzone_id,
      subzoneName: payload.subzone_name,
      isActive: payload.is_active,
    },
  });

  res.json(item);
}

export async function deleteServiceController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);
  await prisma.referentialService.delete({ where: { id } });
  res.status(204).send();
}

export async function listEquipmentController(_req: RequestWithUser, res: Response): Promise<void> {
  res.json(await prisma.referentialEquipment.findMany({ orderBy: { label: "asc" } }));
}

export async function listActiveEquipmentController(_req: RequestWithUser, res: Response): Promise<void> {
  res.json(
    await prisma.referentialEquipment.findMany({
      where: { isActive: true },
      orderBy: { label: "asc" },
    }),
  );
}

export async function createEquipmentController(req: RequestWithUser, res: Response): Promise<void> {
  const payload = equipmentSchema.parse(req.body);
  const item = await prisma.referentialEquipment.create({
    data: {
      label: payload.label,
      sizes: payload.sizes,
      requiredFields: payload.required_fields,
      isActive: payload.is_active ?? true,
    },
  });
  res.status(201).json(item);
}

export async function updateEquipmentController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);
  const payload = equipmentSchema.partial().parse(req.body);

  const item = await prisma.referentialEquipment.update({
    where: { id },
    data: {
      label: payload.label,
      sizes: payload.sizes,
      requiredFields: payload.required_fields,
      isActive: payload.is_active,
    },
  });

  res.json(item);
}

export async function deleteEquipmentController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);
  await prisma.referentialEquipment.delete({ where: { id } });
  res.status(204).send();
}

export async function listTransportModesController(_req: RequestWithUser, res: Response): Promise<void> {
  res.json(await prisma.referentialTransportMode.findMany({ orderBy: { sortOrder: "asc" } }));
}

export async function listActiveTransportModesController(_req: RequestWithUser, res: Response): Promise<void> {
  res.json(
    await prisma.referentialTransportMode.findMany({
      where: { isActive: true },
      orderBy: { sortOrder: "asc" },
    }),
  );
}

export async function createTransportModeController(req: RequestWithUser, res: Response): Promise<void> {
  const payload = transportModeSchema.parse(req.body);
  const item = await prisma.referentialTransportMode.create({
    data: {
      label: payload.label,
      isActive: payload.is_active ?? true,
      sortOrder: payload.sort_order ?? 0,
    },
  });
  res.status(201).json(item);
}

export async function updateTransportModeController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);
  const payload = transportModeSchema.partial().parse(req.body);
  const item = await prisma.referentialTransportMode.update({
    where: { id },
    data: {
      label: payload.label,
      isActive: payload.is_active,
      sortOrder: payload.sort_order,
    },
  });
  res.json(item);
}

export async function deleteTransportModeController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);
  await prisma.referentialTransportMode.delete({ where: { id } });
  res.status(204).send();
}

export async function listSpecimensController(_req: RequestWithUser, res: Response): Promise<void> {
  res.json(await prisma.referentialSpecimen.findMany({ orderBy: { label: "asc" } }));
}

export async function createSpecimenController(req: RequestWithUser, res: Response): Promise<void> {
  const payload = specimenSchema.parse(req.body);
  const item = await prisma.referentialSpecimen.create({
    data: {
      label: payload.label,
      isActive: payload.is_active ?? true,
    },
  });
  res.status(201).json(item);
}

export async function updateSpecimenController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);
  const payload = specimenSchema.partial().parse(req.body);

  const item = await prisma.referentialSpecimen.update({
    where: { id },
    data: {
      label: payload.label,
      isActive: payload.is_active,
    },
  });

  res.json(item);
}

export async function deleteSpecimenController(req: RequestWithUser, res: Response): Promise<void> {
  const { id } = idSchema.parse(req.params);
  await prisma.referentialSpecimen.delete({ where: { id } });
  res.status(204).send();
}