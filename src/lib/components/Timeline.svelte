<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    export let session: any;

    let sessionDuration = 0;
    let timelineWidth = 0;
    let timelineElement: HTMLElement;

    $: if (session) {
        calculateSessionDuration();
    }

    function calculateSessionDuration() {
        const startTime = new Date(session.start_time).getTime();
        const endTime = session.end_time
            ? new Date(session.end_time).getTime()
            : new Date().getTime();

        sessionDuration = endTime - startTime;
    }

    function getMarkerPosition(markerTime: string): number {
        const markerDate = new Date(markerTime).getTime();
        const startTime = new Date(session.start_time).getTime();
        const endTime = session.end_time
            ? new Date(session.end_time).getTime()
            : new Date().getTime();

        const position =
            ((markerDate - startTime) / (endTime - startTime)) * 100;
        return Math.min(Math.max(position, 0), 100);
    }

    function formatTime(dateString: string): string {
        const date = new Date(dateString);
        return date.toLocaleTimeString([], {
            hour: "2-digit",
            minute: "2-digit",
            second: "2-digit",
        });
    }

    function getRelativeTime(markerTime: string): string {
        const markerDate = new Date(markerTime).getTime();
        const startTime = new Date(session.start_time).getTime();
        const diffMs = markerDate - startTime;

        const seconds = Math.floor((diffMs / 1000) % 60);
        const minutes = Math.floor((diffMs / (1000 * 60)) % 60);
        const hours = Math.floor(diffMs / (1000 * 60 * 60));

        if (hours > 0) {
            return `${hours}h ${minutes}m ${seconds}s`;
        } else if (minutes > 0) {
            return `${minutes}m ${seconds}s`;
        } else {
            return `${seconds}s`;
        }
    }

    async function exportSession(format: "JSON" | "CSV" | "Markdown") {
        try {
            await invoke("export_session", {
                id: session.id,
                format,
            });
        } catch (error) {
            console.error("Export failed:", error);
        }
    }

    function updateTimelineWidth() {
        if (timelineElement) {
            timelineWidth = timelineElement.offsetWidth;
        }
    }

    onMount(() => {
        updateTimelineWidth();
        window.addEventListener("resize", updateTimelineWidth);

        return () => {
            window.removeEventListener("resize", updateTimelineWidth);
        };
    });
</script>

<div class="timeline-container">
    <div class="flex justify-between mb-4">
        <div>
            <div class="text-sm text-gray-600">
                Start: <span class="font-medium"
                    >{formatTime(session.start_time)}</span
                >
            </div>
            {#if session.end_time}
                <div class="text-sm text-gray-600">
                    End: <span class="font-medium"
                        >{formatTime(session.end_time)}</span
                    >
                </div>
            {/if}
        </div>

        <div class="flex space-x-2">
            <button
                on:click={() => exportSession("JSON")}
                class="bg-indigo-500 hover:bg-indigo-600 text-white text-sm py-1 px-3 rounded focus:outline-none focus:shadow-outline"
            >
                Export JSON
            </button>
            <button
                on:click={() => exportSession("CSV")}
                class="bg-indigo-500 hover:bg-indigo-600 text-white text-sm py-1 px-3 rounded focus:outline-none focus:shadow-outline"
            >
                Export CSV
            </button>
            <button
                on:click={() => exportSession("Markdown")}
                class="bg-indigo-500 hover:bg-indigo-600 text-white text-sm py-1 px-3 rounded focus:outline-none focus:shadow-outline"
            >
                Export Markdown
            </button>
        </div>
    </div>

    <div
        class="timeline relative h-12 bg-gray-200 rounded-full overflow-hidden mb-4"
        bind:this={timelineElement}
    >
        <!-- Elapsed time indicator for active sessions -->
        {#if !session.end_time}
            <div
                class="absolute top-0 left-0 h-full bg-blue-300 transition-all duration-500"
                style="width: {((new Date().getTime() -
                    new Date(session.start_time).getTime()) /
                    (30 * 60 * 1000)) *
                    100}%"
            ></div>
        {/if}

        <!-- Markers -->
        {#if session.markers && session.markers.length > 0}
            {#each session.markers as marker}
                <div
                    class="marker absolute top-0 w-3 h-full bg-red-500 cursor-pointer transform -translate-x-1.5"
                    style="left: {getMarkerPosition(marker.timestamp)}%"
                    title="{marker.label} - {getRelativeTime(marker.timestamp)}"
                ></div>
            {/each}
        {/if}

        <!-- Time graduations -->
        {#each Array(5) as _, i}
            <div
                class="absolute top-0 w-px h-full bg-gray-400"
                style="left: {i * 25}%"
            ></div>
            <div
                class="absolute -bottom-6 text-xs text-gray-600"
                style="left: calc({i * 25}% - 10px)"
            >
                {Math.floor(i * (sessionDuration / (4 * 60 * 1000)))}m
            </div>
        {/each}
    </div>

    <div class="markers-list mt-8">
        <h3 class="text-lg font-semibold mb-2">
            Markers ({session.markers?.length || 0})
        </h3>

        {#if session.markers && session.markers.length > 0}
            <div class="bg-white border rounded overflow-hidden">
                <table class="min-w-full">
                    <thead>
                        <tr class="bg-gray-100 border-b">
                            <th
                                class="px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                >Time</th
                            >
                            <th
                                class="px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                >Relative</th
                            >
                            <th
                                class="px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                >Label</th
                            >
                            <th
                                class="px-4 py-2 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                                >Notes</th
                            >
                        </tr>
                    </thead>
                    <tbody>
                        {#each session.markers as marker}
                            <tr class="border-b hover:bg-gray-50">
                                <td
                                    class="px-4 py-2 whitespace-nowrap text-sm text-gray-900"
                                >
                                    {formatTime(marker.timestamp)}
                                </td>
                                <td
                                    class="px-4 py-2 whitespace-nowrap text-sm text-gray-600"
                                >
                                    {getRelativeTime(marker.timestamp)}
                                </td>
                                <td
                                    class="px-4 py-2 whitespace-nowrap text-sm text-gray-900"
                                >
                                    {marker.label}
                                </td>
                                <td class="px-4 py-2 text-sm text-gray-600">
                                    {marker.notes || "-"}
                                </td>
                            </tr>
                        {/each}
                    </tbody>
                </table>
            </div>
        {:else}
            <p class="text-gray-500 italic">
                No markers added to this session.
            </p>
        {/if}
    </div>
</div>

<style>
    .timeline {
        position: relative;
    }

    .marker {
        z-index: 10;
    }

    .marker:hover::after {
        content: "";
        position: absolute;
        top: -8px;
        left: 50%;
        transform: translateX(-50%);
        width: 16px;
        height: 16px;
        background-color: #ef4444;
        border-radius: 50%;
        z-index: 20;
    }
</style>
