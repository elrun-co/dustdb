# 🚀 DustDB — A  In-Memory Cache Database for Distributed Systems 

DustDB is a **high-performance**, **standalone**, and **networked** in-memory key-value database — built entirely in **Rust** for systems-level reliability and blazing speed.  
It’s inspired by Redis, but written from the ground up to demonstrate zero-cost abstractions, async concurrency, and modular scalability.

---

## ✨ Features

### ⚡ Core Database (In-Memory)
- **Key–Value store** using lock-free `DashMap`
- **TTL / Expiry support** (like Redis `SETEX` or `EXPIRE`)
- **Atomic increment (`INCR`)**
- **Automatic memory cleaner task** to evict expired keys

### 🧩 Networking Layer
- Built on **Tokio** async runtime
- Uses **TCP socket server** — compatible with `netcat`, `telnet`, or any client (HTTP/cURL over raw TCP)
- Fully **text-based command protocol** inspired by Redis RESP

Example:
SET mykey hello
GET mykey
DEL mykey
EXPIRE mykey 10


### 🗄️ Persistence Layer
- Optional **AOF/RDB-like persistence** using JSON snapshots
- Data durability via `Persistence` struct
- Can be extended to disk-based snapshots or binary serialization (`bincode`)

### 📡 Pub/Sub System
- Lightweight **Publish/Subscribe** messaging channels
- Subscribers receive broadcast messages in real-time
- Thread-safe via `Arc<Mutex<HashMap>>`

Commands:
SUBSCRIBE news
PUBLISH news "Rust is awesome!"


### 🔐 Authentication Layer
- Optional password protection for client sessions
- Disabled if no password is configured

Command:

### 🔁 Clustering & Replication (Pluggable)
- Nodes can join a cluster via `CLUSTERJOIN <ip:port>`
- Nodes automatically sync keyspaces (in progress)
- Supports horizontal scaling and multi-node discovery
- Uses periodic TCP sync to maintain cluster consistency

---

## 📁 Project Structure

dustdb/
├── Cargo.toml
└── src/
├── main.rs # Entry point and TCP server
├── db.rs # Core in-memory database (TTL, set/get/del/incr)
├── commands.rs # Command parser and dispatcher
├── cluster.rs # Cluster and replication logic
├── persistence.rs # Disk snapshot (AOF/RDB-like persistence)
├── pubsub.rs # Publish/Subscribe implementation
├── auth.rs # Authentication manager
└── protocol.rs # Command parsing (RESP-lite)



---

## 🧰 Commands Supported

| Command | Description | Example |
|----------|--------------|---------|
| `SET <key> <value>` | Store a value in memory | `SET foo bar` |
| `GET <key>` | Retrieve a value | `GET foo` |
| `DEL <key>` | Delete a key | `DEL foo` |
| `EXPIRE <key> <seconds>` | Set TTL | `EXPIRE foo 30` |
| `INCR <key>` | Atomic integer increment | `INCR counter` |
| `KEYS` | List all keys | `KEYS` |
| `AUTH <password>` | Authenticate client | `AUTH mypass` |
| `SUBSCRIBE <channel>` | Listen for pub/sub events | `SUBSCRIBE news` |
| `PUBLISH <channel> <msg>` | Publish message | `PUBLISH news hello` |
| `CLUSTERJOIN <ip:port>` | Join another DustDB node | `CLUSTERJOIN 127.0.0.1:6380` |

---

## 🧩 Cluster Design Overview

Each DustDB node:
- Listens on a TCP port (default: `6379`)
- Maintains a list of peer nodes
- Periodically synchronizes key-value pairs
- Propagates mutations (`SET`, `DEL`, `EXPIRE`, etc.) to peers asynchronously

The clustering system will evolve into a **Redis Cluster–style topology**, with:
- **Leader election**
- **Slot-based sharding**
- **Full/partial replication**

---

## ⚙️ Running DustDB

```bash
# 1️⃣ Build
cargo build --release

# 2️⃣ Run server
cargo run

# 3️⃣ Connect from another terminal
nc 127.0.0.1 6379
SET hello world
GET hello



cargo run -- --port 6379
cargo run -- --port 6380



CLUSTERJOIN 127.0.0.1:6380
---
## run with netcat ** nc ** or nmap ** ncat**   
`

$ nc 127.0.0.1 6379
SET hello world
OK
GET hello
world
INCR counter
1
EXPIRE hello 10
OK
PUBLISH news "Rust 2.0 released!"
Subscribers: 3


`


🧾 License

GPL3 © 2025 Yasir Gujjar —
Made with 💙 for the future of distributed systems.
