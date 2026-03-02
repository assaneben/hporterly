import type { Metadata } from "next";

import "@/styles/globals.css";
import { OfflineBanner } from "@/components/layout/OfflineBanner";
import { SkipToContent } from "@/components/layout/SkipToContent";
import { SyncBadge } from "@/components/layout/SyncBadge";
import { AuthProvider } from "@/components/providers/AuthProvider";
import { ThemeProvider } from "@/components/providers/ThemeProvider";
import { ToastProvider } from "@/components/providers/ToastProvider";

export const metadata: Metadata = {
  title: "HPorterly",
  description:
    "Plateforme web securisee de coordination des transports internes en environnement de sante generique",
  manifest: "/manifest.json",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="fr">
      <body>
        <SkipToContent />
        <ThemeProvider>
          <AuthProvider>
            <ToastProvider>
              <OfflineBanner />
              <div className="fixed right-4 top-4 z-[200]">
                <SyncBadge />
              </div>
              {children}
            </ToastProvider>
          </AuthProvider>
        </ThemeProvider>
      </body>
    </html>
  );
}