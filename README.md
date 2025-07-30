# Binance API in Rust 📡

## Warning ⚠️  
**This project is under active development and may contain bugs or partial implementations. Use at your own risk.** ❌

---

## Credits ⭐  

### People & Ideas 💡  
- 🦀 **Rust Language & Community** — for making systems programming modern and safe.  
- 📘 **Binance API Docs** — [https://binance-docs.github.io/apidocs/](https://binance-docs.github.io/apidocs/)  
- 🧠 **High-frequency trading system architectures** — inspiration for thread-safe, real-time, high-performance design.  
- 💬 **ChatGPT by OpenAI** — for guidance, feedback, and code architecture suggestions. 🤖  

### Crates & Libraries 📦  
| Crate               | Version     | Purpose                                                                 | Link                                                                 |
|---------------------|-------------|-------------------------------------------------------------------------|----------------------------------------------------------------------|
| `ed25519-dalek`     | 2.1.1       | Ed25519 digital signature algorithm (crypto)                            | [docs.rs](https://docs.rs/ed25519-dalek)                            |
| `futures-util`      | 0.3.31      | Utilities for async/await, stream handling                              | [docs.rs](https://docs.rs/futures-util)                             |
| `hex`               | 0.4.3       | Encode/decode hex strings                                               | [docs.rs](https://docs.rs/hex)                                      |
| `hmac`              | 0.12.1      | HMAC (keyed-hash message authentication)                               | [docs.rs](https://docs.rs/hmac)                                     |
| `reqwest`           | 0.12.15     | HTTP client with TLS + JSON support                                     | [docs.rs](https://docs.rs/reqwest)                                  |
| `serde`             | 1.0.219     | Serialization framework                                                 | [docs.rs](https://docs.rs/serde)                                    |
| `serde_json`        | 1.0.140     | JSON serialization/deserialization                                      | [docs.rs](https://docs.rs/serde_json)                               |
| `serde_urlencoded`  | 0.7.1       | URL form encoding/decoding support                                      | [docs.rs](https://docs.rs/serde_urlencoded)                         |
| `sha2`              | 0.10.8      | SHA-2 family hash functions                                             | [docs.rs](https://docs.rs/sha2)                                     |
| `tokio`             | 1.44.2      | Asynchronous runtime for Rust                                           | [docs.rs](https://docs.rs/tokio)                                    |
| `tokio-tungstenite` | 0.26.2      | Async WebSocket client/server over Tokio                                | [docs.rs](https://docs.rs/tokio-tungstenite)                        |
| `binance-common`    | —           | Shared types/utilities (local crate)                                    | Local path: `../binance-common`                                     |
| `chrono`            | 0.4.40      | Date and time library            

---

## About the Project 🪴  

This library provides both asynchronous and synchronous Binance API clients for Spot and Futures markets. REST clients are separated by type (market, trade, account, etc.) and asset class (Spot, USDT Futures, COIN-M Futures).

Authenticated endpoints use an HMAC-SHA256 signer to generate signatures for private actions like placing orders or querying account balances. All API types share common data structures and error handling logic for consistency.

For WebSockets, the project supports the Websocket Market for Futures data streaming with a multiplexed, manager/controller architecture that enables multiple topics (e.g. trades, depth, kline, account) to be streamed concurrently over a single connection. Messages are routed across threads using thread-safe channels and handled with strongly typed enums (WebSocketResponse) for zero-cost abstraction and safety.

The code is modular and built for extension — REST and WS clients can be plugged into higher-level trading systems, bots, or analytics engines.
---

## Built With ⚒️  

- 🦀 [Rust](https://www.rust-lang.org/)
- ⚙️ [Tokio](https://tokio.rs/) — async runtime
- 🌐 [Tungstenite](https://github.com/snapview/tungstenite-rs) — WebSocket protocol
- 🧩 [Serde](https://serde.rs/) — serialization/deserialization
- 📦 [Cargo](https://doc.rust-lang.org/cargo/) — package manager

---

## Getting Started 🚀  

### System Requirements ✅  
- Operating System: Linux, macOS
- Rust toolchain (recommended: stable)
- WebSocket connectivity (ensure ports aren't blocked)

## Features 🌐

- ✅ **Subscribe to trade, depth, kline, and user/account WebSocket streams**  
  Real-time access to Binance market and account events.

- 🧠 **Multiplexed WebSocket handling with internal manager/controller architecture**  
  Efficient coordination of multiple WebSocket tasks in a structured system.

- 🧵 **Thread-safe command + response channel design**  
  Allows cross-thread interaction with WebSocket tasks using safe, bounded channels.

- 🔧 **Custom message enums (`WebSocketResponse`) for type-safe stream handling**  
  Ensures efficient and maintainable decoding of WebSocket messages.

- 🔌 **Easily extendable to REST integration or order execution**  
  Designed with future support in mind for REST trading endpoints and more.

---

## Learning Resources 🌱

- 📘 [Binance API Docs](https://binance-docs.github.io/apidocs/)
- 🧵 [Tokio Book](https://tokio.rs/tokio/tutorial)
- 📙 [Rust Async Book](https://rust-lang.github.io/async-book/)
- 🦀 [The Rust Programming Language](https://doc.rust-lang.org/book/)
- 📈 [Crypto Trading Systems in Rust (cryptick)](https://github.com/bitsy-ai/cryptick)
- 🔌 [Tungstenite Docs](https://docs.rs/tungstenite)
- 📚 [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

---

## Contributing 🏗️

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any improvements, bug reports, or ideas are **greatly appreciated**!

### How to Contribute
If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

Describe clearly what you’ve added or fixed, and wait for feedback!
Don’t forget to ⭐ the project if you found it useful — thanks again for your support!

## Contact. ✉️
**Denis Gruia**

- Twitter - [@denisgruiax](https://twitter.com/denisgruiax) 
- Email - denis.gruiax@icloud.com
- Project Link - [https://github.com/denisgruiax](https://github.com/denisgruiax)
