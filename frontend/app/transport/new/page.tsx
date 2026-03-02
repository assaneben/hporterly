"use client";

import { useMemo, useState } from "react";
import { useRouter } from "next/navigation";

import { EquipmentForm } from "@/components/transport/EquipmentForm";
import { FormSummary } from "@/components/transport/FormSummary";
import { PatientForm } from "@/components/transport/PatientForm";
import { PriorityBadge } from "@/components/transport/PriorityBadge";
import { SpecimenForm } from "@/components/transport/SpecimenForm";
import { TypeSelector } from "@/components/transport/TypeSelector";
import { useToast } from "@/components/providers/ToastProvider";
import { Button } from "@/components/ui/Button";
import { InlineFeedback } from "@/components/ui/InlineFeedback";
import { api } from "@/lib/api";
import { evaluatePriority, getDefaultPriorityConfig, buildPriorityContext } from "@/lib/priority-engine";

type TransportType = "PATIENT" | "EQUIPMENT" | "SPECIMEN";
type FormState = Record<string, string | boolean | undefined>;

const initialState: FormState = {
  transport_type: "PATIENT",
  transport_subtype: "TP-BRANC",
  patient_name: "",
  origin: "",
  destination: "",
  mode: "Brancard",
};

function isBlank(value: string | boolean | undefined): boolean {
  return String(value ?? "").trim().length === 0;
}

export default function NewTransportPage() {
  const router = useRouter();
  const { pushToast } = useToast();
  const [step, setStep] = useState(1);
  const [values, setValues] = useState<FormState>(initialState);
  const [loading, setLoading] = useState(false);
  const [stepError, setStepError] = useState<string | null>(null);

  const transportType = (values.transport_type as TransportType | undefined) ?? "PATIENT";

  const priorityResult = useMemo(() => {
    const context = buildPriorityContext({
      origin: String(values.origin ?? ""),
      destination: String(values.destination ?? ""),
      scheduled_time: String(values.scheduled_time ?? ""),
      vital_emergency: Boolean(values.vital_emergency),
      intubated_ventilated: Boolean(values.intubated_ventilated),
      patient_monitoring: Boolean(values.patient_monitoring),
      needs_o2: Boolean(values.needs_o2),
      needs_perfusion: Boolean(values.needs_perfusion),
      isolation: Boolean(values.isolation),
      patient_agitated: Boolean(values.patient_agitated),
      ide_accompanying_confirmed: Boolean(values.ide_accompanying_confirmed),
      patient_bariatric: Boolean(values.patient_bariatric),
      patient_dialysis: Boolean(values.patient_dialysis),
      patient_psychiatry: Boolean(values.patient_psychiatry),
      patient_contentious: Boolean(values.patient_contentious),
      patient_over_120kg: Boolean(values.patient_over_120kg),
      patient_stable: values.patient_stable !== false,
      transport_type: transportType,
    });

    return evaluatePriority(context, getDefaultPriorityConfig());
  }, [transportType, values]);

  const setField = (name: string, value: string | boolean) => {
    setValues((current) => ({
      ...current,
      [name]: value,
    }));
  };

  const validateStepTwo = () => {
    if (isBlank(values.origin) || isBlank(values.destination)) {
      return "Les champs origine et destination sont obligatoires.";
    }

    if (transportType === "PATIENT" && isBlank(values.patient_name)) {
      return "Le nom du patient est obligatoire.";
    }

    if (transportType === "EQUIPMENT" && isBlank(values.equipment_recipient_patient_name)) {
      return "Indiquez un patient destinataire pour le materiel.";
    }

    if (transportType === "SPECIMEN" && isBlank(values.specimen_types)) {
      return "Le type de prelevement est obligatoire.";
    }

    return null;
  };

  const submit = async () => {
    const validationError = validateStepTwo();
    if (validationError) {
      setStepError(validationError);
      setStep(2);
      return;
    }

    setLoading(true);
    setStepError(null);

    const payload = {
      patient_name: String(values.patient_name ?? values.equipment_recipient_patient_name ?? "Transport interne"),
      patient_ipp: values.patient_ipp ? String(values.patient_ipp) : undefined,
      origin: String(values.origin ?? ""),
      destination: String(values.destination ?? ""),
      priority: priorityResult.priority,
      mode: String(values.mode ?? "Brancard"),
      notes: values.notes ? String(values.notes) : undefined,
      transport_type: transportType,
      transport_subtype: String(values.transport_subtype ?? (transportType === "PATIENT" ? "TP-BRANC" : "MAT-GENERIQUE")),
      needs_o2: Boolean(values.needs_o2),
      needs_perfusion: Boolean(values.needs_perfusion),
      isolation: Boolean(values.isolation),
      patient_agitated: Boolean(values.patient_agitated),
      patient_monitoring: Boolean(values.patient_monitoring),
      needs_two_porters: Boolean(values.needs_two_porters),
      patient_contentious: Boolean(values.patient_contentious),
      patient_over_120kg: Boolean(values.patient_over_120kg),
      patient_bariatric: Boolean(values.patient_bariatric),
      patient_psychiatry: Boolean(values.patient_psychiatry),
      patient_dialysis: Boolean(values.patient_dialysis),
      equipment_recipient_patient_name: values.equipment_recipient_patient_name
        ? String(values.equipment_recipient_patient_name)
        : undefined,
      equipment_size: values.equipment_size ? String(values.equipment_size) : undefined,
      laboratory_name: values.laboratory_name ? String(values.laboratory_name) : undefined,
      specimen_types: values.specimen_types ? [String(values.specimen_types)] : undefined,
      notes_for_reception: values.notes_for_reception ? String(values.notes_for_reception) : undefined,
      motif: values.motif ? String(values.motif) : undefined,
      ide_accompanying_confirmed: Boolean(values.ide_accompanying_confirmed),
    };

    try {
      await api.post("/tickets", payload);
      pushToast({ title: "Demande creee", type: "success" });
      router.push("/dashboard");
    } catch (error) {
      pushToast({
        title: "Echec de creation",
        message: error instanceof Error ? error.message : "Erreur inconnue",
        type: "error",
      });
    } finally {
      setLoading(false);
    }
  };

  return (
    <main id="main-content" className="mx-auto max-w-5xl space-y-4 p-4">
      <header className="hply-fade-in rounded-xl border border-slate-200 bg-white px-5 py-4 shadow-sm">
        <h1 className="font-title text-3xl font-bold text-slate-900">Nouvelle demande de transport</h1>
        <p className="mt-1 text-sm text-slate-500">Saisissez les informations puis confirmez la demande.</p>
      </header>

      <section className="hply-fade-in hply-fade-in-delay-1 space-y-4 rounded-xl border border-slate-200 bg-white p-4 shadow-sm">
        <div className="flex flex-wrap items-center gap-2 text-sm">
          {[1, 2, 3].map((value) => (
            <div key={value} className="flex items-center gap-2">
              <span
                className={`inline-flex h-7 w-7 items-center justify-center rounded-full border text-xs font-semibold ${
                  step >= value ? "border-sky-500 bg-sky-50 text-sky-700" : "border-slate-300 text-slate-400"
                }`}
              >
                {value}
              </span>
              <span className={step === value ? "font-semibold text-slate-900" : "text-slate-500"}>{`Etape ${value}`}</span>
              {value < 3 ? <span className="text-slate-300">→</span> : null}
            </div>
          ))}
        </div>

        {stepError ? <InlineFeedback message={stepError} tone="error" /> : null}

        {step === 1 ? (
          <TypeSelector
            value={transportType}
            onChange={(value) => {
              setField("transport_type", value);
              if (value === "PATIENT") {
                setField("transport_subtype", "TP-BRANC");
              }
              if (value === "EQUIPMENT") {
                setField("transport_subtype", "MAT-GENERIQUE");
              }
              if (value === "SPECIMEN") {
                setField("transport_subtype", "SPECIMEN");
              }
            }}
          />
        ) : null}

        {step === 2 ? (
          <div className="space-y-4">
            {transportType === "PATIENT" ? <PatientForm values={values} onChange={setField} /> : null}
            {transportType === "EQUIPMENT" ? <EquipmentForm values={values} onChange={setField} /> : null}
            {transportType === "SPECIMEN" ? <SpecimenForm values={values} onChange={setField} /> : null}
            <PriorityBadge priority={priorityResult.priority} reason={priorityResult.reason} />
          </div>
        ) : null}

        {step === 3 ? (
          <div className="space-y-4">
            <FormSummary values={values} />
            <PriorityBadge priority={priorityResult.priority} reason={priorityResult.reason} />
          </div>
        ) : null}

        <div className="flex flex-wrap gap-2">
          {step > 1 ? (
            <Button variant="ghost" onClick={() => setStep((value) => value - 1)}>
              Retour
            </Button>
          ) : null}

          {step < 3 ? (
            <Button
              onClick={() => {
                if (step === 2) {
                  const validationError = validateStepTwo();
                  if (validationError) {
                    setStepError(validationError);
                    return;
                  }
                }
                setStepError(null);
                setStep((value) => value + 1);
              }}
            >
              Suivant
            </Button>
          ) : (
            <Button disabled={loading} onClick={submit}>
              {loading ? "Envoi..." : "Confirmer et envoyer"}
            </Button>
          )}
        </div>
      </section>
    </main>
  );
}
