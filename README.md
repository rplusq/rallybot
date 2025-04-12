# 🏓 RallyBot

**Smart matchmaking and league management for Padel clubs** — built in Rust with a focus on clean architecture, modularity, and developer joy.

---

## 🎯 Vision

RallyBot aims to be a lightweight, highly modular, robust, and well-tested matchmaking and league management system built entirely in Rust, primarily for Padel clubs. It addresses the common pain points of manual scheduling, uneven matches, and fragmented communication within club communities.

The system manifests as a CLI tool coupled with chatbot integrations (Telegram, Discord) to provide a seamless experience for players and administrators.

**Core Goals:**
- **Streamline Operations:** Automate matchmaking, scheduling, results tracking, and communication
- **Enhance Player Experience:** Provide fair, skill-matched games with zero-friction signups
- **Foster Community:** Support various play formats (competitive leagues, social ladders, mix-ins)
- **Build a Solid Foundation:** Create a well-architected backend that is maintainable and extensible
- **Emulate Best Practices:** Adopt architectural patterns from high-quality Rust projects like Reth

---

## ✨ Core Features

### Matchmaking & Scheduling
- 🎯 Skill-based matchmaking (Elo or manual levels)
- 📅 Automated scheduling based on player availability
- 🤝 Support for different game formats (2v2)
- ⚙️ Manual match creation/adjustment by admins

### Competitive Leagues
- 🏆 Structured league formats with promotion/relegation
- 📊 Automated round generation and match creation
- 📈 Dynamic leaderboards and ranking updates
- ⚙️ Configurable league parameters

### Social Games & Events
- 🎉 Recurring social sessions
- 🏃 Ladder formats with challenge mechanisms
- 🤝 "Swap & Mix" style events
- 📝 Sign-up management

### Communication & User Interaction
- 🤖 Bot Interface (Telegram, Discord)
  - Player commands (sign up, report results, check schedule)
  - Admin commands (manage players, configure leagues)
  - Automated notifications
- 💻 CLI Interface (Admin/Advanced)
  - System administration and configuration
  - Bulk operations and debugging
  - Direct database interaction

---

## 🏗 Architecture

Inspired by the modularity and robustness of projects like Reth, RallyBot is built as a Cargo workspace with clear separation of concerns.

### Workspace Structure
```txt
rallybot/
├── crates/
│   ├── rallybot-core        # Matchmaking logic, business rules
│   ├── rallybot-types       # Shared domain types
│   ├── rallybot-db          # Storage trait + PostgreSQL impl
│   ├── rallybot-cli         # CLI interface
│   ├── rallybot-bot         # Bot platform integrations
│   ├── rallybot-config      # Config loading
│   ├── rallybot-error       # Centralized error types
│   └── rallybot-utils       # Shared utilities
├── config/                  # Config TOML files
├── tests/                   # Integration / prop tests
└── Cargo.toml               # Workspace root
```

### Tech Stack
| Area            | Tooling                   |
|-----------------|---------------------------|
| Language        | Rust                      |
| Async Runtime   | Tokio                     |
| CLI Parser      | Clap                      |
| Config          | Figment + Serde           |
| Bot API         | Teloxide (Telegram)       |
| Database        | PostgreSQL via `sqlx`     |
| Error Handling  | `thiserror`               |
| Logging/Tracing | `tracing`, `tracing-subscriber` |
| Testing         | `cargo test`, `proptest`  |

---

## 🚀 Getting Started

```bash
git clone https://github.com/yourusername/rallybot
cd rallybot
cargo build
cargo run -p rallybot-cli
```

### Configuration
Edit `config/config.toml`:
```toml
[default]
db_url = "postgres://localhost/rallybot"
bot_token = "YOUR_TELEGRAM_BOT_TOKEN"
league_name = "Rising League"
```

---

## 🧪 Development Practices

- **Testing Strategy:**
  - Unit tests for pure logic
  - Integration tests for crate interactions
  - Property-based testing for core algorithms
- **Error Handling:** Consistent use of `Result<T, RallyBotError>`
- **Observability:** Structured logging with `tracing`
- **Code Quality:** Enforced via `rustfmt` and `clippy`
- **CI/CD:** GitHub Actions for automated checks

---

## 🔮 Future Plans

- 🌐 Web UI (Axum, Tauri)
- 📊 Public leaderboard API
- 🧠 Adaptive Elo rating system
- 🧪 League data seeding tools
- 🕹️ Live match admin dashboard
- ✨ Multi-club support

---

## 🤝 Contributing

This project is designed as a solo-to-collaborative evolution. It's a space to:
- Learn Rust deeply
- Practice modular backend architecture
- Deliver tools people actually use

PRs, ideas, and issues are welcome!

---

## 👤 Maintainer

Rafael Quintero  
Smart Contract Engineer @ Reown  
Rust enthusiast | Padeler | Systems Learner  
X: [@rplusq](https://twitter.com/rplusq)

---

## 🧠 Philosophy

RallyBot is my training ground to become the kind of engineer who contributes to things like Reth, Foundry, or Erigon—by solving real-world problems for people I care about.

Build tools. Sharpen your mind. Rally together. 🎾