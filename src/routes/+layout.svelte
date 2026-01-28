<script lang="ts">
	import './layout.css';
	import Titlebar from '$lib/components/Titlebar.svelte';
	import { ModeWatcher } from "mode-watcher";

	const { children } = $props();
</script>

<ModeWatcher/>
<Titlebar />
{@render children()}
