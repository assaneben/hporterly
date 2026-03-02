import { cx } from "@/lib/utils";

type FeedbackTone = "info" | "success" | "warning" | "error";

const tones: Record<FeedbackTone, string> = {
  info: "border-sky-200 bg-sky-50 text-sky-800",
  success: "border-emerald-200 bg-emerald-50 text-emerald-800",
  warning: "border-amber-200 bg-amber-50 text-amber-800",
  error: "border-red-200 bg-red-50 text-red-800",
};

type Props = {
  title?: string;
  message: string;
  tone?: FeedbackTone;
  className?: string;
};

export function InlineFeedback({ title, message, tone = "info", className }: Props) {
  return (
    <div
      className={cx("rounded-lg border px-3 py-2 text-sm", tones[tone], className)}
      role={tone === "error" ? "alert" : "status"}
    >
      {title ? <p className="font-semibold">{title}</p> : null}
      <p className={title ? "mt-0.5" : ""}>{message}</p>
    </div>
  );
}
