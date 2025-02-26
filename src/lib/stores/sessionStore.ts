import { writable } from "svelte/store";

// Define SessionType first
export type SessionType = {
  id: string;
  title: string;
  description: string;
  duration: number;
};

// Define the initial state
const initialState = {
  isActive: false,
  lastSession: null as SessionType | null,
};

// Create the store
export const sessionStore = writable(initialState);
