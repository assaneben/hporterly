import { Button } from "@/components/ui/Button";

export function ReinforcementBanner({ onRequest }: { onRequest: () => void }) {
  return (
    <div className="rounded-lg border border-amber-400 bg-amber-100 p-3 text-amber-900">
      <p className="text-sm font-semibold">Besoin d'un renfort ?</p>
      <p className="text-xs">Demander un co-partenaire pour cette mission.</p>
      <Button className="mt-2" variant="ghost" onClick={onRequest}>
        Demander renfort
      </Button>
    </div>
  );
}