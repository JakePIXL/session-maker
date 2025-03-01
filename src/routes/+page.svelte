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
    import { sessionStore, type SessionType, startSessionTimer, stopSessionTimer } from "$lib/stores/sessionStore";
    import Timeline from "$lib/components/Timeline.svelte";
    import toast from "svelte-french-toast";
    import { Accordion } from "bits-ui";

    let activeSession: any = null;
    let unlisten: any[] = [];

    async function checkActiveSession() {
        try {
            const session = await invoke("get_active_session");
            if (session) {
                activeSession = session;
                sessionStore.isActive = true;
                
                // Store last session and start the timer
                sessionStore.lastSession = session as any;
                startSessionTimer();
            } else {
                activeSession = null;
                sessionStore.isActive = false;
                stopSessionTimer();
            }
        } catch (error) {
            console.error("Failed to check active session:", error);
        }
    }

    async function startSession() {
        try {
            const id = await invoke("start_session");
            toast.success("SESSION STARTED");
            await checkActiveSession();
        } catch (error) {
            toast.error(`FAILED TO START SESSION: ${error}`);
        }
    }

    async function stopSession() {
        try {
            const session: SessionType = await invoke("stop_session");
            toast.success("SESSION STOPPED");
            sessionStore.lastSession = session;
            stopSessionTimer();
            await checkActiveSession();
        } catch (error) {
            toast.error(`FAILED TO STOP SESSION: ${error}`);
        }
    }

    async function addMarker() {
        try {
            const marker = await invoke("add_marker", { label: "Marker" });
            toast.success("MARKER ADDED");
            checkActiveSession();
        } catch (error) {
            toast.error(`FAILED TO ADD MARKER: ${error}`);
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
                sessionStore.lastSession = event.payload as any;
                stopSessionTimer();
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
        stopSessionTimer(); // Make sure we clean up the timer on component destroy
    });
</script>

<div class="container mx-auto">
    <header class="brutalist-header mt-8 mb-16">
        <h1 class="text-4xl font-bold uppercase tracking-tight">MarkerMoment</h1>
        <p class="text-xl uppercase mt-2 tracking-tight">Recording Session Marker</p>
    </header>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
        <div class="col-span-1 brutalist-card h-fit">
            <h2 class="text-2xl font-bold mb-6 uppercase tracking-tight">Session Control</h2>

            {#if sessionStore.isActive}
                <div
                    class="alert alert-success mb-6 transform rotate-[-0.5deg]"
                    role="alert"
                >
                    <div class="uppercase text-sm absolute -top-2 -left-2 bg-black text-white px-2 py-1">STATUS</div>
                    <strong class="font-bold block mb-2 uppercase mt-2">Active Session</strong>
                    <div class="p-2 border border-dashed border-black">
                        <p class="mb-2">
                            Running for <span class="font-bold">{formatDuration(sessionStore.currentDuration)}</span>
                        </p>
                        <p>
                            Markers: <span class="font-bold">{activeSession?.markers?.length || 0}</span>
                        </p>
                    </div>
                </div>

                <div class="flex flex-col space-y-4">
                    <button
                        onclick={stopSession}
                        class="border-2 border-black bg-red-500 hover:bg-red-600 active:bg-red-700 text-white font-bold py-3 transform rotate-[0.5deg] rounded shadow-sm transition-colors cursor-pointer"
                    >
                        STOP SESSION
                    </button>

                    <button
                        onclick={addMarker}
                        class="border-2 border-black bg-blue-500 hover:bg-blue-600 active:bg-blue-700 text-white font-bold py-3 transform rotate-[-0.5deg] rounded shadow-sm transition-colors cursor-pointer"
                    >
                        ADD MARKER
                    </button>
                </div>
            {:else}
                <div
                    class="alert mb-6 border-dashed transform rotate-[0.5deg]"
                    role="alert"
                >
                    <div class="uppercase text-sm absolute -top-2 -left-2 bg-black text-white px-2 py-1">STATUS</div>
                    <strong class="font-bold block mb-2 uppercase mt-2">No Active Session</strong>
                </div>

                <button
                    onclick={startSession}
                    class="border-2 border-black bg-green-500 hover:bg-green-600 active:bg-green-700 text-white font-bold py-3 w-full transform rotate-[-0.5deg] rounded shadow-sm transition-colors cursor-pointer"
                >
                    START NEW SESSION
                </button>
            {/if}

            <div class="brutalist-divider my-8"></div>

            <Accordion.Root type="single">
                <Accordion.Item value="hotkeys">
                    <Accordion.Header>
                        <Accordion.Trigger>HOTKEYS</Accordion.Trigger>
                    </Accordion.Header>
                    <Accordion.Content>
                        <ul class="space-y-4 pt-2">
                            <li class="flex justify-between items-center">
                                <span class="uppercase font-bold">Start/Stop:</span>
                                <code>Alt+Numpad 2</code>
                            </li>
                            <li class="flex justify-between items-center">
                                <span class="uppercase font-bold">Add Marker:</span>
                                <code>Alt+Numpad 3</code>
                            </li>
                        </ul>
                    </Accordion.Content>
                </Accordion.Item>
            </Accordion.Root>
        </div>

        <div class="col-span-1 md:col-span-2 brutalist-card h-fit">
            <h2 class="text-2xl font-bold mb-6 uppercase tracking-tight">Timeline View</h2>

            {#if activeSession}
                <Timeline session={activeSession} />
            {:else if sessionStore.lastSession}
                <Timeline session={sessionStore.lastSession} />
            {:else}
                <div class="text-center py-16 border-2 border-dashed border-black transform rotate-[0.5deg]">
                    <p class="text-xl uppercase font-bold mb-4">No Session Data</p>
                    <p class="text-lg">
                        Start tracking or view past sessions in History.
                    </p>
                </div>
            {/if}
        </div>
    </div>
</div>