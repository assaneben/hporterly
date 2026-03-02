"use client";

import { useEffect, useMemo, useState } from "react";
import {
  Area,
  AreaChart,
  Bar,
  BarChart,
  CartesianGrid,
  Cell,
  Pie,
  PieChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";

import { Card } from "@/components/ui/Card";
import type { Ticket } from "@/lib/types";

type Props = {
  tickets: Ticket[];
};

const priorityColors: Record<number, string> = {
  1: "#ef4444",
  2: "#f97316",
  3: "#3b82f6",
  4: "#10b981",
};

const statusLabels: Record<string, string> = {
  pending: "En attente",
  assigned: "Assigne",
  in_progress: "En cours",
  suspended: "Suspendu",
  arrived: "Arrive",
  completed: "Termine",
  canceled: "Annule",
};

export function ReportsPanel({ tickets }: Props) {
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  const data = useMemo(() => {
    const byPriority = [1, 2, 3, 4].map((level) => ({
      level: `N${level}`,
      count: tickets.filter((ticket) => ticket.priority === level).length,
      color: priorityColors[level],
    }));

    const byStatus = Object.keys(statusLabels).map((status) => ({
      name: statusLabels[status],
      value: tickets.filter((ticket) => ticket.status === status).length,
    }));

    const timeline = Array.from({ length: 24 }).map((_, hour) => ({
      hour: `${String(hour).padStart(2, "0")}h`,
      demandes: tickets.filter((ticket) => new Date(ticket.createdAt).getHours() === hour).length,
    }));

    const heatmap = ["Lun", "Mar", "Mer", "Jeu", "Ven", "Sam", "Dim"].map((day, dayIndex) => {
      const hours = Array.from({ length: 24 }).map((_, hour) => {
        const count = tickets.filter((ticket) => {
          const date = new Date(ticket.createdAt);
          const normalizedDay = date.getDay() === 0 ? 6 : date.getDay() - 1;
          return normalizedDay === dayIndex && date.getHours() === hour;
        }).length;

        return { hour, count };
      });

      return { day, hours };
    });

    const slaByPriority = [1, 2, 3, 4].map((level) => {
      const subset = tickets.filter((ticket) => ticket.priority === level);
      const done = subset.filter((ticket) => ["completed", "canceled"].includes(ticket.status)).length;
      const sla = subset.length ? Math.round((done / subset.length) * 100) : 100;

      return { level: `N${level}`, sla };
    });

    return {
      byPriority,
      byStatus,
      timeline,
      heatmap,
      slaByPriority,
    };
  }, [tickets]);

  const maxHeat = Math.max(
    1,
    ...data.heatmap.flatMap((row) => row.hours.map((item) => item.count)),
  );

  if (!mounted) {
    return (
      <section className="grid gap-4 xl:grid-cols-2">
        <Card variant="dashboard" className="h-[330px] animate-pulse bg-slate-100" />
        <Card variant="dashboard" className="h-[330px] animate-pulse bg-slate-100" />
      </section>
    );
  }

  return (
    <section className="space-y-4">
      <div className="grid gap-4 xl:grid-cols-2">
        <Card variant="dashboard" className="h-[330px] p-4">
          <h2 className="mb-2 font-title text-lg font-semibold text-slate-900">Timeline des demandes (24h)</h2>
          <ResponsiveContainer width="100%" height="88%">
            <AreaChart data={data.timeline}>
              <defs>
                <linearGradient id="timelineGradient" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="0%" stopColor="#3b82f6" stopOpacity={0.5} />
                  <stop offset="100%" stopColor="#3b82f6" stopOpacity={0.05} />
                </linearGradient>
              </defs>
              <CartesianGrid stroke="#e2e8f0" strokeDasharray="3 3" />
              <XAxis dataKey="hour" tick={{ fontSize: 11 }} />
              <YAxis allowDecimals={false} tick={{ fontSize: 11 }} />
              <Tooltip />
              <Area type="monotone" dataKey="demandes" stroke="#2563eb" fill="url(#timelineGradient)" strokeWidth={2} />
            </AreaChart>
          </ResponsiveContainer>
        </Card>

        <Card variant="dashboard" className="h-[330px] p-4">
          <h2 className="mb-2 font-title text-lg font-semibold text-slate-900">Distribution des statuts</h2>
          <ResponsiveContainer width="100%" height="88%">
            <PieChart>
              <Pie data={data.byStatus} dataKey="value" nameKey="name" innerRadius={55} outerRadius={95} paddingAngle={3}>
                {data.byStatus.map((entry, index) => {
                  const palette = ["#f59e0b", "#14b8a6", "#3b82f6", "#eab308", "#0ea5e9", "#10b981", "#ef4444"];
                  return <Cell key={`${entry.name}-${index}`} fill={palette[index % palette.length]} />;
                })}
              </Pie>
              <Tooltip />
            </PieChart>
          </ResponsiveContainer>
        </Card>
      </div>

      <div className="grid gap-4 xl:grid-cols-2">
        <Card variant="dashboard" className="h-[330px] p-4">
          <h2 className="mb-2 font-title text-lg font-semibold text-slate-900">Repartition par priorite</h2>
          <ResponsiveContainer width="100%" height="88%">
            <BarChart data={data.byPriority}>
              <CartesianGrid stroke="#e2e8f0" strokeDasharray="3 3" />
              <XAxis dataKey="level" />
              <YAxis allowDecimals={false} />
              <Tooltip />
              <Bar dataKey="count" radius={[8, 8, 0, 0]}>
                {data.byPriority.map((entry) => (
                  <Cell key={entry.level} fill={entry.color} />
                ))}
              </Bar>
            </BarChart>
          </ResponsiveContainer>
        </Card>

        <Card variant="dashboard" className="h-[330px] p-4">
          <h2 className="mb-2 font-title text-lg font-semibold text-slate-900">SLA par priorite</h2>
          <ResponsiveContainer width="100%" height="88%">
            <BarChart data={data.slaByPriority}>
              <CartesianGrid stroke="#e2e8f0" strokeDasharray="3 3" />
              <XAxis dataKey="level" />
              <YAxis domain={[0, 100]} />
              <Tooltip formatter={(value) => [`${value}%`, "SLA"]} />
              <Bar dataKey="sla" fill="#0ea5e9" radius={[8, 8, 0, 0]} />
            </BarChart>
          </ResponsiveContainer>
        </Card>
      </div>

      <Card variant="dashboard" className="p-4">
        <h2 className="mb-3 font-title text-lg font-semibold text-slate-900">Heatmap heure x jour</h2>
        <div className="overflow-auto">
          <div className="min-w-[780px] space-y-1">
            {data.heatmap.map((row) => (
              <div key={row.day} className="grid grid-cols-[60px_1fr] items-center gap-2">
                <span className="text-xs font-semibold text-slate-500">{row.day}</span>
                <div
                  className="grid gap-1"
                  style={{
                    gridTemplateColumns: "repeat(24, minmax(0, 1fr))",
                  }}
                >
                  {row.hours.map((cell) => {
                    const intensity = cell.count / maxHeat;
                    return (
                      <div
                        key={`${row.day}-${cell.hour}`}
                        className="h-5 rounded-sm"
                        style={{
                          backgroundColor: `rgba(37, 99, 235, ${0.08 + intensity * 0.9})`,
                        }}
                        title={`${row.day} ${cell.hour}h: ${cell.count}`}
                      />
                    );
                  })}
                </div>
              </div>
            ))}
          </div>
        </div>
      </Card>
    </section>
  );
}
