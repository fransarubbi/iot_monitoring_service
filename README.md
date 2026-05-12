# IoT Monitoring Service

This service is a status monitor (watchdog) developed in **Rust** designed to monitor an IoT network. Its main function is to cross-reference information from a management database with a telemetry database to detect devices (Edges and Hubs) that have stopped reporting data and notify users via **Telegram**.

---

## 1. Architecture and Monitoring Logic

The system operates using an asynchronous loop managed by the **Tokio** runtime, performing periodic checks every 30 seconds.

### Detection Logic
The service implements **State Memory** logic to prevent notification spam:
* **Downtime Detection:** If a device exceeds its tolerance time without sending messages, an alert is sent, and the ID is saved in a memory register (`HashSet`).

* **Spam Prevention:** While the device remains down, no further alerts will be sent.

* **Recovery Notification:** As soon as the device sends another message, the system detects it, sends a "Recovered" message, and removes it from the alert memory.

* **Dynamic Synchronization:** If a network is deactivated or removed from the database, the service automatically clears its internal state for that device.

---

## 2. Technologies Used

* **Language:** Rust (https://www.rust-lang.org/).

* **Asynchronous Runtime:** Tokio (with multi-threading support).

* **Database:** sqlx with PostgreSQL driver (with connection pooling).

* **Notifications:** Telegram API via reqwest.

* **Time Management:** chrono (UTC).

* **Configuration:** dotenvy and environment variables. * **Logs and Diagnostics:** `tracing` and `tracing-subscriber`.

--- 

## 3. Project Structure

```text
monitoring/
├── Cargo.toml   # Dependency Definitions
├── src/
├── main.rs      # Entry Point and Orchestration
├── config.rs    # Loading Environment Variables
├── db.rs        # PostgreSQL Pool Management
├── models.rs    # Data Structures (SQL Mapping)
├── monitor.rs   # Watchdog Core Logic
└── telegram.rs  # Telegram Alert Client
