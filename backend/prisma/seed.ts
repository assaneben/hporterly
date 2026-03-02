import bcrypt from "bcryptjs";
import { PrismaClient } from "@prisma/client";

import { DEFAULT_NOTIFICATION_PREFERENCES } from "../src/services/notification.service";
import { getDefaultPriorityConfig } from "../src/services/priority.service";

const prisma = new PrismaClient();

const DEMO_USERS = [
  {
    username: "admin",
    password: "password123",
    role: "administrateur",
    firstName: "Thomas",
    lastName: "Dubois",
  },
  {
    username: "marie.durand",
    password: "password123",
    role: "demandeur",
    firstName: "Marie",
    lastName: "Durand",
  },
  {
    username: "jean.martin",
    password: "password123",
    role: "brancardier",
    firstName: "Jean",
    lastName: "Martin",
    skills: ["BRANCARD", "FAUTEUIL", "URG", "O2"],
  },
  {
    username: "regulateur",
    password: "password123",
    role: "regulateur",
    firstName: "Pierre",
    lastName: "Lambert",
  },
] as const;

const ADDITIONAL_PORTERS = [
  {
    username: "lea.bernard",
    firstName: "Lea",
    lastName: "Bernard",
    skills: ["BRANCARD", "LIT", "O2"],
  },
  {
    username: "nassim.belkacem",
    firstName: "Nassim",
    lastName: "Belkacem",
    skills: ["BRANCARD", "FAUTEUIL", "URG"],
  },
  {
    username: "sarah.nguyen",
    firstName: "Sarah",
    lastName: "Nguyen",
    skills: ["BRANCARD", "LIT", "FAUTEUIL", "O2"],
  },
  {
    username: "alex.morel",
    firstName: "Alex",
    lastName: "Morel",
    skills: ["BRANCARD", "URG", "LIT"],
  },
] as const;

const SERVICE_NAMES = [
  "Urgences",
  "Imagerie",
  "Bloc operatoire",
  "Cardiologie",
  "Pneumologie",
  "Neurologie",
  "Nephrologie",
  "Oncologie",
  "Reanimation",
  "USC",
  "SSPI",
  "Radiologie",
  "Consultation externe",
  "HDJ",
  "Maternite",
  "Pediatrie",
  "Geriatrie",
  "Dialyse",
  "Laboratoire",
  "Sortie patients",
] as const;

const EQUIPMENT_TYPES = [
  { label: "Materiel generique", sizes: ["S", "M", "L", "XL"] },
  { label: "Pompe a perfusion", sizes: ["M"] },
  { label: "Moniteur portable", sizes: ["M", "L"] },
  { label: "Lit medicalise", sizes: ["XL"] },
  { label: "Fauteuil roulant", sizes: ["M", "L"] },
] as const;

const SPECIMENS = ["Prise de sang", "Biopsie", "Bacteriologie"] as const;

const TRANSPORT_MODES = [
  "A pied",
  "A pied avec assistance",
  "En fauteuil roulant",
  "En brancard",
  "En lit",
] as const;

async function resetDatabase() {
  await prisma.auditLog.deleteMany();
  await prisma.helpRequest.deleteMany();
  await prisma.notification.deleteMany();
  await prisma.notificationPreference.deleteMany();
  await prisma.ticketAssignment.deleteMany();
  await prisma.ticket.deleteMany();
  await prisma.priorityRulesConfig.deleteMany();
  await prisma.referentialSpecimen.deleteMany();
  await prisma.referentialTransportMode.deleteMany();
  await prisma.referentialEquipment.deleteMany();
  await prisma.referentialService.deleteMany();
  await prisma.patient.deleteMany();
  await prisma.porter.deleteMany();
  await prisma.user.deleteMany();
}

async function seedUsersAndPorters() {
  const createdUsers: Record<string, string> = {};
  const createdPorters: string[] = [];

  for (const demo of DEMO_USERS) {
    const passwordHash = await bcrypt.hash(demo.password, 10);
    const user = await prisma.user.create({
      data: {
        username: demo.username,
        passwordHash,
        role: demo.role,
        firstName: demo.firstName,
        lastName: demo.lastName,
        email: `${demo.username}@hporterly.local`,
        service: demo.role === "demandeur" ? "Urgences" : "Regulation",
      },
    });

    createdUsers[demo.username] = user.id;

    if (demo.role === "brancardier") {
      const porter = await prisma.porter.create({
        data: {
          userId: user.id,
          status: "available",
          skills: [...(demo.skills ?? [])],
          rating: 4.2,
        },
      });
      createdPorters.push(porter.id);
    }

    await prisma.notificationPreference.create({
      data: {
        userId: user.id,
        preferences: DEFAULT_NOTIFICATION_PREFERENCES,
        soundEnabled: true,
      },
    });
  }

  for (const porterSeed of ADDITIONAL_PORTERS) {
    const passwordHash = await bcrypt.hash("password123", 10);
    const user = await prisma.user.create({
      data: {
        username: porterSeed.username,
        passwordHash,
        role: "brancardier",
        firstName: porterSeed.firstName,
        lastName: porterSeed.lastName,
        email: `${porterSeed.username}@hporterly.local`,
        service: "Brancardage",
      },
    });

    const porter = await prisma.porter.create({
      data: {
        userId: user.id,
        status: "available",
        skills: [...porterSeed.skills],
        completedMissionsToday: Math.floor(Math.random() * 4),
        totalMissions: 100 + Math.floor(Math.random() * 300),
        rating: 3.4 + Math.random() * 1.5,
      },
    });

    createdUsers[porterSeed.username] = user.id;
    createdPorters.push(porter.id);

    await prisma.notificationPreference.create({
      data: {
        userId: user.id,
        preferences: DEFAULT_NOTIFICATION_PREFERENCES,
        soundEnabled: true,
      },
    });
  }

  return { createdUsers, createdPorters };
}

async function seedReferentials() {
  for (const [index, service] of SERVICE_NAMES.entries()) {
    await prisma.referentialService.create({
      data: {
        name: service,
        building: `Batiment ${index % 4 + 1}`,
        floor: `${(index % 5) + 1}`,
        siteId: "SITE-001",
        siteName: "Hopital Central",
        buildingId: `BAT-${index % 4 + 1}`,
        buildingName: `Batiment ${index % 4 + 1}`,
        levelId: `LVL-${(index % 5) + 1}`,
        levelName: `Niveau ${(index % 5) + 1}`,
        zoneId: `ZONE-${index % 6 + 1}`,
        zoneName: `Zone ${index % 6 + 1}`,
        subzoneId: `SUB-${index % 3 + 1}`,
        subzoneName: `Sous-zone ${index % 3 + 1}`,
      },
    });
  }

  for (const equipment of EQUIPMENT_TYPES) {
    await prisma.referentialEquipment.create({
      data: {
        label: equipment.label,
        sizes: [...equipment.sizes],
        requiredFields: {
          recipient: true,
          priority: true,
          origin: true,
          destination: true,
          note_free: false,
          scheduled: false,
          code: equipment.label.toUpperCase().replace(/\s+/g, "-"),
          business_rules: {
            need_2p: equipment.label === "Lit medicalise",
            workflow_return_required: false,
            photo_proof_required: false,
          },
          custom_fields: [],
        },
      },
    });
  }

  for (const specimen of SPECIMENS) {
    await prisma.referentialSpecimen.create({
      data: {
        label: specimen,
      },
    });
  }

  for (const [index, mode] of TRANSPORT_MODES.entries()) {
    await prisma.referentialTransportMode.create({
      data: {
        label: mode,
        sortOrder: index,
      },
    });
  }

  await prisma.priorityRulesConfig.create({
    data: {
      rulesJson: getDefaultPriorityConfig(),
      isActive: true,
    },
  });
}

async function seedPatients() {
  const now = Date.now();

  for (let index = 1; index <= 20; index += 1) {
    await prisma.patient.create({
      data: {
        id: `IPP-${1000 + index}`,
        firstName: `Patient${index}`,
        lastName: `Demo${index}`,
        age: 20 + (index % 70),
        gender: index % 2 === 0 ? "F" : "M",
        service: SERVICE_NAMES[index % SERVICE_NAMES.length],
        room: `CH-${index}`,
        building: `Batiment ${index % 4 + 1}`,
        floor: `${(index % 5) + 1}`,
        dateOfBirth: new Date(now - (20 + index) * 365 * 24 * 3600 * 1000),
        sex: index % 2 === 0 ? "F" : "M",
      },
    });
  }
}

async function seedTickets(userIds: Record<string, string>, porterIds: string[]) {
  const requesterId = userIds["marie.durand"];
  const adminId = userIds.admin;

  const statuses = [
    "pending",
    "assigned",
    "in_progress",
    "arrived",
    "suspended",
    "completed",
    "canceled",
    "pending",
    "assigned",
    "in_progress",
  ] as const;

  for (let index = 0; index < 10; index += 1) {
    const status = statuses[index];
    const priority = ((index % 4) + 1) as 1 | 2 | 3 | 4;
    const requester = index % 2 === 0 ? requesterId : adminId;
    const assignedPorter = status === "assigned" || status === "in_progress" || status === "arrived" ? porterIds[index % porterIds.length] : null;

    const ticket = await prisma.ticket.create({
      data: {
        patientId: `IPP-${1000 + index + 1}`,
        patientName: `Patient${index + 1} Demo${index + 1}`,
        patientFirstName: `Patient${index + 1}`,
        patientLastName: `Demo${index + 1}`,
        patientIpp: `IPP-${1000 + index + 1}`,
        origin: SERVICE_NAMES[index % SERVICE_NAMES.length],
        destination: SERVICE_NAMES[(index + 3) % SERVICE_NAMES.length],
        priority,
        mode: index % 4 === 0 ? "Lit" : "Brancard",
        status,
        porterId: assignedPorter,
        requesterId: requester,
        needsO2: index % 3 === 0,
        needsPerfusion: index % 4 === 0,
        isolation: index % 5 === 0,
        patientAgitated: index % 6 === 0,
        patientMonitoring: index % 2 === 0,
        needsTwoPorters: index % 4 === 0,
        notes: `Ticket de demonstration #${index + 1}`,
        transportType: index % 3 === 0 ? "EQUIPMENT" : index % 3 === 1 ? "PATIENT" : "SPECIMEN",
        transportSubtype: index % 3 === 0 ? "MAT-GENERIQUE" : "TP-BRANC",
        laboratoryName: index % 3 === 2 ? "Laboratoire Central" : null,
        specimenTypes: index % 3 === 2 ? ["Prise de sang"] : [],
        isVisibleToPorters: true,
        completedAt: status === "completed" ? new Date() : null,
      },
    });

    if (assignedPorter) {
      await prisma.ticketAssignment.create({
        data: {
          ticketId: ticket.id,
          porterId: assignedPorter,
          role: "primary",
          isActive: true,
        },
      });
    }
  }
}

async function main() {
  await resetDatabase();
  const { createdUsers, createdPorters } = await seedUsersAndPorters();
  await seedReferentials();
  await seedPatients();
  await seedTickets(createdUsers, createdPorters);

  console.log("Seed terminee avec succes");
}

main()
  .catch((error) => {
    console.error(error);
    process.exit(1);
  })
  .finally(async () => {
    await prisma.$disconnect();
  });