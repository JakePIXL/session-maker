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
    <div class="flex flex-col md:flex-row justify-between space-y-4 md:space-y-0 mb-6">
        <div class="brutalist-card p-3 transform rotate-[-0.5deg]">
            <div class="text-sm font-bold">
                START: <span class="font-medium uppercase">{formatTime(session.start_time)}</span>
            </div>
            {#if session.end_time}
                <div class="text-sm font-bold mt-2">
                    END: <span class="font-medium uppercase">{formatTime(session.end_time)}</span>
                </div>
            {/if}
        </div>

        <div class="flex space-x-2">
            <button
                on:click={() => exportSession("JSON")}
                class="btn btn-primary text-sm py-1 px-3 transform rotate-[0.3deg]"
            >
                JSON
            </button>
            <button
                on:click={() => exportSession("CSV")}
                class="btn btn-primary text-sm py-1 px-3 transform rotate-[-0.3deg]"
            >
                CSV
            </button>
            <button
                on:click={() => exportSession("Markdown")}
                class="btn btn-primary text-sm py-1 px-3 transform rotate-[0.3deg]"
            >
                MD
            </button>
        </div>
    </div>

    <div
        class="timeline relative h-16 border-2 border-black overflow-hidden mb-8 bg-white"
        bind:this={timelineElement}
    >
        <!-- Elapsed time indicator for active sessions -->
        {#if !session.end_time}
            <div
                class="absolute top-0 left-0 h-full bg-black brutalist-pulse"
                style="width: {((new Date().getTime() -
                    new Date(session.start_time).getTime()) /
                    (30 * 60 * 1000)) *
                    100}%; opacity: 0.2;"
            ></div>
        {/if}

        <!-- Markers -->
        {#if session.markers && session.markers.length > 0}
            {#each session.markers as marker}
                <div
                    class="marker absolute top-0 w-3 h-full bg-black cursor-pointer transform -translate-x-1.5"
                    style="left: {getMarkerPosition(marker.timestamp)}%"
                    title="{marker.label} - {getRelativeTime(marker.timestamp)}"
                ></div>
            {/each}
        {/if}

        <!-- Time graduations -->
        {#each Array(5) as _, i}
            <div
                class="absolute top-0 w-[3px] h-full bg-black"
                style="left: {i * 25}%"
            ></div>
            <div
                class="absolute -bottom-6 text-xs font-bold"
                style="left: calc({i * 25}% - 10px)"
            >
                {Math.floor(i * (sessionDuration / (4 * 60 * 1000)))}m
            </div>
        {/each}
    </div>

    <div class="markers-list mt-12">
        <div class="flex items-center mb-4">
            <h3 class="text-xl font-bold uppercase tracking-tight">
                Markers
            </h3>
            <span class="ml-2 border-3 border-black px-2 font-bold">{session.markers?.length || 0}</span>
        </div>

        {#if session.markers && session.markers.length > 0}
            <div class="overflow-x-auto">
                <table class="w-full">
                    <thead>
                        <tr>
                            <th class="uppercase">Time</th>
                            <th class="uppercase">Relative</th>
                            <th class="uppercase">Label</th>
                            <th class="uppercase">Notes</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each session.markers as marker, i}
                            <tr class="hover:bg-gray-50" style={i % 2 === 0 ? "transform: rotate(-0.2deg);" : "transform: rotate(0.2deg);"}>
                                <td>{formatTime(marker.timestamp)}</td>
                                <td><code>{getRelativeTime(marker.timestamp)}</code></td>
                                <td class="font-bold uppercase">{marker.label}</td>
                                <td>{marker.notes || "-"}</td>
                            </tr>
                        {/each}
                    </tbody>
                </table>
            </div>
        {:else}
            <div class="border-2 border-dashed border-black p-6 text-center transform rotate-[0.5deg]">
                <p class="font-bold uppercase">
                    No markers added to this session
                </p>
            </div>
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
        top: -5px;
        left: 50%;
        transform: translateX(-50%);
        width: 10px;
        height: 10px;
        background-color: var(--color-accent);
        border: 2px solid black;
        z-index: 20;
    }
</style>