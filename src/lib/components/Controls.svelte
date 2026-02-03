<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import { Input } from "$lib/components/ui/input";
    import { Slider } from "$lib/components/ui/slider";
    import { invoke } from "@tauri-apps/api/core";
    import { connectionStore } from "$lib/stores/connection.svelte";
    import Home from "@lucide/svelte/icons/home";
    import OctagonX from "@lucide/svelte/icons/octagon-x";
    import Play from "@lucide/svelte/icons/play";

    let x = $state(0);
    let y = $state(0);
    let z = $state(0);

    let speedX = $state([100]);
    let speedY = $state([100]);
    let speedZ = $state([100]);

    let sending = $state(false);

    async function go() {
        if (sending) return;

        sending = true;
        try {
            const result = await invoke<string>("send_go_command", {
                x,
                y,
                z,
            });
            console.log("GO Success:", result);
        } catch (error) {
            console.error("GO Error:", error);
        } finally {
            sending = false;
        }
    }

    function home() {
        console.log("HOME");
    }

    function stop() {
        console.log("STOP");
    }
</script>

<div class="flex flex-col h-full p-4 gap-6">
    <!-- Connection Status -->
    <button
        onclick={() =>
            connectionStore.connected
                ? connectionStore.disconnect()
                : connectionStore.connect()}
        disabled={connectionStore.connecting}
        class="flex items-center gap-3 p-3 rounded-lg border bg-card hover:bg-accent/50 transition-colors disabled:opacity-50"
    >
        <div class="relative">
            <div
                class={`w-3 h-3 rounded-full ${connectionStore.connected ? "bg-green-500" : "bg-muted-foreground"}`}
            ></div>
            {#if connectionStore.connected}
                <div
                    class="absolute inset-0 w-3 h-3 rounded-full bg-green-500 animate-ping opacity-75"
                ></div>
            {/if}
        </div>
        <span class="text-sm font-medium">
            {connectionStore.connecting
                ? "Connecting..."
                : connectionStore.connected
                  ? "Connected (click to disconnect)"
                  : "Disconnected (click to connect)"}
        </span>
    </button>

    <!-- Position -->
    <div class="space-y-3">
        <h3
            class="text-xs font-semibold uppercase tracking-wider text-muted-foreground"
        >
            Position
        </h3>
        <div class="grid grid-cols-3 gap-2">
            <div class="space-y-1">
                <div
                    class="text-xs font-medium text-muted-foreground text-center"
                >
                    X
                </div>
                <Input
                    type="number"
                    bind:value={x}
                    class="h-9 text-center font-mono"
                />
            </div>
            <div class="space-y-1">
                <div
                    class="text-xs font-medium text-muted-foreground text-center"
                >
                    Y
                </div>
                <Input
                    type="number"
                    bind:value={y}
                    class="h-9 text-center font-mono"
                />
            </div>
            <div class="space-y-1">
                <div
                    class="text-xs font-medium text-muted-foreground text-center"
                >
                    Z
                </div>
                <Input
                    type="number"
                    bind:value={z}
                    class="h-9 text-center font-mono"
                />
            </div>
        </div>
    </div>

    <!-- Speed -->
    <div class="space-y-4">
        <h3
            class="text-xs font-semibold uppercase tracking-wider text-muted-foreground"
        >
            Speed (mm/s)
        </h3>
        <div class="space-y-3">
            <div class="flex items-center gap-3">
                <span class="text-xs font-medium w-4">X</span>
                <Slider
                    bind:value={speedX}
                    max={500}
                    step={10}
                    class="flex-1"
                />
                <span class="text-xs font-mono w-8 text-right">{speedX[0]}</span
                >
            </div>
            <div class="flex items-center gap-3">
                <span class="text-xs font-medium w-4">Y</span>
                <Slider
                    bind:value={speedY}
                    max={500}
                    step={10}
                    class="flex-1"
                />
                <span class="text-xs font-mono w-8 text-right">{speedY[0]}</span
                >
            </div>
            <div class="flex items-center gap-3">
                <span class="text-xs font-medium w-4">Z</span>
                <Slider
                    bind:value={speedZ}
                    max={500}
                    step={10}
                    class="flex-1"
                />
                <span class="text-xs font-mono w-8 text-right">{speedZ[0]}</span
                >
            </div>
        </div>
    </div>

    <!-- Action Buttons -->
    <div class="flex-1 flex flex-col justify-end gap-3">
        <div class="flex gap-2">
            <Button onclick={home} variant="outline" class="flex-1 gap-2">
                <Home class="w-4 h-4" />
                Home
            </Button>
            <Button onclick={stop} variant="destructive" class="flex-1 gap-2">
                <OctagonX class="w-4 h-4" />
                Stop
            </Button>
        </div>
        <Button
            onclick={go}
            size="lg"
            class="w-full gap-2 h-12 text-base"
            disabled={sending}
        >
            <Play class="w-5 h-5" />
            {sending ? "Sending..." : "Go"}
        </Button>
    </div>
</div>
