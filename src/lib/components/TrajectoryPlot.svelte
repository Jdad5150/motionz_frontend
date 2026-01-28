<script lang="ts">
  import { onMount } from 'svelte';
  // @ts-ignore - no types available for plotly.js-dist-min
  import Plotly from 'plotly.js-dist-min';

  interface Props {
    x?: number[];
    y?: number[];
    z?: number[];
  }

  let { x = [0, 10, 20, 30, 40, 50], y = [0, 15, 25, 20, 30, 45], z = [0, 5, 10, 15, 12, 20] }: Props = $props();

  let plotDiv: HTMLDivElement;
  let trajectoryX = $state<number[]>([]);
  let trajectoryY = $state<number[]>([]);
  let trajectoryZ = $state<number[]>([]);

  $effect(() => {
    trajectoryX = [...x];
    trajectoryY = [...y];
    trajectoryZ = [...z];
  });

  onMount(() => {
    const data = [{
      type: 'scatter3d',
      mode: 'lines+markers',
      x: trajectoryX,
      y: trajectoryY,
      z: trajectoryZ,
      line: {
        color: '#14b8a6',
        width: 4
      },
      marker: {
        size: 4,
        color: '#14b8a6',
      }
    }];

    const layout = {
      autosize: true,
      scene: {
        xaxis: { title: 'X (mm)' },
        yaxis: { title: 'Y (mm)' },
        zaxis: { title: 'Z (mm)' },
        camera: {
          eye: { x: 1.5, y: 1.5, z: 1.5 }
        }
      },
      margin: { l: 0, r: 0, t: 0, b: 0 },
      paper_bgcolor: 'transparent',
      plot_bgcolor: 'transparent',
    };

    const config = {
      responsive: true,
      displayModeBar: true,
      displaylogo: false,
    };

    Plotly.newPlot(plotDiv, data, layout, config);

    const resizeObserver = new ResizeObserver(() => {
      Plotly.Plots.resize(plotDiv);
    });
    resizeObserver.observe(plotDiv);

    return () => {
      resizeObserver.disconnect();
      Plotly.purge(plotDiv);
    };
  });

  // Function to update trajectory (call this when new data arrives)
  export function updateTrajectory(x: number, y: number, z: number) {
    trajectoryX.push(x);
    trajectoryY.push(y);
    trajectoryZ.push(z);

    Plotly.update(plotDiv, {
      x: [trajectoryX],
      y: [trajectoryY],
      z: [trajectoryZ]
    }, {}, [0]);
  }

  // Function to load complete trajectory data
  export function loadTrajectory(x: number[], y: number[], z: number[]) {
    trajectoryX = x;
    trajectoryY = y;
    trajectoryZ = z;

    Plotly.update(plotDiv, {
      x: [trajectoryX],
      y: [trajectoryY],
      z: [trajectoryZ]
    }, {}, [0]);
  }

  // Function to clear trajectory
  export function clearTrajectory() {
    trajectoryX = [];
    trajectoryY = [];
    trajectoryZ = [];

    Plotly.update(plotDiv, {
      x: [[]],
      y: [[]],
      z: [[]]
    }, {}, [0]);
  }
</script>

<div bind:this={plotDiv} class="w-full h-full"></div>
