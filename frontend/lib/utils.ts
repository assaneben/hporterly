import { format } from "date-fns";
import { fr } from "date-fns/locale";

export function formatDate(date: Date | string | null | undefined): string {
  if (!date) {
    return "-";
  }

  const value = typeof date === "string" ? new Date(date) : date;
  return format(value, "dd/MM/yyyy HH:mm", { locale: fr });
}

export function escapeHtml(value: string): string {
  return value
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#039;");
}

export function debounce<T extends (...args: unknown[]) => void>(fn: T, wait: number): T {
  let timer: ReturnType<typeof setTimeout> | undefined;

  return ((...args: unknown[]) => {
    if (timer) {
      clearTimeout(timer);
    }

    timer = setTimeout(() => {
      fn(...args);
    }, wait);
  }) as T;
}

export function cx(...classes: Array<string | false | null | undefined>): string {
  return classes.filter(Boolean).join(" ");
}