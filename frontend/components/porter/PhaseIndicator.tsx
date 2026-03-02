import { cx } from "@/lib/utils";

export function PhaseIndicator({ phases, current }: { phases: string[]; current: number }) {
  return (
    <ol className="grid grid-cols-5 gap-1 text-[10px] sm:text-xs">
      {phases.map((phase, index) => (
        <li
          key={phase}
          className={cx(
            "rounded-md border px-2 py-2 text-center",
            index <= current
              ? "border-primary bg-gradient-to-r from-primary to-primary-dark text-white"
              : "border-porter-border text-[#B9D9E6]",
          )}
        >
          {phase}
        </li>
      ))}
    </ol>
  );
}