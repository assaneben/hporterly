"use client";

import Link from "next/link";

export function SkipToContent() {
  return (
    <Link
      href="#main-content"
      className="absolute left-2 top-2 -translate-y-20 rounded bg-primary px-3 py-2 text-sm text-white focus:translate-y-0"
    >
      Aller au contenu principal
    </Link>
  );
}