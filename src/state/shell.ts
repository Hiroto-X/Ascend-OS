import { create } from "zustand";

interface ShellState {
  isNavigationOpen: boolean;
  setNavigationOpen: (isNavigationOpen: boolean) => void;
}

export const useShellStore = create<ShellState>((set) => ({
  isNavigationOpen: true,
  setNavigationOpen: (isNavigationOpen) => set({ isNavigationOpen }),
}));
