# QPie MCP Server🍓

### AI-Accessible Hardware Runtime for Raspberry Pi

**QPie** is a lightweight runtime that exposes **Raspberry Pi hardware and system capabilities** through APIs and an **MCP control layer**, allowing AI agents, automation systems, and developers to interact with real-world devices.

It bridges the gap between **AI systems and physical hardware**.

---

## ✨ Features

* 🔌 **GPIO Control** – Read and write Raspberry Pi GPIO pins
* 🌡 **Temperature Monitoring** – Access CPU temperature
* 📡 **WiFi Scanning** – Discover nearby networks
* 📷 **Camera Control** – Capture images from the Pi camera
* 🧠 **System Metrics** – CPU, memory, and disk monitoring
* 🔍 **Process Management** – Inspect running processes
* 🔗 **I2C Device Discovery** – Detect connected sensors
* 🤖 **MCP Integration** – AI tools can control and query hardware

---

## 🧠 Architecture

QPie separates **control interfaces** from **data interfaces**.

```
                ┌──────────────────────┐
                │      AI Agents       │
                │  (Claude / LLMs)    │
                └──────────┬───────────┘
                           │
                           │ MCP
                           ▼
                 ┌─────────────────┐
                 │   MCP Control   │
                 │  (tools layer)  │
                 └───────┬─────────┘
                         │
            ┌────────────┼─────────────┐
            ▼            ▼             ▼
        GPIO API    Sensor APIs   Camera API
            │            │             │
            └─────── Raspberry Pi Hardware ────────
```

### Design Principles

| Layer              | Purpose                         |
| ------------------ | ------------------------------- |
| **MCP**            | AI control & tool discovery     |
| **REST APIs**      | sensor data & system monitoring |
| **Hardware Layer** | GPIO, camera, I2C, WiFi         |

This keeps **AI interactions simple and safe**, while allowing high-performance APIs for other systems.

---

## 📦 Project Structure

```
qpie
 ├── Cargo.toml
 └── src
     ├── main.rs
     ├── tools.rs
     ├── system.rs
     ├── gpio.rs
     ├── temperature.rs
     ├── wifi.rs
     ├── camera.rs
     ├── process.rs
     └── i2c.rs
```

Each module handles a specific hardware or system capability.

---

## 🚀 Getting Started

### Requirements

* Rust 1.70+
* Raspberry Pi OS (recommended)
* Optional hardware:

  * Pi Camera
  * I2C sensors
  * GPIO devices

---

### Install Rust

```
curl https://sh.rustup.rs -sSf | sh
```

---

### Build

```
cargo build
```

---

### Run

```
cargo run
```

Server starts on:

```
http://localhost:8080
```

---

## 🔌 REST API Endpoints

### System

```
GET /cpu
GET /memory
GET /disk
```

### Hardware

```
GET /temperature
GET /wifi/scan
GET /process/list
GET /i2c/scan
```

### GPIO

```
GET  /gpio/:pin
POST /gpio/write
```

Example:

```
POST /gpio/write
{
  "pin": 17,
  "value": "high"
}
```

---

## 🤖 MCP Tools

The MCP layer exposes hardware capabilities as **AI tools**.

```
GET /tools
```

Example response:

```json
[
  {"name":"cpu_usage"},
  {"name":"memory_usage"},
  {"name":"disk_usage"},
  {"name":"gpio_read"},
  {"name":"gpio_write"},
  {"name":"temperature"},
  {"name":"wifi_scan"},
  {"name":"camera_photo"},
  {"name":"process_list"},
  {"name":"i2c_scan"}
]
```

AI agents can call these tools to interact with the physical system.

---

## 🧪 Development Mode

When running on **non-Linux systems**, hardware calls are **simulated** so development can be done on macOS or Windows.

Example:

```
temperature -> SIMULATED
```

---

## 🧩 Example Use Cases

### AI-controlled robotics

AI can read sensors and control motors through GPIO.

---

### Smart home hub

Monitor sensors and control relays via MCP.

---

### Edge AI node

Combine QPie with LLM agents for real-time automation.

---

## 🔒 Safety Philosophy

QPie keeps **hardware control explicit**.

* MCP handles **control operations**
* REST APIs handle **data access**

This prevents unintended automation from uncontrolled AI calls.

---

## 🛠 Technology Stack

* Rust
* Axum web framework
* Sysinfo system metrics
* RPPAL Raspberry Pi hardware access
* Tokio async runtime

---

## 🤝 Contributing

Contributions are welcome!

You can help by adding:

* sensor drivers
* MCP tool improvements
* device integrations
* performance optimizations

---

## 📜 License

MIT License

---

## 🌍 Vision

QPie aims to become a **standard interface between AI systems and physical devices**, enabling safe, structured, and powerful interaction with hardware.

Think of it as:

> **“An operating layer where AI meets the real world.”**

---
