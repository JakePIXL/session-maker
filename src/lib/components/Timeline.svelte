<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
	import { ScrollArea } from "bits-ui";

    let { session } = $props();

    let sessionDuration = $state(0);
    let timelineWidth = $state(0);
    let timelineElement: HTMLElement;

    $effect(() => {
        if (session) {
            calculateSessionDuration();
        }
    });
    
    // Update the UI when session duration changes
    $effect(() => {
        if (sessionDuration && !session.end_time) {
            updateTimelineWidth();
        }
    });

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

    let timer: number;

    onMount(() => {
        updateTimelineWidth();
        window.addEventListener("resize", updateTimelineWidth);
        
        // Set up an interval to update in real-time
        timer = setInterval(() => {
            if (!session.end_time) {
                calculateSessionDuration();
            }
        }, 1000);

        return () => {
            window.removeEventListener("resize", updateTimelineWidth);
            clearInterval(timer);
        };
    });
</script>

<div class="timeline-container">
    <div class="flex flex-col md:flex-row justify-between space-y-4 md:space-y-0 mb-6">
        <div class="border-2 border-black p-4 bg-white rounded transform rotate-[-0.5deg] shadow-sm">
            <div class="text-sm font-bold">
                START: <span class="font-medium uppercase">{formatTime(session.start_time)}</span>
            </div>
            {#if session.end_time}
                <div class="text-sm font-bold mt-2">
                    END: <span class="font-medium uppercase">{formatTime(session.end_time)}</span>
                </div>
            {:else}
                <div class="text-sm font-bold mt-2">
                    DURATION: <span class="font-medium uppercase bg-yellow-100 px-2 py-1 rounded">{formatDuration(sessionDuration)}</span>
                </div>
            {/if}
        </div>

        <div class="flex space-x-2">
            <button
                onclick={() => exportSession("JSON")}
                class="border-2 border-black bg-white hover:bg-gray-50 font-bold text-sm py-2 px-4 transform rotate-[0.3deg] rounded shadow-sm"
            >
                JSON
            </button>
            <button
                onclick={() => exportSession("CSV")}
                class="border-2 border-black bg-white hover:bg-gray-50 font-bold text-sm py-2 px-4 transform rotate-[-0.3deg] rounded shadow-sm"
            >
                CSV
            </button>
            <button
                onclick={() => exportSession("Markdown")}
                class="border-2 border-black bg-white hover:bg-gray-50 font-bold text-sm py-2 px-4 transform rotate-[0.3deg] rounded shadow-sm"
            >
                MD
            </button>
        </div>
    </div>

    <div
        class="timeline relative h-16 border-2 border-black overflow-hidden mb-8 bg-white rounded shadow-sm"
        bind:this={timelineElement}
    >
        <!-- Elapsed time indicator for active sessions -->
        {#if !session.end_time}
            <div
                class="absolute top-0 left-0 h-full bg-blue-500"
                style="width: {Math.min((sessionDuration / (30 * 60 * 1000)) * 100, 100)}%; opacity: 0.2; animation: pulse 2s infinite;"
            ></div>
        {/if}

        <!-- Markers -->
        {#if session.markers && session.markers.length > 0}
            {#each session.markers as marker}
                <div
                    class="marker absolute top-0 w-2 h-full bg-black cursor-pointer transform -translate-x-1"
                    style="left: {getMarkerPosition(marker.timestamp)}%"
                    title="{marker.label} - {getRelativeTime(marker.timestamp)}"
                ></div>
            {/each}
        {/if}

        <!-- Time graduations -->
        {#each Array(5) as _, i}
            <div
                class="absolute top-0 w-[2px] h-full bg-black"
                style="left: {i * 25}%"
            ></div>
            <div
                class="absolute -bottom-6 text-xs font-bold bg-white px-1 rounded"
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
            <span class="ml-2 border-2 border-black px-2 font-bold rounded">{session.markers?.length || 0}</span>
        </div>

        {#if session.markers && session.markers.length > 0}
            <ScrollArea.Root class="border-2 border-black rounded">
	            <ScrollArea.Viewport class="h-full w-full max-h-[300px]">
                <table class="w-full">
                    <thead class="bg-gray-100 sticky top-0">
                        <tr>
                            <th class="uppercase text-left p-2 font-bold">Time</th>
                            <th class="uppercase text-left p-2 font-bold">Relative</th>
                            <th class="uppercase text-left p-2 font-bold">Label</th>
                            <th class="uppercase text-left p-2 font-bold">Notes</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each session.markers as marker, i}
                            <tr class={i % 2 === 0 ? "bg-white" : "bg-gray-50"} style={i % 2 === 0 ? "transform: rotate(-0.2deg);" : "transform: rotate(0.2deg);"}>
                                <td class="p-2">{formatTime(marker.timestamp)}</td>
                                <td class="p-2"><code class="bg-gray-100 px-2 py-1 rounded text-sm">{getRelativeTime(marker.timestamp)}</code></td>
                                <td class="p-2 font-bold uppercase">{marker.label}</td>
                                <td class="p-2">{marker.notes || "-"}</td>
                            </tr>
                        {/each}
                    </tbody>
                </table>
                </ScrollArea.Viewport>          
                <ScrollArea.Scrollbar orientation="vertical" class="p-0.5">
                    <ScrollArea.Thumb class="bg-black/50 rounded" />
                </ScrollArea.Scrollbar>
            </ScrollArea.Root>
        {:else}
            <div class="border-2 border-dashed border-black p-6 text-center transform rotate-[0.5deg] rounded">
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
        background-color: #ffd600;
        border: 2px solid black;
        border-radius: 50%;
        z-index: 20;
    }

    @keyframes pulse {
        0% {
            opacity: 0.1;
        }
        50% {
            opacity: 0.3;
        }
        100% {
            opacity: 0.1;
        }
    }
</style>