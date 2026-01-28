# Motionz Frontend

A modern desktop application for robot motion control and trajectory visualization built with Tauri, SvelteKit, and Plotly.js.

## Architecture

```
Frontend (Tauri + Svelte)  <-->  Zig Server (Pi)  <-->  Pico Controllers
     HTTP/JSON                    Serial/UART
```

The frontend communicates with a Zig-based HTTP server running on a Raspberry Pi, which relays commands to Pico microcontrollers over serial connections.

## Features

### LIVE Mode
- **Real-time Control**: X/Y/Z position inputs with speed sliders (0-500 mm/s)
- **Connection Management**: Shared connection state with visual status indicator
- **3D Trajectory Visualization**: Live plotting of robot movement paths using Plotly.js
- **Live Telemetry**: Collapsible banner showing current position, axis status, and connection state
- **Demo Mode**: Simulated spiral trajectory pattern for testing without hardware
- **Auto-save**: Trajectories automatically exported to CSV in app data directory

### REVIEW Mode
- **Multi-trajectory Analysis**: Load and compare multiple saved trajectories
- **Tabbed Interface**: Switch between different trajectory datasets
- **3D Visualization**: Full interactive 3D plots of historical movement data
- **CSV Import**: Load trajectory data from any CSV file

### Settings
- **Persistent Configuration**: Settings stored in app data directory and shared between frontend and Rust backend
- **Server URL**: Configurable connection endpoint for the Zig server
- **Default Speeds**: Configurable default speed values per axis

### Additional Features
- **Custom Titlebar**: Frameless window with integrated menu bar and window controls
- **Theme Toggle**: Light/dark mode support
- **Resizable Layout**: Adjustable control panel and visualization areas

## Tech Stack

- **Frontend**: SvelteKit 5 (with runes), TypeScript, Tailwind CSS
- **UI Components**: shadcn-svelte (new-york style)
- **Desktop Framework**: Tauri v2
- **Backend**: Rust (reqwest for HTTP, serde for JSON)
- **Visualization**: Plotly.js
- **Icons**: Lucide Svelte

## Getting Started

### Prerequisites
- Node.js (v18+)
- Rust (latest stable)
- npm

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
│   ├── stores/
│   │   └── connection.svelte.ts     # Shared connection state
│   └── utils.ts                     # Utility functions
├── routes/
│   ├── +layout.svelte               # Root layout with titlebar
│   ├── +page.svelte                 # Main page with resizable panes
│   └── layout.css                   # Theme configuration
└── src-tauri/
    ├── src/lib.rs                   # Tauri commands & app state
    ├── capabilities/default.json    # Permission configuration
    └── tauri.conf.json              # Tauri settings
```

## API

### Tauri Commands

| Command | Description |
|---------|-------------|
| `connect` | Connect to server, returns robot status |
| `send_go_command` | Send X/Y/Z position command |
| `get_settings` | Get current settings |
| `update_settings` | Update and persist settings |

### Server Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/position` | GET | Get current robot position and status |
| `/api/move` | POST | Send move command `{x, y, z}` |

## Data Format

Trajectories are saved as CSV files with the following format:

```csv
timestamp,x,y,z
1234567890,10.5,20.3,15.7
1234567891,10.6,20.4,15.8
...
```

**Storage Location**:
- **Linux**: `~/.local/share/motionz-frontend/`
- **macOS**: `~/Library/Application Support/motionz-frontend/`
- **Windows**: `%APPDATA%\motionz-frontend\`

## Development Notes

- Uses Svelte 5 runes (`$state`, `$derived`, `$effect`, `$props`)
- Tauri v2 capability-based permissions
- Shared connection state via Svelte 5 store pattern
- Settings persisted to JSON in app data directory
- Plotly.js requires actual color values (doesn't support CSS variables)

## License

MIT

## Author

jeslitt
