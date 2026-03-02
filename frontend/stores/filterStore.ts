import { create } from "zustand";

type FilterState = {
  search: string;
  status: string;
  priority: string;
  transportType: string;
  setFilter: (name: "search" | "status" | "priority" | "transportType", value: string) => void;
  reset: () => void;
};

const initialState = {
  search: "",
  status: "",
  priority: "",
  transportType: "",
};

export const useFilterStore = create<FilterState>((set) => ({
  ...initialState,
  setFilter: (name, value) => set({ [name]: value }),
  reset: () => set(initialState),
}));