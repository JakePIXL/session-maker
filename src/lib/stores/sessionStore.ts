// Using Svelte 5 reactive state instead of stores
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

// Create the store using $state for Svelte 5
let sessionStore = $state(initialState);

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
    if (sessionStore.isActive && sessionStore.lastSession) {
      const startTime = new Date(sessionStore.lastSession.start_time).getTime();
      const now = new Date().getTime();
      sessionStore.currentDuration = now - startTime;
    }
  }, 1000);
}

// Function to stop the timer
export function stopSessionTimer() {
  if (timer) {
    clearInterval(timer);
    timer = null;
  }
}

// Export the session store
export { sessionStore };
