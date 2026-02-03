<script lang="ts">
    import { Badge } from "$lib/components/ui/badge";
    import { Button } from "$lib/components/ui/button";
    import TrajectoryPlot from "$lib/components/TrajectoryPlot.svelte";
    import { connectionStore } from "$lib/stores/connection.svelte";
    import Circle from "@lucide/svelte/icons/circle";
    import CircleArrowRight from "@lucide/svelte/icons/circle-arrow-right";
    import CircleAlert from "@lucide/svelte/icons/circle-alert";
    import Trash2 from "@lucide/svelte/icons/trash-2";

    let plotComponent: TrajectoryPlot;

    // Track last known position to detect changes
    let lastPosition: { x: number; y: number; z: number } | null = null;

    // Live data from WebSocket via store
    let currentX = $derived(connectionStore.robotStatus?.position.x ?? 0);
    let currentY = $derived(connectionStore.robotStatus?.position.y ?? 0);
    let currentZ = $derived(connectionStore.robotStatus?.position.z ?? 0);

    let xAxisStatus = $derived(connectionStore.robotStatus?.x_status ?? "idle");
    let yAxisStatus = $derived(connectionStore.robotStatus?.y_status ?? "idle");
    let zAxisStatus = $derived(connectionStore.robotStatus?.z_status ?? "idle");

    let systemState = $derived(connectionStore.robotStatus?.status ?? "idle");
    let systemStatusStyle = $derived(getSystemStatusStyle(systemState));

    // Status to color/icon mapping
    function getStatusStyle(status: string): {
        bg: string;
        text: string;
        border: string;
    } {
        const s = status.toLowerCase();
        if (s === "moving") {
            return {
                bg: "bg-blue-500/20",
                text: "text-blue-600 dark:text-blue-400",
                border: "border-blue-500/50",
            };
        } else if (s === "error" || s === "fault") {
            return {
                bg: "bg-red-500/20",
                text: "text-red-600 dark:text-red-400",
                border: "border-red-500/50",
            };
        } else {
            // idle, ready, etc.
            return {
                bg: "bg-green-500/20",
                text: "text-green-600 dark:text-green-400",
                border: "border-green-500/50",
            };
        }
    }

    function getSystemStatusStyle(status: string): {
        bg: string;
        text: string;
    } {
        const s = status.toLowerCase();
        if (s === "moving") {
            return { bg: "bg-blue-500", text: "text-white" };
        } else if (s === "error" || s === "fault") {
            return { bg: "bg-red-500", text: "text-white" };
        } else {
            return { bg: "bg-green-500", text: "text-white" };
        }
    }

    // Track position changes and update plot only when position actually changes
    $effect(() => {
        if (connectionStore.robotStatus) {
            const { x, y, z } = connectionStore.robotStatus.position;

            // Only update if position has actually changed
            if (
                lastPosition === null ||
                x !== lastPosition.x ||
                y !== lastPosition.y ||
                z !== lastPosition.z
            ) {
                lastPosition = { x, y, z };
                plotComponent?.updateTrajectory(x, y, z);
            }
        }
    });

    function clearPlot() {
        plotComponent?.clearTrajectory();
        lastPosition = null;
    }
</script>

<div class="relative h-full">
    <!-- Telemetry Bar -->
    <div class="absolute bottom-0 left-0 right-0 z-10">
        <div
            class={`backdrop-blur-sm border-2 rounded-t-lg mx-2 ${
                connectionStore.connected
                    ? "bg-background/95 border-orange-500"
                    : "bg-background/95 border-muted-foreground/30"
            }`}
        >
            <div class="flex items-center h-12 px-1">
                <!-- Axis Cells -->
                {#each [{ label: "X", value: currentX, status: xAxisStatus }, { label: "Y", value: currentY, status: yAxisStatus }, { label: "Z", value: currentZ, status: zAxisStatus }] as axis}
                    {@const style = getStatusStyle(axis.status)}
                    <div
                        class={`flex items-center gap-2 px-4 py-2 border-r border-border/50 ${style.bg}`}
                    >
                        <span
                            class="text-xs font-semibold text-muted-foreground"
                            >{axis.label}</span
                        >
                        <span
                            class={`font-mono text-sm font-medium ${style.text}`}
                        >
                            {axis.value.toFixed(2)}
                        </span>
                        {#if axis.status.toLowerCase() === "moving"}
                            <CircleArrowRight
                                class={`w-3.5 h-3.5 ${style.text}`}
                            />
                        {:else if axis.status.toLowerCase() === "error" || axis.status.toLowerCase() === "fault"}
                            <CircleAlert class={`w-3.5 h-3.5 ${style.text}`} />
                        {:else}
                            <Circle
                                class={`w-3.5 h-3.5 ${style.text} fill-current`}
                            />
                        {/if}
                    </div>
                {/each}

                <!-- Spacer -->
                <div class="flex-1"></div>

                <!-- System Status -->
                <div class="px-3">
                    <span
                        class={`inline-flex items-center px-2.5 py-1 rounded text-xs font-medium ${systemStatusStyle.bg} ${systemStatusStyle.text}`}
                    >
                        {systemState.charAt(0).toUpperCase() +
                            systemState.slice(1)}
                    </span>
                </div>
            </div>
        </div>
    </div>

    <!-- Plot Area -->
    <div class="h-full relative">
        <TrajectoryPlot bind:this={plotComponent} x={[]} y={[]} z={[]} />

        <!-- Clear Button -->
        <div class="absolute top-4 right-4">
            <Button onclick={clearPlot} variant="outline" class="gap-2">
                <Trash2 class="w-4 h-4" />
                Clear
            </Button>
        </div>

        <!-- Sandbox Mode Indicator -->
        <div class="absolute top-4 left-4">
            <Badge variant="outline" class="border-orange-500 text-orange-500">
                Sandbox Mode
            </Badge>
        </div>
    </div>
</div>
