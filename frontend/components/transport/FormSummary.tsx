type Values = Record<string, string | boolean | undefined>;

export function FormSummary({ values }: { values: Values }) {
  return (
    <div className="hply-lift rounded-lg border border-slate-200 bg-slate-50 p-4 text-slate-800">
      <h3 className="font-title text-lg font-semibold">Resume de la demande</h3>
      <dl className="mt-3 grid gap-2 text-sm">
        <div className="flex justify-between gap-2">
          <dt className="text-slate-500">Type</dt>
          <dd>{String(values.transport_type ?? "PATIENT")}</dd>
        </div>
        <div className="flex justify-between gap-2">
          <dt className="text-slate-500">Patient / Objet</dt>
          <dd>{String(values.patient_name ?? values.equipment_recipient_patient_name ?? "-")}</dd>
        </div>
        <div className="flex justify-between gap-2">
          <dt className="text-slate-500">Origine</dt>
          <dd>{String(values.origin ?? "-")}</dd>
        </div>
        <div className="flex justify-between gap-2">
          <dt className="text-slate-500">Destination</dt>
          <dd>{String(values.destination ?? "-")}</dd>
        </div>
        <div className="flex justify-between gap-2">
          <dt className="text-slate-500">Mode</dt>
          <dd>{String(values.mode ?? "-")}</dd>
        </div>
      </dl>
    </div>
  );
}
