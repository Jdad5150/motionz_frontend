<script lang="ts">
  import * as Collapsible from "$lib/components/ui/collapsible";
  import { Badge } from "$lib/components/ui/badge";
  import { Button } from "$lib/components/ui/button";
  import ChevronDown from "@lucide/svelte/icons/chevron-down";
  import TrajectoryPlot from "$lib/components/TrajectoryPlot.svelte";
  import { writeTextFile, BaseDirectory, mkdir, exists } from '@tauri-apps/plugin-fs';

  let plotComponent: TrajectoryPlot;
  let demoInterval: number | null = null;
  let demoRunning = $state(false);
  let t = 0;
  
  // Trajectory data storage
  let trajectoryData: {x: number, y: number, z: number, timestamp: number}[] = [];

  // Mock data - will be replaced with WebSocket data
  let currentX = $state(0);
  let currentY = $state(0);
  let currentZ = $state(0);
  
  let xAxisStatus = $state("Ready");
  let yAxisStatus = $state("Ready");
  let zAxisStatus = $state("Ready");
  
  let systemState = $state("Idle");
  let connectionStatus = $state("Disconnected");
  let lastCommand = $state("GO to (100, 200, 300)");

  let telemetryOpen = $state(false);
  
  let isConnected = $derived(connectionStatus === "Connected");

  async function saveTrajectoryToCSV() {
    if (trajectoryData.length === 0) return;
    
    // Generate CSV content
    let csv = "timestamp,x,y,z\n";
    trajectoryData.forEach(point => {
      csv += `${point.timestamp},${point.x.toFixed(2)},${point.y.toFixed(2)},${point.z.toFixed(2)}\n`;
    });
    
    // Ensure trajectories directory exists
    const dirPath = 'trajectories';
    const dirExists = await exists(dirPath, { baseDir: BaseDirectory.AppData });
    if (!dirExists) {
      await mkdir(dirPath, { baseDir: BaseDirectory.AppData, recursive: true });
    }
    
    // Save to app data directory
    const filename = `trajectories/trajectory_${new Date().toISOString().replace(/[:.]/g, '-')}.csv`;
    
    try {
      await writeTextFile(filename, csv, { baseDir: BaseDirectory.AppData });
      console.log(`Trajectory saved to AppData/${filename}`);
    } catch (error) {
      console.error('Failed to save trajectory:', error);
    }
  }

  async function toggleDemo() {
    if (demoRunning) {
      // Stop demo
      if (demoInterval) clearInterval(demoInterval);
      demoInterval = null;
      demoRunning = false;
      connectionStatus = "Disconnected";
      systemState = "Idle";
      
      // Save trajectory data
      await saveTrajectoryToCSV();
    } else {
      // Start demo
      connectionStatus = "Connected";
      systemState = "Moving";
      demoRunning = true;
      t = 0;
      trajectoryData = [];
      
      // Clear existing trajectory
      plotComponent?.clearTrajectory();
      
      // Simulate robot movement
      demoInterval = setInterval(() => {
        t += 0.1;
        
        // Generate spiral trajectory
        currentX = 50 + 30 * Math.cos(t) * (1 + t * 0.1);
        currentY = 50 + 30 * Math.sin(t) * (1 + t * 0.1);
        currentZ = 10 + t * 2;
        
        // Store data point
        trajectoryData.push({
          x: currentX,
          y: currentY,
          z: currentZ,
          timestamp: Date.now()
        });
        
        // Update plot
        plotComponent?.updateTrajectory(currentX, currentY, currentZ);
        
        // Stop after ~10 seconds
        if (t > 10) {
          toggleDemo();
        }
      }, 100);
    }
  }
</script>

<div class="relative h-full">
  <!-- Slim Telemetry Banner -->
  <div class="absolute bottom-0 left-0 right-0 z-10">
    <Collapsible.Root bind:open={telemetryOpen}>
      <div class={`backdrop-blur border-t ${isConnected ? 'bg-background/95' : 'bg-destructive/95'}`}>
        <!-- Expanded View -->
        <Collapsible.Content>
          <div class="px-4 py-3 space-y-3 text-sm border-b">
            <div class="flex gap-8">
              <div>
                <span class="text-muted-foreground">Axis Status:</span>
                <div class="flex gap-2 mt-1">
                  <Badge variant="outline">X: {xAxisStatus}</Badge>
                  <Badge variant="outline">Y: {yAxisStatus}</Badge>
                  <Badge variant="outline">Z: {zAxisStatus}</Badge>
                </div>
              </div>
              <div>
                <span class="text-muted-foreground">Last Command:</span>
                <div class="font-mono mt-1">{lastCommand}</div>
              </div>
            </div>
          </div>
        </Collapsible.Content>
        
        <!-- Collapsed View -->
        <button 
          onclick={() => telemetryOpen = !telemetryOpen}
          class={`w-full px-4 py-2 flex items-center justify-between transition-colors ${isConnected ? 'hover:bg-accent/50' : 'hover:bg-destructive/80'}`}
        >
          <div class={`flex items-center gap-6 text-sm ${isConnected ? '' : 'text-destructive-foreground'}`}>
            <span class="font-semibold">Telemetry</span>
            <span class="text-muted-foreground">Pos:</span>
            <span class="font-mono">X: {currentX.toFixed(2)}</span>
            <span class="font-mono">Y: {currentY.toFixed(2)}</span>
            <span class="font-mono">Z: {currentZ.toFixed(2)}</span>
            <span class="text-muted-foreground">|</span>
            <Badge variant="default">{connectionStatus}</Badge>
            <Badge variant="secondary">{systemState}</Badge>
          </div>
          <ChevronDown class={`h-4 w-4 transition-transform duration-200 ${telemetryOpen ? 'rotate-180' : ''}`} />
        </button>
      </div>
    </Collapsible.Root>
  </div>

  <!-- Plot Area -->
  <div class="h-full relative">
    <TrajectoryPlot bind:this={plotComponent} />
    
    <!-- Demo Button -->
    <div class="absolute top-4 right-4">
      <Button onclick={toggleDemo} variant={demoRunning ? "destructive" : "default"}>
        {demoRunning ? "Stop Demo" : "Start Demo"}
      </Button>
    </div>
  </div>
</div>
