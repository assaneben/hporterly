"use client";

import { useEffect } from "react";
import { usePathname } from "next/navigation";

export function ThemeProvider({ children }: { children: React.ReactNode }) {
  const pathname = usePathname();

  useEffect(() => {
    const body = document.body;
    body.classList.remove("theme-login", "theme-dashboard", "theme-porter");

    if (pathname.startsWith("/dashboard") || pathname.startsWith("/admin") || pathname.startsWith("/transport")) {
      body.classList.add("theme-dashboard");
      return;
    }

    if (pathname.startsWith("/porter")) {
      body.classList.add("theme-porter");
      return;
    }

    body.classList.add("theme-login");
  }, [pathname]);

  return <>{children}</>;
}
