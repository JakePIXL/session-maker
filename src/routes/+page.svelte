<script context="module">
    function formatDuration(ms: number): string {
        const seconds = Math.floor((ms / 1000) % 60);
        const minutes = Math.floor((ms / (1000 * 60)) % 60);
        const hours = Math.floor(ms / (1000 * 60 * 60));

        if (hours > 0) {
            return `${hours}h ${minutes}m ${seconds}s`;
        } else if (minutes > 0) {
            return `${minutes}m ${seconds}s`;
        } else {
            return `${seconds}s`;
        }
    }
</script>

<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import { sessionStore, type SessionType } from "$lib/stores/sessionStore";
    import Timeline from "$lib/components/Timeline.svelte";
    import toast from "svelte-french-toast";

    let activeSession: any = null;
    let unlisten: any[] = [];

    async function checkActiveSession() {
        try {
            const session = await invoke("get_active_session");
            if (session) {
                activeSession = session;
                $sessionStore.isActive = true;
            } else {
                activeSession = null;
                $sessionStore.isActive = false;
            }
        } catch (error) {
            console.error("Failed to check active session:", error);
        }
    }

    async function startSession() {
        try {
            const id = await invoke("start_session");
            toast.success("Session started");
            checkActiveSession();
        } catch (error) {
            toast.error(`Failed to start session: ${error}`);
        }
    }

    async function stopSession() {
        try {
            const session: SessionType = await invoke("stop_session");
            toast.success("Session stopped");
            $sessionStore.lastSession = session;
            checkActiveSession();
        } catch (error) {
            toast.error(`Failed to stop session: ${error}`);
        }
    }

    async function addMarker() {
        try {
            const marker = await invoke("add_marker", { label: "Marker" });
            toast.success("Marker added");
            checkActiveSession();
        } catch (error) {
            toast.error(`Failed to add marker: ${error}`);
        }
    }

    onMount(async () => {
        await checkActiveSession();

        // Set up event listeners
        unlisten.push(
            await listen("session-started", (_) => {
                checkActiveSession();
            }),
        );

        unlisten.push(
            await listen("session-stopped", (event) => {
                $sessionStore.lastSession = event.payload as any;
                checkActiveSession();
            }),
        );

        unlisten.push(
            await listen("marker-added", (_) => {
                checkActiveSession();
            }),
        );

        unlisten.push(
            await listen("notification", (event) => {
                const [title, message] = event.payload as [string, string];
                toast(message, {
                    duration: 3000,
                    position: "top-right",
                });
            }),
        );
    });

    onDestroy(() => {
        unlisten.forEach((unlistenFn) => unlistenFn());
    });
</script>

<div class="container mx-auto p-4">
    <header class="mb-8">
        <h1 class="text-3xl font-bold text-gray-800 mb-2">Dibik'aandaagozi</h1>
        <p class="text-gray-600">Session Tracking with Timeline Markers</p>
    </header>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div class="col-span-1 bg-white rounded-lg shadow p-6">
            <h2 class="text-xl font-semibold mb-4">Session Control</h2>

            {#if $sessionStore.isActive}
                <div
                    class="bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded mb-4"
                    role="alert"
                >
                    <strong class="font-bold">Active Session</strong>
                    <p>
                        Session running for {activeSession
                            ? formatDuration(
                                  new Date().getTime() -
                                      new Date(
                                          activeSession.start_time,
                                      ).getTime(),
                              )
                            : "0m"}
                    </p>
                    <p class="text-sm">
                        Markers: {activeSession?.markers?.length || 0}
                    </p>
                </div>

                <div class="flex flex-col space-y-3">
                    <button
                        on:click={stopSession}
                        class="bg-red-500 hover:bg-red-600 text-white py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                    >
                        Stop Session
                    </button>

                    <button
                        on:click={addMarker}
                        class="bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                    >
                        Add Marker
                    </button>
                </div>
            {:else}
                <div
                    class="bg-gray-100 border border-gray-300 text-gray-700 px-4 py-3 rounded mb-4"
                    role="alert"
                >
                    <strong class="font-bold">No Active Session</strong>
                    <p>
                        Press the button below or use the hotkey (Ctrl+Shift+S)
                        to start a new session.
                    </p>
                </div>

                <button
                    on:click={startSession}
                    class="bg-green-500 hover:bg-green-600 text-white py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                >
                    Start New Session
                </button>
            {/if}

            <div class="mt-8">
                <h3 class="text-lg font-semibold mb-2">Hotkeys</h3>
                <ul class="text-gray-700 space-y-2">
                    <li class="flex justify-between">
                        <span>Start/Stop Session:</span>
                        <code class="bg-gray-200 px-2 py-1 rounded"
                            >Ctrl+Shift+S</code
                        >
                    </li>
                    <li class="flex justify-between">
                        <span>Add Marker:</span>
                        <code class="bg-gray-200 px-2 py-1 rounded"
                            >Ctrl+Shift+M</code
                        >
                    </li>
                </ul>
            </div>
        </div>

        <div class="col-span-1 md:col-span-2 bg-white rounded-lg shadow p-6">
            <h2 class="text-xl font-semibold mb-4">Current Timeline</h2>

            {#if activeSession}
                <Timeline session={activeSession} />
            {:else if $sessionStore.lastSession}
                <Timeline session={$sessionStore.lastSession} />
            {:else}
                <div class="text-center py-10 text-gray-500">
                    <p>No session data to display.</p>
                    <p class="mt-2">
                        Start a new session or view past sessions in the History
                        tab.
                    </p>
                </div>
            {/if}
        </div>
    </div>
</div>
