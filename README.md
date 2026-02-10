# Real-Time IoT Light Controller

This project is a full-stack, real-time control system for IoT devices (simulated as microcontrollers) that manage lights. It features a web-based UI that dynamically adjusts to discover and control multiple devices, showcasing a reactive and decoupled architecture ideal for modern IoT applications.

![GIF Placeholder: A screen recording of the web UI discovering and toggling the simulated lights would go here.]

---

## Architecture

The system is composed of four main components that communicate asynchronously, ensuring scalability and resilience.

![Architecture Diagram Placeholder: A diagram showing Frontend -> Backend -> MQTT -> MCU and the WebSocket connection would be effective here.]

1.  **Frontend Web UI (`static/index.html`)**:
    *   A pure HTML, CSS, and vanilla JavaScript single-page application that provides the user interface.
    *   It establishes a WebSocket connection to the backend server to receive real-time state updates.
    *   When a user toggles a light, it sends a command to the backend via an HTTP POST request.
    *   The UI dynamically renders controls for each device discovered by the backend, making the system scalable without frontend changes.

2.  **Backend Server (`light-server`)**:
    *   Built with Rust using the **Axum** web framework and **Tokio** for asynchronous operations.
    *   It serves the static frontend files.
    *   It exposes HTTP endpoints (`/update`, `/state`) for receiving commands and a WebSocket endpoint (`/ws`) for broadcasting state changes.
    *   It contains the core business logic within a `LightActor`, which maintains the state of all known devices.
    *   It communicates with the IoT devices via an MQTT broker, publishing commands and subscribing to status updates.

3.  **MQTT Broker (e.g., Mosquitto)**:
    *   Acts as the central message bus, decoupling the backend server from the IoT devices. This is a standard and highly effective pattern in IoT systems.
    *   The server publishes `cmd/{id}/power` topics to send commands.
    *   The devices (or simulators) publish `stat/{id}/power` topics to report their state.

4.  **MCU Simulator (`mcu_simulator`)**:
    *   A Rust application that simulates multiple microcontrollers.
    *   On startup, it announces the presence of several devices by publishing an initial "OFF" state to their respective `stat` topics.
    *   It subscribes to `cmd/+/power` topics. When it receives a command for one of its simulated devices, it publishes a corresponding state update back to the appropriate `stat` topic, completing the control loop.

### Key Feature: Dynamic Device Discovery

This project avoids hardcoding device IDs. When the backend server receives an MQTT message on the `stat/+/power` topic from a previously unseen device ID, it automatically adds that device to its internal state map. The updated state map is then broadcast via WebSocket to all connected web clients, which dynamically render a new control for the newly discovered device.

---

## Getting Started

### Prerequisites

1.  **Rust**: Install the Rust toolchain by following the official instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).
2.  **MQTT Broker**: Install a broker like Mosquitto.
    *   **macOS**: `brew install mosquitto`
    *   **Ubuntu**: `sudo apt-get install mosquitto mosquitto-clients`
    *   For other systems, see the [Mosquitto Downloads page](https://mosquitto.org/download/).

### Running the System

You will need to run three components in three separate terminal windows.

**1. Start the MQTT Broker**

Open your first terminal and start the Mosquitto service. This command may vary based on your installation.

```bash
# This is the most common command
mosquitto
```

**2. Start the Light Server**

In your second terminal, navigate to the project root and run the main server binary:

```bash
cargo run --bin light-server
```

You should see output indicating the server is listening on `0.0.0.0:3000`.

**3. Start the MCU Simulator**

In your third terminal, run the MCU simulator binary. This will announce the simulated devices to the broker, which the server will then discover.

```bash
cargo run --bin mcu_simulator
```

**4. View the Frontend**

Open your web browser and navigate to `http://localhost:3000`. You should see the web interface with toggles for "Node 0", "Node 1", and "Node 2". You can now toggle the lights and see their state update in real-time.# light-server
