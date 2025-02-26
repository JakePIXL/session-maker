<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    import Timeline from "$lib/components/Timeline.svelte";

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

<div class="container mx-auto p-4">
    <h1 class="text-2xl font-bold text-gray-800 mb-6">Session History</h1>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div class="col-span-1 bg-white rounded-lg shadow overflow-hidden">
            <div class="border-b px-4 py-3 bg-gray-50">
                <h2 class="font-medium text-gray-700">Past Sessions</h2>
            </div>

            {#if loading}
                <div class="flex justify-center items-center p-6">
                    <div
                        class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"
                    ></div>
                </div>
            {:else if sessions.length === 0}
                <div class="p-6 text-center text-gray-500">
                    <p>No past sessions found.</p>
                </div>
            {:else}
                <div class="overflow-y-auto max-h-[calc(100vh-240px)]">
                    {#each sessions as session}
                        <button
                            class="border-b px-4 py-3 cursor-pointer hover:bg-gray-50 transition-colors"
                            class:bg-blue-50={selectedSession &&
                                selectedSession.id === session.id}
                            on:click={() => selectSession(session)}
                        >
                            <div class="flex justify-between items-center">
                                <div>
                                    <div class="font-medium text-gray-800">
                                        {session.name ||
                                            formatDate(session.start_time)}
                                    </div>
                                    <div class="text-sm text-gray-600">
                                        {formatTime(session.start_time)} - {formatTime(
                                            session.end_time,
                                        )}
                                    </div>
                                </div>
                                <div class="flex flex-col items-end">
                                    <span
                                        class="text-sm font-medium text-gray-700"
                                    >
                                        {calculateDuration(
                                            session.start_time,
                                            session.end_time,
                                        )}
                                    </span>
                                    <span class="text-xs text-gray-500">
                                        {session.markers?.length || 0} markers
                                    </span>
                                </div>
                            </div>
                        </button>
                    {/each}
                </div>
            {/if}
        </div>

        <div class="col-span-1 md:col-span-2 bg-white rounded-lg shadow p-6">
            {#if selectedSession}
                <h2 class="text-xl font-semibold mb-4">
                    {selectedSession.name ||
                        formatDate(selectedSession.start_time)}
                </h2>

                <Timeline session={selectedSession} />
            {:else}
                <div
                    class="flex flex-col items-center justify-center h-64 text-gray-500"
                >
                    <p class="mb-2">Select a session to view details</p>
                    {#if sessions.length === 0 && !loading}
                        <p class="text-sm">
                            No sessions available. Start tracking to create new
                            sessions.
                        </p>
                    {/if}
                </div>
            {/if}
        </div>
    </div>
</div>
