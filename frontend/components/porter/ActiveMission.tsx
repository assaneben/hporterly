import { Button } from "@/components/ui/Button";
import { getStatusLabel } from "@/lib/ticket-display";
import type { Ticket } from "@/lib/types";

import { PhaseIndicator } from "./PhaseIndicator";

const PHASES = ["Assignation", "Prise en charge", "Transport", "Livraison", "Completion"];

type Props = {
  ticket: Ticket;
  currentPhase: number;
  onAdvance: () => void;
  onSuspend: () => void;
  advanceDisabled?: boolean;
  suspendDisabled?: boolean;
};

export function ActiveMission({
  ticket,
  currentPhase,
  onAdvance,
  onSuspend,
  advanceDisabled,
  suspendDisabled,
}: Props) {
  return (
    <section className="hply-lift rounded-xl border border-porter-border bg-porter-surface p-5 text-[#EAF4F9] shadow-card">
      <h2 className="font-title text-2xl font-semibold">Mission active</h2>
      <PhaseIndicator phases={PHASES} current={currentPhase} />

      <div className="mt-4 rounded-lg border border-porter-border bg-porter-surface-elev p-4">
        <p className="font-title text-xl font-semibold">{ticket.patientName}</p>
        <p className="mt-1 text-sm text-[#B9D9E6]">
          {ticket.origin} -&gt; {ticket.destination}
        </p>
        <p className="mt-2 text-xs uppercase tracking-[0.08em] text-[#B9D9E6]">{getStatusLabel(ticket.status)}</p>
      </div>

      <div className="mt-4 grid gap-2 sm:grid-cols-2">
        <Button data-testid="mission-advance-button" className="min-h-[60px]" disabled={advanceDisabled} onClick={onAdvance}>
          Avancer la mission
        </Button>
        <Button
          data-testid="mission-suspend-button"
          className="min-h-[60px]"
          disabled={suspendDisabled}
          onClick={onSuspend}
          variant="porter"
        >
          Suspendre mission
        </Button>
      </div>
    </section>
  );
}
