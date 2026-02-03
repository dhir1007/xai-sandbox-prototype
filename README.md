# xAI-Sandbox-Prototype 

A high-performance, asynchronous WebAssembly execution environment built in **Rust**. This project serves as a prototype for a secure code execution service, enforcing hardware-level resource constraints (CPU/Memory) on untrusted guest modules.



## Key Features
* **Strict Resource Isolation:** Custom implementation of `wasmtime::ResourceLimiter` to cap linear memory growth and table growth.
* **Instruction-Level Fuel Metering:** Prevents Infinite Loop DoS attacks by enforcing execution "fuel" (CPU instruction) limits.
* **Asynchronous Orchestration:** Built with **Tokio** to handle concurrent execution environments without blocking the host system.
* **Structured Telemetry:** Real-time monitoring of fuel consumption, memory usage, and sub-microsecond execution duration via `env_logger`.
* **Modular Systems Architecture:** Clean separation of concerns between CLI parsing, execution logic, and security state.

## Architecture
The system is architected into four core modules:
1.  **`main.rs`**: Entry point that manages the async runtime and CLI orchestration.
2.  **`cli.rs`**: Handles terminal input, dynamic argument collection, and type validation using `clap`.
3.  **`executor.rs`**: The engine room; handles `wasmtime` configurations, module instantiation, and dynamic function calls.
4.  **`limiter.rs`**: The security layer; implements the `ResourceLimiter` trait to monitor and restrict hardware access.



## Installation

1.  **Install Rust:** Ensure you have the [Rust toolchain](https://rustup.rs/) installed.
2.  **Clone the Repository:**
    ```bash
    git clone [https://github.com/dhir1007/xai-sandbox-prototype](https://github.com/dhir1007/xai-sandbox-prototype)
    cd xai-sandbox-prototype
    ```
3.  **Build:**
    ```bash
    cargo build --release
    ```

## Usage

Run a WASM module by providing the file path, function name, and optional resource limits.

```bash
# Basic run with default limits (10MB RAM, 10,000 Fuel units)
RUST_LOG=info cargo run -- run add.wasm 5 10

# Custom limits: 20MB Memory and 50,000 Fuel units
RUST_LOG=info cargo run -- run add.wasm --memory 20 --fuel 50000 12 30
