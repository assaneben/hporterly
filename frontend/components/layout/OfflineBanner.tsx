"use client";

import { useEffect, useState } from "react";

export function OfflineBanner() {
  const [offline, setOffline] = useState(false);

  useEffect(() => {
    const update = () => {
      setOffline(!navigator.onLine);
    };

    update();

    window.addEventListener("online", update);
    window.addEventListener("offline", update);

    return () => {
      window.removeEventListener("online", update);
      window.removeEventListener("offline", update);
    };
  }, []);

  if (!offline) {
    return null;
  }

  return (
    <div id="offline-banner" className="w-full px-4 py-2 text-center text-sm font-medium">
      Vous etes hors ligne. Les donnees peuvent etre obsoletes.
    </div>
  );
}