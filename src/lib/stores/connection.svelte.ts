import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

interface RobotStatus {
  position: { x: number; y: number; z: number };
  x_status: string;
  y_status: string;
  z_status: string;
  status: string;
}

function createConnectionStore() {
  let connected = $state(false);
  let connecting = $state(false);
  let robotStatus = $state<RobotStatus | null>(null);
  let unlistenStatus: UnlistenFn | null = null;
  let unlistenDisconnect: UnlistenFn | null = null;

  async function connect() {
    if (connecting || connected) return;

    connecting = true;
    try {
      // Set up event listeners before connecting
      unlistenStatus = await listen<RobotStatus>("robot-status", (event) => {
        robotStatus = event.payload;
      });

      unlistenDisconnect = await listen("robot-disconnected", () => {
        console.log("Robot disconnected");
        cleanup();
      });

      // Connect and get initial status
      const status = await invoke<RobotStatus>("connect");
      console.log("Connected:", status);
      robotStatus = status;
      connected = true;
    } catch (error) {
      console.error("Connection failed:", error);
      cleanup();
    } finally {
      connecting = false;
    }
  }

  async function disconnect() {
    try {
      await invoke("disconnect");
    } catch (error) {
      console.error("Disconnect error:", error);
    }
    cleanup();
  }

  function cleanup() {
    connected = false;
    robotStatus = null;
    if (unlistenStatus) {
      unlistenStatus();
      unlistenStatus = null;
    }
    if (unlistenDisconnect) {
      unlistenDisconnect();
      unlistenDisconnect = null;
    }
  }

  return {
    get connected() {
      return connected;
    },
    get connecting() {
      return connecting;
    },
    get robotStatus() {
      return robotStatus;
    },
    connect,
    disconnect,
  };
}

export const connectionStore = createConnectionStore();
