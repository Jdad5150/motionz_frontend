<script lang="ts">
  import * as Dialog from "$lib/components/ui/dialog";
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";

  let { open = $bindable(false) } = $props();

  const DEFAULT_SPEEDS = { x: 100, y: 100, z: 100 };
  
  let serverUrl = $state("ws://localhost:8080");
  let apiKey = $state("");
  let speedX = $state(DEFAULT_SPEEDS.x);
  let speedY = $state(DEFAULT_SPEEDS.y);
  let speedZ = $state(DEFAULT_SPEEDS.z);

  function resetToDefaults() {
    speedX = DEFAULT_SPEEDS.x;
    speedY = DEFAULT_SPEEDS.y;
    speedZ = DEFAULT_SPEEDS.z;
  }

  function save() {
    // TODO: Save to Tauri store
    console.log('Saving settings:', { serverUrl, apiKey, speedX, speedY, speedZ });
    open = false;
  }

  function cancel() {
    // TODO: Reload from store
    open = false;
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Content class="sm:max-w-[425px]">
    <Dialog.Header>
      <Dialog.Title>Settings</Dialog.Title>
      <Dialog.Description>
        Configure your robot connection and default speeds.
      </Dialog.Description>
    </Dialog.Header>
    <div class="space-y-4 py-4">
      <!-- Server URL -->
      <div class="space-y-2">
        <Label for="serverUrl">Server URL</Label>
        <Input id="serverUrl" bind:value={serverUrl} placeholder="ws://localhost:8080" />
      </div>

      <!-- API Key -->
      <div class="space-y-2">
        <Label for="apiKey">API Key (optional)</Label>
        <Input id="apiKey" type="password" bind:value={apiKey} placeholder="Enter API key" />
      </div>

      <!-- Default Speeds -->
      <div class="space-y-2">
        <div class="flex items-center justify-between">
          <Label>Default Speeds (mm/s)</Label>
          <Button variant="ghost" size="sm" onclick={resetToDefaults}>
            Reset to Default
          </Button>
        </div>
        <div class="grid grid-cols-3 gap-2">
          <div>
            <Label for="speedX" class="text-xs">X Speed</Label>
            <Input id="speedX" type="number" bind:value={speedX} />
          </div>
          <div>
            <Label for="speedY" class="text-xs">Y Speed</Label>
            <Input id="speedY" type="number" bind:value={speedY} />
          </div>
          <div>
            <Label for="speedZ" class="text-xs">Z Speed</Label>
            <Input id="speedZ" type="number" bind:value={speedZ} />
          </div>
        </div>
      </div>
    </div>
    <Dialog.Footer>
      <Button variant="outline" onclick={cancel}>Cancel</Button>
      <Button onclick={save}>Save</Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
