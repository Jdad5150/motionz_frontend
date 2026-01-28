<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import * as Tabs from "$lib/components/ui/tabs";
    import Telemetry from "$lib/components/Telemetry.svelte";
    import TrajectoryPlot from "$lib/components/TrajectoryPlot.svelte";
    import { open } from "@tauri-apps/plugin-dialog";
    import { readTextFile, BaseDirectory } from "@tauri-apps/plugin-fs";
    import { appDataDir } from "@tauri-apps/api/path";
    import X from "@lucide/svelte/icons/x";

    type Mode = "LIVE" | "REVIEW";
    let mode = $state<Mode>("LIVE");

    type Trajectory = {
        id: string;
        name: string;
        x: number[];
        y: number[];
        z: number[];
    };

    let savedTrajectories = $state<Trajectory[]>([]);

    function removeTrajectory(trajectory: Trajectory) {
        savedTrajectories = savedTrajectories.filter(
            (t) => t.id !== trajectory.id,
        );
    }

    async function loadTrajectory() {
        // Get the app data directory path
        const appDataPath = await appDataDir();
        const trajectoriesPath = `${appDataPath}/trajectories`;

        const file = await open({
            multiple: false,
            filters: [{ name: "CSV", extensions: ["csv"] }],
            defaultPath: trajectoriesPath,
        });
        if (file) {
            const content = await readTextFile(file);
            const lines = content.trim().split("\n");
            const x: number[] = [];
            const y: number[] = [];
            const z: number[] = [];

            // Skip header, parse data (timestamp,x,y,z)
            for (let i = 1; i < lines.length; i++) {
                const [, xVal, yVal, zVal] = lines[i].split(",");
                x.push(parseFloat(xVal));
                y.push(parseFloat(yVal));
                z.push(parseFloat(zVal));
            }

            const filename = file.split(/[\\/]/).pop() || "trajectory";
            savedTrajectories.push({
                id: Date.now().toString(),
                name: filename.replace(".csv", ""),
                x,
                y,
                z,
            });
        }
    }
</script>

<div class="h-full flex flex-col">
    <!-- Mode Toggle Bar -->
    <div class="border-b bg-background px-4 py-2 flex items-center gap-2">
        <span class="text-sm font-semibold">Mode:</span>
        <Button
            variant={mode === "LIVE" ? "default" : "outline"}
            size="sm"
            onclick={() => (mode = "LIVE")}
        >
            LIVE
        </Button>
        <Button
            variant={mode === "REVIEW" ? "default" : "outline"}
            size="sm"
            onclick={() => (mode = "REVIEW")}
        >
            REVIEW
        </Button>
    </div>

    <!-- Content Area -->
    <div class="flex-1">
        {#if mode === "LIVE"}
            <!-- LIVE Mode: Single active trajectory -->
            <Telemetry />
        {:else}
            <!-- REVIEW Mode: Tabs for multiple saved trajectories -->
            <Tabs.Root
                value={savedTrajectories[0]?.id}
                class="h-full flex flex-col"
            >
                <Tabs.List class="border-b px-4">
                    {#each savedTrajectories as trajectory}
                        <Tabs.Trigger value={trajectory.id}>
                            {trajectory.name}
                            <Button
                                size="icon"
                                variant="ghost"
                                onclick={() => removeTrajectory(trajectory)}
                                class="rounded-full"
                                ><X class="text-teal-500" /></Button
                            >
                        </Tabs.Trigger>
                    {/each}
                    <Button
                        variant="ghost"
                        size="sm"
                        class="ml-2"
                        onclick={loadTrajectory}
                    >
                        + Load Trajectory
                    </Button>
                </Tabs.List>

                {#each savedTrajectories as trajectory}
                    <Tabs.Content value={trajectory.id} class="flex-1">
                        <TrajectoryPlot
                            x={trajectory.x}
                            y={trajectory.y}
                            z={trajectory.z}
                        />
                    </Tabs.Content>
                {/each}
            </Tabs.Root>
        {/if}
    </div>
</div>
