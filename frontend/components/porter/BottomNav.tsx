import Link from "next/link";

import { cx } from "@/lib/utils";

const items = [
  { href: "/porter", label: "File" },
  { href: "/porter/missions", label: "Missions" },
  { href: "/porter/active", label: "Active" },
  { href: "/porter/messages", label: "Messages" },
  { href: "/porter/settings", label: "Reglages" },
];

export function BottomNav({ currentPath }: { currentPath: string }) {
  return (
    <nav className="fixed bottom-0 left-0 right-0 z-20 border-t border-porter-border bg-porter-surface/95 px-2 pb-2 pt-1 backdrop-blur">
      <div className="mx-auto grid w-full max-w-3xl grid-cols-5 gap-1">
        {items.map((item) => {
          const active = currentPath === item.href;

          return (
            <Link
              key={item.href}
              data-testid={`porter-nav-${item.href.replaceAll("/", "-")}`}
              className={cx(
                "flex min-h-[60px] flex-col items-center justify-center rounded-lg px-1 text-[11px] font-semibold tracking-wide transition",
                active ? "bg-[#54ACBF] text-[#011C40]" : "text-[#B9D9E6] hover:-translate-y-px hover:bg-porter-surface-elev",
              )}
              href={item.href}
            >
              <span>{item.label}</span>
            </Link>
          );
        })}
      </div>
    </nav>
  );
}
