# Motionz Frontend

A modern desktop application for robot motion control and trajectory visualization built with Tauri, SvelteKit, and Plotly.js.

## Features

### LIVE Mode
- **Real-time Control**: X/Y/Z position and speed controls with HOME/STOP/GO commands
- **3D Trajectory Visualization**: Live plotting of robot movement paths using Plotly.js
- **Live Telemetry**: Collapsible banner showing current position, axis status, and connection state
- **Demo Mode**: Simulated spiral trajectory pattern for testing without hardware
- **Auto-save**: Trajectories automatically exported to CSV in app data directory

### REVIEW Mode
- **Multi-trajectory Analysis**: Load and compare multiple saved trajectories
- **Tabbed Interface**: Switch between different trajectory datasets
- **3D Visualization**: Full interactive 3D plots of historical movement data
- **CSV Import**: Load trajectory data from any CSV file

### Additional Features
- **Custom Titlebar**: Frameless window with integrated menu bar and window controls
- **Theme Toggle**: Light/dark mode support
- **Settings Dialog**: Configure server URL, API key, and default speeds
- **Resizable Layout**: Adjustable control panel and visualization areas
- **Connection Management**: Visual indicators for robot connection status

## Tech Stack

- **Frontend**: SvelteKit 5 (with runes), TypeScript, Tailwind CSS
- **UI Components**: shadcn-svelte (new-york style)
- **Desktop Framework**: Tauri v2
- **Visualization**: Plotly.js
- **Icons**: Lucide Svelte

## Getting Started

### Prerequisites
- Node.js (v18+)
- Rust (latest stable)
- pnpm (recommended) or npm

### Installation

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Project Structure

```
src/
├── lib/
│   ├── components/
│   │   ├── Titlebar.svelte          # Custom window titlebar
│   │   ├── Controls.svelte          # Robot control panel
│   │   ├── MainContent.svelte       # Mode switcher (LIVE/REVIEW)
│   │   ├── Telemetry.svelte         # Live data display & demo mode
│   │   ├── TrajectoryPlot.svelte    # 3D Plotly visualization
│   │   ├── SettingsDialog.svelte    # Configuration dialog
│   │   └── ui/                      # shadcn-svelte components
│   └── utils.ts                     # Utility functions
├── routes/
│   ├── +layout.svelte               # Root layout with titlebar
│   ├── +page.svelte                 # Main page with resizable panes
│   └── layout.css                   # Theme configuration
└── src-tauri/
    ├── src/lib.rs                   # Tauri app initialization
    ├── capabilities/default.json    # Permission configuration
    └── tauri.conf.json              # Tauri settings
```

## Data Format

Trajectories are saved as CSV files with the following format:

```csv
timestamp,x,y,z
1234567890,10.5,20.3,15.7
1234567891,10.6,20.4,15.8
...
```

**Storage Location**:
- **Windows**: `%APPDATA%\[app-identifier]\trajectories\`
- **macOS**: `~/Library/Application Support/[app-identifier]/trajectories/`
- **Linux**: `~/.local/share/[app-identifier]/trajectories/`

## Color Scheme

- **Primary**: Teal (`#14b8a6`)
- **Background**: Black/White (theme-dependent)
- **Accent**: Teal for active states and trajectory lines

## Development Notes

- Uses Svelte 5 runes (`$state`, `$derived`, `$effect`, `$props`)
- Tauri v2 capability-based permissions
- Plotly.js requires actual color values (doesn't support CSS variables)
- Individual icon imports from `@lucide/svelte/icons/`

## Future Use Cases

The CSV trajectory data is designed for machine learning applications:
- Motion pattern recognition
- Predictive path planning
- Anomaly detection
- Training data for robotic control models

## License

MIT

## Author

jeslitt
