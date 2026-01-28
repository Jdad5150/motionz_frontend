import { invoke } from "@tauri-apps/api/core";

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

    async function connect() {
        if (connecting) return;

        connecting = true;
        try {
            const status = await invoke<RobotStatus>("connect");
            console.log("Connected:", status);
            robotStatus = status;
            connected = true;
        } catch (error) {
            console.error("Connection failed:", error);
            connected = false;
            robotStatus = null;
        } finally {
            connecting = false;
        }
    }

    function disconnect() {
        connected = false;
        robotStatus = null;
    }

    return {
        get connected() { return connected; },
        get connecting() { return connecting; },
        get robotStatus() { return robotStatus; },
        connect,
        disconnect,
    };
}

export const connectionStore = createConnectionStore();
