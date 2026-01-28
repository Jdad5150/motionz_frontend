<script lang="ts">
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { onMount } from 'svelte';
  import Minus from '@lucide/svelte/icons/minus';
  import Expand from '@lucide/svelte/icons/expand';
  import Shrink from '@lucide/svelte/icons/shrink';
  import X from '@lucide/svelte/icons/x';
  import SunIcon from "@lucide/svelte/icons/sun";
  import MoonIcon from "@lucide/svelte/icons/moon";
  import Menu from "@lucide/svelte/icons/menu";
  import Box from "@lucide/svelte/icons/box";
 
  import { toggleMode } from "mode-watcher";
  import { Button } from "$lib/components/ui/button/index.js";
  import * as Menubar from "$lib/components/ui/menubar";
  import SettingsDialog from "$lib/components/SettingsDialog.svelte";

  const appWindow = getCurrentWindow();
  let isMaximized = $state(false);
  let settingsOpen = $state(false);

  onMount(async () => {
    isMaximized = await appWindow.isMaximized();
    appWindow.onResized(async () => {
      isMaximized = await appWindow.isMaximized();
    });
  });

  async function minimize() {
    await appWindow.minimize();
  }

  async function toggleMaximize() {
    await appWindow.toggleMaximize();
  }

  async function close() {
    await appWindow.close();
  }
</script>

<div class="titlebar">
  <div class="titlebar-left">
    <Box class="h-4 w-4 ml-3" />
    <Menubar.Root class="border-none bg-transparent h-7">
      <Menubar.Menu>
        <Menubar.Trigger class="h-7 px-2">File</Menubar.Trigger>
        <Menubar.Content>
          <Menubar.Item onclick={() => settingsOpen = true}>
            Settings
          </Menubar.Item>
        </Menubar.Content>
      </Menubar.Menu>
    </Menubar.Root>
  </div>
  <div class="titlebar-title" data-tauri-drag-region>
    Motionz
  </div>
  <div class="titlebar-buttons">
    <Button onclick={toggleMode} variant="ghost" size="icon" class="h-8 w-8">
      <SunIcon
        class="h-[1.2rem] w-[1.2rem] scale-100 rotate-0 !transition-all dark:scale-0 dark:-rotate-90"
      />
      <MoonIcon
        class="absolute h-[1.2rem] w-[1.2rem] scale-0 rotate-90 !transition-all dark:scale-100 dark:rotate-0"
      />
      <span class="sr-only">Toggle theme</span>
    </Button>
    <button class="titlebar-button" onclick={minimize}>
      <Minus size={16} />
    </button>
    <button class="titlebar-button" onclick={toggleMaximize}>
      {#if isMaximized}
        <Shrink size={16} />
      {:else}
        <Expand size={16} />
      {/if}
    </button>
    <button class="titlebar-button close" onclick={close}>
      <X size={16} />
    </button>
  </div>
</div>

<SettingsDialog bind:open={settingsOpen} />

<style>
  .titlebar {
    height: 32px;
    background: hsl(var(--background));
    border-bottom: 1px solid hsl(var(--border));
    display: flex;
    justify-content: space-between;
    align-items: center;
    user-select: none;
  }

  .titlebar-left {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .titlebar-title {
    position: absolute;
    left: 50%;
    transform: translateX(-50%);
    font-size: 13px;
    font-weight: 500;
    color: hsl(var(--foreground));
  }

  .titlebar-buttons {
    display: flex;
    height: 100%;
  }

  .titlebar-button {
    width: 46px;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: hsl(var(--foreground));
    cursor: pointer;
    transition: background-color 0.15s;
    pointer-events: auto;
  }

  .titlebar-button:hover {
    background: hsl(var(--accent));
  }

  .titlebar-button.close:hover {
    background: hsl(var(--destructive));
    color: hsl(var(--destructive-foreground));
  }
</style>
