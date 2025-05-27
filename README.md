# 🦀 RedisOnWings

> A blazing-fast, modular, Redis-like in-memory data store written in Rust, built for performance, extensibility, and learning how real databases work under the hood.

For version = V0.1.0

---

## 🚀 Overview

`redisonwings` is a multi-crate Rust workspace that reimagines the core design of Redis, focusing on high-performance primitives, protocol parsing, command execution, and eventual cluster support. It's ideal for learning systems programming, protocol design, and scalable architecture with Rust.

---

## 📁 Project Structure

This project is structured into independent crates for maximum modularity and reusability:

```text
.
├── Cargo.toml                   # Root workspace manifest
├── redis-server/               # Main server binary (entry point)
├── redis-cli/                  # Command-line client to interact with server
├── redis-storage/             # Core in-memory storage engine
├── redis-protocol/            # RESP parser and encoder
├── redis-commands/            # Redis command implementations
├── redis-utils/               # Shared helpers and utilities
├── redis-tests/               # Cross-crate integration and unit tests
└── target/                     # (ignored) Build artifacts

```

---

🛠 Features
⚡ RESP protocol support (RESP v2, v3 parsing)

🧠 In-memory key-value store with modular storage engine

🧪 Built-in test suite across crates

🧰 Clean separation of responsibilities (protocol, commands, CLI, server, etc.)

📦 Workspace-wide dependency and build management

🦺 Rust 2024 edition with resolver = "3" and strict build profiles

---

## 🔧 Installation

Make sure you have [Rust](https://rustup.rs) installed (recommended: latest stable with `rustup`):

```bash
git clone https://github.com/your-username/redisonwings.git
cd redisonwings
cargo build --workspace
```

---

## 🧪 Running Tests

Run tests across the entire workspace:

```bash
cargo test --workspace
```

Or run tests for a specific crate (e.g., `redis-protocol`):

```bash
cargo test -p redis-protocol
```

---

## 🧵 Running the Server

Launch the Redis-like server:

```bash
cargo run -p redis-server
```

---

## 💬 Using the CLI

Interact with the server using the bundled CLI client:

```bash
cargo run -p redis-cli
```

---

## 🤝 Contributing

We welcome contributions! Feel free to submit PRs, open issues, or suggest improvements. If you're learning Rust or interested in distributed systems, this is a great place to grow.

---

## 📜 License

MIT License © 2025 \[Abhishek]

---

## 💡 Inspirations

- [Redis](https://redis.io) — for the original brilliance
- [tokio](https://tokio.rs) — for async networking
- [Mini-Redis](https://github.com/tokio-rs/mini-redis) — for learning-focused implementation
- [Rust Language](https://www.rust-lang.org) — for memory safety and zero-cost abstractions

---

## 🧭 Roadmap (WIP)

- [ ] Command support: `GET`, `SET`, `DEL`, `EXPIRE`, etc.
- [ ] Background save (RDB/AOF)
- [ ] Publish/Subscribe
- [ ] Authentication
- [ ] Clustering
- [ ] Web dashboard (monitoring, insights)
