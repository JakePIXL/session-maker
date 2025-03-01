<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import Timeline from "$lib/components/Timeline.svelte";
	import { ScrollArea } from "bits-ui";

    let sessions: any[] = [];
    let selectedSession: any = null;
    let loading = true;

    async function loadSessions() {
        loading = true;
        try {
            const result = await invoke("get_sessions");
            sessions = result as any[];

            if (sessions.length > 0 && !selectedSession) {
                selectSession(sessions[0]);
            }
        } catch (error) {
            console.error("Failed to load sessions:", error);
        } finally {
            loading = false;
        }
    }

    function selectSession(session: any) {
        selectedSession = session;
    }

    function formatDate(dateString: string): string {
        const date = new Date(dateString);
        return date.toLocaleDateString([], {
            year: "numeric",
            month: "short",
            day: "numeric",
        });
    }

    function formatTime(dateString: string): string {
        const date = new Date(dateString);
        return date.toLocaleTimeString([], {
            hour: "2-digit",
            minute: "2-digit",
        });
    }

    function calculateDuration(start: string, end: string): string {
        const startTime = new Date(start).getTime();
        const endTime = new Date(end).getTime();
        const durationMs = endTime - startTime;

        const seconds = Math.floor((durationMs / 1000) % 60);
        const minutes = Math.floor((durationMs / (1000 * 60)) % 60);
        const hours = Math.floor(durationMs / (1000 * 60 * 60));

        if (hours > 0) {
            return `${hours}h ${minutes}m`;
        } else {
            return `${minutes}m ${seconds}s`;
        }
    }

    onMount(async () => {
        await loadSessions();
    });
</script>

<div class="container mx-auto">
    <header class="brutalist-header mt-8 mb-16">
        <h1 class="text-4xl font-bold uppercase tracking-tight">MarkerMoment History</h1>
        <p class="text-xl uppercase mt-2 tracking-tight">Past Recording Moments</p>
    </header>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
        <div class="col-span-1 brutalist-card h-fit">
            <div class="flex items-center justify-between mb-6">
                <h2 class="text-2xl font-bold uppercase tracking-tight">Sessions</h2>
                <span class="border-2 border-black px-2 py-1 font-bold">{sessions.length}</span>
            </div>

            {#if loading}
                <div class="h-64 flex justify-center items-center border-2 border-dashed border-black p-6 transform rotate-[-0.5deg]">
                    <div class="flex flex-col items-center gap-2">
                        <div class="w-16 h-4 bg-black"></div>
                        <div class="w-24 h-4 bg-black"></div>
                        <div class="w-8 h-4 bg-black"></div>
                    </div>
                </div>
            {:else if sessions.length === 0}
                <div class="p-6 text-center border-2 border-dashed border-black transform rotate-[0.5deg]">
                    <p class="font-bold uppercase">No past sessions found</p>
                </div>
            {:else}
                <ScrollArea.Root>         
	                <ScrollArea.Viewport class="h-full w-full max-h-[calc(100vh-400px)]">
                    {#each sessions as session, i}
                        <button
                            class="block w-full text-left border-3 border-black p-4 transition-colors relative mb-2"
                            class:bg-black={selectedSession && selectedSession.id === session.id}
                            class:text-white={selectedSession && selectedSession.id === session.id}
                            style={i % 2 === 0 ? "transform: rotate(-0.5deg);" : "transform: rotate(0.5deg);"}
                            on:click={() => selectSession(session)}
                        >
                            <div class="flex justify-between items-center">
                                <div>
                                    <div class="font-bold uppercase">
                                        {session.name ||
                                            formatDate(session.start_time)}
                                    </div>
                                    <div class="text-sm">
                                        {formatTime(session.start_time)} - {formatTime(
                                            session.end_time,
                                        )}
                                    </div>
                                </div>
                                <div class="flex flex-col items-end">
                                    <span class="font-bold">
                                        {calculateDuration(
                                            session.start_time,
                                            session.end_time,
                                        )}
                                    </span>
                                    <div class="border-2 border-current px-2 mt-1 text-sm">
                                        {session.markers?.length || 0} MARKER{session.markers?.length !== 1 ? 'S' : ''}
                                    </div>
                                </div>
                            </div>
                        </button>
                    {/each}
                    </ScrollArea.Viewport>
                    <ScrollArea.Scrollbar orientation="vertical" >
                        <ScrollArea.Thumb />
                    </ScrollArea.Scrollbar>
                </ScrollArea.Root>
            {/if}
        </div>

        <div class="col-span-1 md:col-span-2 brutalist-card h-fit">
            {#if selectedSession}
                <h2 class="text-2xl font-bold mb-6 uppercase tracking-tight">
                    {selectedSession.name ||
                        formatDate(selectedSession.start_time)}
                </h2>

                <Timeline session={selectedSession} />
            {:else}
                <div class="flex flex-col items-center justify-center h-64 border-2 border-dashed border-black p-8 transform rotate-[0.5deg]">
                    <p class="mb-4 text-xl uppercase font-bold">Select a session to view details</p>
                    {#if sessions.length === 0 && !loading}
                        <p class="text-md">
                            No sessions available. Start tracking to create new sessions.
                        </p>
                    {/if}
                </div>
            {/if}
        </div>
    </div>
</div>