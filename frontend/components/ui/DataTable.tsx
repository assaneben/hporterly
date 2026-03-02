"use client";

import { useEffect, useMemo, useState } from "react";

import { cx } from "@/lib/utils";

type Column<T> = {
  id: string;
  header: string;
  width?: string;
  cell: (row: T) => React.ReactNode;
  sortable?: boolean;
  sortValue?: (row: T) => string | number;
};

type Props<T> = {
  data: T[];
  columns: Column<T>[];
  pageSize?: number;
  rowKey: (row: T) => string;
  title?: string;
};

export function DataTable<T>({ data, columns, pageSize = 25, rowKey, title }: Props<T>) {
  const [page, setPage] = useState(1);
  const [sort, setSort] = useState<{ id: string; direction: "asc" | "desc" } | null>(null);

  const sorted = useMemo(() => {
    if (!sort) return data;

    const column = columns.find((item) => item.id === sort.id);
    if (!column?.sortValue) return data;

    const next = [...data].sort((a, b) => {
      const left = column.sortValue!(a);
      const right = column.sortValue!(b);
      if (left < right) return sort.direction === "asc" ? -1 : 1;
      if (left > right) return sort.direction === "asc" ? 1 : -1;
      return 0;
    });

    return next;
  }, [columns, data, sort]);

  const pages = Math.max(1, Math.ceil(sorted.length / pageSize));
  const start = (page - 1) * pageSize;
  const paged = sorted.slice(start, start + pageSize);

  useEffect(() => {
    setPage((current) => Math.min(current, pages));
  }, [pages]);

  useEffect(() => {
    setPage(1);
  }, [sort, data.length, pageSize]);

  return (
    <section className="overflow-hidden rounded-xl border border-dash-border bg-white shadow-sm">
      <div className="flex flex-wrap items-center justify-between gap-2 border-b border-dash-border bg-slate-50 px-4 py-3.5">
        <h2 className="font-title text-[17px] font-semibold text-slate-900">{title ?? "Demandes"}</h2>
        <p className="text-xs font-medium text-slate-500">{sorted.length} element(s)</p>
      </div>

      <div className="hply-soft-scroll overflow-auto">
        <table className="min-w-[1180px] table-fixed">
          <thead className="sticky top-0 z-10 border-b border-dash-border bg-slate-50/95 backdrop-blur">
            <tr>
              {columns.map((column) => (
                <th
                  key={column.id}
                  aria-sort={
                    column.sortable
                      ? sort?.id === column.id
                        ? sort.direction === "asc"
                          ? "ascending"
                          : "descending"
                        : "none"
                      : undefined
                  }
                  className="px-3 py-3 text-left text-[11px] font-semibold uppercase tracking-[0.13em] text-slate-500"
                  scope="col"
                  style={column.width ? { width: column.width } : undefined}
                >
                  <button
                    className={cx(
                      "inline-flex items-center gap-1 rounded px-1 py-0.5 disabled:opacity-100",
                      column.sortable ? "hover:bg-slate-200" : "cursor-default",
                    )}
                    disabled={!column.sortable}
                    onClick={() => {
                      if (!column.sortable) return;

                      setSort((current) => {
                        if (!current || current.id !== column.id) {
                          return { id: column.id, direction: "asc" };
                        }

                        return { id: column.id, direction: current.direction === "asc" ? "desc" : "asc" };
                      });
                    }}
                    type="button"
                  >
                    {column.header}
                    {sort?.id === column.id ? (
                      <span aria-hidden>{sort.direction === "asc" ? "↑" : "↓"}</span>
                    ) : null}
                  </button>
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {paged.map((row, index) => (
              <tr
                key={rowKey(row)}
                className={cx(
                  index % 2 === 0 ? "bg-white" : "bg-dash-hover",
                  "border-b border-slate-100 hover:bg-slate-50",
                )}
              >
                {columns.map((column) => (
                  <td key={column.id} className="px-3 py-3.5 text-sm leading-relaxed text-slate-700">
                    {column.cell(row)}
                  </td>
                ))}
              </tr>
            ))}
            {paged.length === 0 ? (
              <tr>
                <td className="px-4 py-10 text-center text-sm text-slate-500" colSpan={columns.length}>
                  Aucun resultat pour ces filtres.
                </td>
              </tr>
            ) : null}
          </tbody>
        </table>
      </div>

      <div className="flex flex-wrap items-center justify-between gap-2 border-t border-dash-border bg-slate-50 px-3 py-2.5 text-sm text-slate-600">
        <span>
          Page {page}/{pages}
        </span>
        <div className="flex gap-2">
          <button
            className="rounded-md border border-slate-300 bg-white px-3 py-1.5 text-xs font-semibold disabled:opacity-40"
            disabled={page <= 1}
            onClick={() => setPage((p) => Math.max(1, p - 1))}
            type="button"
          >
            {"<"} Prec
          </button>
          <button
            className="rounded-md border border-slate-300 bg-white px-3 py-1.5 text-xs font-semibold disabled:opacity-40"
            disabled={page >= pages}
            onClick={() => setPage((p) => Math.min(pages, p + 1))}
            type="button"
          >
            Suiv {">"}
          </button>
        </div>
      </div>
    </section>
  );
}
