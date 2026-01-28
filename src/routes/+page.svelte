<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import * as Resizable from "$lib/components/ui/resizable/index.js";
  import Controls from "$lib/components/Controls.svelte";
  import MainContent from "$lib/components/MainContent.svelte";
</script>

<div class="h-[calc(100vh-32px)] w-full">
  <Resizable.PaneGroup
    direction="horizontal"
    class="h-full w-full"
  >
    <Resizable.Pane defaultSize={25}>
      <Controls />
    </Resizable.Pane>
    <Resizable.Handle withHandle />
    <Resizable.Pane defaultSize={75}>
      <MainContent />
    </Resizable.Pane>
  </Resizable.PaneGroup>

</div>


<style>
</style>
