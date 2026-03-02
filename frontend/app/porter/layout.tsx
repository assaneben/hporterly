"use client";

import { usePathname } from "next/navigation";

import { BottomNav } from "@/components/porter/BottomNav";
import { PorterHeader } from "@/components/porter/PorterHeader";
import { useAuthStore } from "@/stores/authStore";

export default function PorterLayout({ children }: { children: React.ReactNode }) {
  const pathname = usePathname();
  const user = useAuthStore((state) => state.user);

  return (
    <div className="min-h-screen bg-porter-bg pb-24 text-[#EAF4F9]">
      <PorterHeader name={`${user?.first_name ?? ""} ${user?.last_name ?? ""}`.trim() || "Brancardier"} />
      <main id="main-content" className="hply-fade-in mx-auto w-full max-w-3xl space-y-3 px-4 pb-3 pt-4">
        {children}
      </main>
      <BottomNav currentPath={pathname} />
    </div>
  );
}
