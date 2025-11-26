import { create } from "zustand";

interface AppState {
  connections: string[];
}

export const useAppStore = create<AppState>()(() => ({
  connections: ["connection1", "connection2"]
}));
