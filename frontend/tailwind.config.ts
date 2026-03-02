import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./hooks/**/*.{js,ts,jsx,tsx,mdx}",
    "./lib/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ["Inter", "system-ui", "-apple-system", "sans-serif"],
        title: ["Outfit", "system-ui", "-apple-system", "sans-serif"],
      },
      colors: {
        primary: "#54acbf",
        "primary-dark": "#26658c",
        "primary-light": "#7bc7d8",
        accent: "#10b981",
        "accent-glow": "rgba(84,172,191,0.42)",
        danger: "#ef4444",
        warning: "#f59e0b",
        info: "#3b82f6",
        success: "#10b981",
        "dash-bg": "#f1f5f9",
        "dash-border": "#e2e8f0",
        "dash-hover": "#f8fafc",
        "porter-bg": "#011C40",
        "porter-surface": "#012A4A",
        "porter-surface-elev": "#0F3A5F",
        "porter-border": "#1F6A8A",
      },
      borderRadius: {
        sm: "8px",
        md: "12px",
        lg: "16px",
        xl: "24px",
      },
      boxShadow: {
        card: "0 4px 6px -1px rgba(0,0,0,0.1), 0 2px 4px -1px rgba(0,0,0,0.06)",
        glow: "0 0 20px rgba(84,172,191,0.3)",
        glass: "0 8px 32px 0 rgba(0,0,0,0.36)",
      },
      spacing: {
        xs: "0.25rem",
        sm: "0.5rem",
        md: "1rem",
        lg: "1.5rem",
        xl: "2rem",
        "2xl": "3rem",
      },
      keyframes: {
        hplyPulse: {
          "0%, 100%": { transform: "scale(1)", opacity: "1" },
          "50%": { transform: "scale(1.08)" },
        },
        skeletonShimmer: {
          "0%": { backgroundPosition: "0%" },
          "100%": { backgroundPosition: "-200%" },
        },
        msgPulse: {
          "0%, 100%": { opacity: "1" },
          "50%": { opacity: "0.5" },
        },
      },
      animation: {
        hplyPulse: "hplyPulse 1.5s ease-in-out infinite",
        skeleton: "skeletonShimmer 1.2s ease-in-out infinite",
        msgPulse: "msgPulse 2.5s ease-in-out infinite",
      },
      screens: {
        tablet: "900px",
      },
    },
  },
  plugins: [],
};

export default config;