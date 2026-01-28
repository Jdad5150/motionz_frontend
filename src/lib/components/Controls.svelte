<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Separator } from "$lib/components/ui/separator";

  let x = $state(0);
  let y = $state(0);
  let z = $state(0);

  let speedX = $state(100);
  let speedY = $state(100);
  let speedZ = $state(100);

  let connected = $state(false);

  function connect() {
    connected = !connected;
  }

  function go() {
    console.log('GO:', { x, y, z, speedX, speedY, speedZ });
  }

  function home() {
    console.log('HOME');
  }

  function stop() {
    console.log('STOP');
  }
</script>

<div class="flex flex-col h-full p-4 gap-4">
  <!-- Connection -->
  <div>
    <Label>Connection</Label>
    <Button onclick={connect} variant={connected ? "default" : "outline"} class="w-full mt-2">
      {connected ? "Connected" : "Connect"}
    </Button>
  </div>

  <Separator />

  <!-- Position Control -->
  <div>
    <Label class="text-sm font-semibold">Position Control</Label>
    <div class="space-y-2 mt-2">
      <div>
        <Label for="x">X</Label>
        <Input id="x" type="number" bind:value={x} />
      </div>
      <div>
        <Label for="y">Y</Label>
        <Input id="y" type="number" bind:value={y} />
      </div>
      <div>
        <Label for="z">Z</Label>
        <Input id="z" type="number" bind:value={z} />
      </div>
    </div>
  </div>

  <Separator />

  <!-- Speed Control -->
  <div>
    <Label class="text-sm font-semibold">Speed (mm/s)</Label>
    <div class="space-y-2 mt-2">
      <div>
        <Label for="speedX">X Speed</Label>
        <Input id="speedX" type="number" bind:value={speedX} />
      </div>
      <div>
        <Label for="speedY">Y Speed</Label>
        <Input id="speedY" type="number" bind:value={speedY} />
      </div>
      <div>
        <Label for="speedZ">Z Speed</Label>
        <Input id="speedZ" type="number" bind:value={speedZ} />
      </div>
    </div>
  </div>

  <Separator />

  <!-- Action Buttons -->
  <div class="flex-1 flex flex-col justify-end gap-2">
    <div class="flex gap-2">
      <Button onclick={home} variant="outline" class="flex-1">HOME</Button>
      <Button onclick={stop} variant="destructive" class="flex-1">STOP</Button>
    </div>
    <Button onclick={go} size="lg" class="w-full">GO</Button>
  </div>
</div>