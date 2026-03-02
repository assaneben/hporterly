import { cx } from "@/lib/utils";

export function Spinner({ small }: { small?: boolean }) {
  return (
    <div
      className={cx(
        "animate-spin rounded-full border-2 border-primary/40 border-t-primary",
        small ? "h-5 w-5" : "h-10 w-10 border-[3px]",
      )}
      aria-label="Chargement"
      role="status"
    />
  );
}