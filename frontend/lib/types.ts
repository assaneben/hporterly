export type Role = "demandeur" | "brancardier" | "regulateur" | "administrateur";

export type User = {
  id: string;
  username: string;
  role: Role;
  first_name: string;
  last_name: string;
  email?: string;
  service?: string;
  porter_id?: string;
};

export type Ticket = {
  id: string;
  patientName: string;
  patientFirstName?: string;
  patientLastName?: string;
  patientIpp?: string;
  origin: string;
  destination: string;
  priority: number;
  status: string;
  porterId?: string | null;
  requesterId: string;
  mode: string;
  createdAt: string;
  updatedAt: string;
  completedAt?: string | null;
  transportType: "PATIENT" | "EQUIPMENT" | "SPECIMEN";
  transportSubtype: string;
  needsO2?: boolean;
  needsPerfusion?: boolean;
  isolation?: boolean;
  patientMonitoring?: boolean;
  needsTwoPorters?: boolean;
  notes?: string;
  pending_sync?: boolean;
};

export type Porter = {
  id: string;
  userId: string;
  status: "available" | "busy" | "break" | "offline";
  skills: string[];
  currentLocation?: string | null;
  completedMissionsToday: number;
  totalMissions: number;
  rating: number;
  user?: {
    firstName: string;
    lastName: string;
    username: string;
  };
};

export type NotificationItem = {
  id: string;
  userId: string;
  notificationType: string;
  title: string;
  message: string;
  priority: string;
  isRead: boolean;
  createdAt: string;
};