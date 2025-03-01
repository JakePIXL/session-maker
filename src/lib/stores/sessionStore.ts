import { writable, derived, get } from 'svelte/store';

// Define SessionType first
export type SessionType = {
  id: string;
  title: string;
  description: string;
  start_time: string;
  end_time?: string;
  duration: number;
  markers?: Array<{
    timestamp: string;
    label: string;
    notes?: string;
  }>;
};

export type SessionStoreType = {
  isActive: boolean;
  lastSession: SessionType | null;
  currentDuration: number; // Add currentDuration for active session time tracking
};

// Define the initial state
const initialState: SessionStoreType = {
  isActive: false,
  lastSession: null,
  currentDuration: 0
};

// Create the base store
const sessionStoreBase = writable<SessionStoreType>(initialState);

// Setup a timer to update the duration
let timer: number | null = null;

// Function to start the timer
export function startSessionTimer() {
  // Clear any existing timer
  if (timer) {
    clearInterval(timer);
  }
  
  // Set up a timer to update the currentDuration every second
  timer = setInterval(() => {
    sessionStoreBase.update(state => {
      if (state.isActive && state.lastSession) {
        const startTime = new Date(state.lastSession.start_time).getTime();
        const now = new Date().getTime();
        return {
          ...state,
          currentDuration: now - startTime
        };
      }
      return state;
    });
  }, 1000);
}

// Function to stop the timer
export function stopSessionTimer() {
  if (timer) {
    clearInterval(timer);
    timer = null;
  }
}

// Create a custom store with helper methods
function createSessionStore() {
  const { subscribe, set, update } = sessionStoreBase;

  return {
    subscribe,
    set,
    update,
    // Helper method to set active session
    setActiveSession: (session: SessionType) => {
      update(state => ({
        ...state,
        isActive: true,
        lastSession: session
      }));
      startSessionTimer();
    },
    // Helper method to stop active session
    stopActiveSession: () => {
      update(state => ({
        ...state,
        isActive: false
      }));
      stopSessionTimer();
    },
    // Helper method to update last session
    setLastSession: (session: SessionType) => {
      update(state => ({
        ...state,
        lastSession: session
      }));
    },
    // Reset the store
    reset: () => {
      stopSessionTimer();
      set(initialState);
    }
  };
}

// Export the session store
export const sessionStore = createSessionStore();
