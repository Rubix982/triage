# 🧠 Triage

**Triage** is a local-first developer and analyst tool for querying, visualizing, and exploring structured issue data. It runs as a cross-platform desktop app powered by **Electron**, **React**, **TailwindCSS**, and **D3**, backed by a **Rust + DuckDB** backend.

This project is designed for performance, expressiveness, and clarity — with zero external cloud dependencies.

---

## ⚙️ Tech Stack

### 🎛️ Client (`/client`)

- [Electron](https://www.electronjs.org/) — native desktop runtime
- [Vite](https://vitejs.dev/) — fast dev & build tooling
- [React 19](https://react.dev/) — UI
- [Tailwind CSS](https://tailwindcss.com/) — styling
- [D3.js](https://d3js.org/) — visualizations
- [DuckDB-WASM](https://duckdb.org/) — analytical SQL engine in the browser

### 🦀 Server (`/server`)

- [Rust](https://www.rust-lang.org/)
- [DuckDB](https://duckdb.org/) CLI integration
- JSON and file-backed issue analysis

---

## 📁 Project Structure

```text
triage/
├── client/               # Electron + React desktop UI
│   ├── src/
│   │   ├── main.ts       # Electron main process
│   │   ├── index.tsx     # React entry point
│   │   ├── App.tsx       # Root React component
│   │   ├── styles/       # Tailwind input CSS
│   │   ├── backend/      # DuckDB logic (frontend)
│   │   ├── components/   # D3, charts, reusable UI
│   │   └── index.html    # Injected by Vite
│   ├── dist-electron/    # Electron build output
│   ├── tailwind.config.js
│   ├── eslint.config.js  # ESLint v9+ flat config
│   └── vite.config.ts    # Vite + Electron setup
│
├── server/               # Rust backend
│   ├── src/              # Rust source
│   ├── docs/             # DuckDB feature docs
│   └── misc/             # Sample issue data
│
├── docs/                 # Shared references
└── dist/                 # Final packaged output (auto-generated)
```

---

## 🚀 Getting Started

### ✅ Prerequisites

- Node.js + `pnpm`
- Rust + `cargo`
- DuckDB CLI (optional)

---

### ▶️ Client Setup

```bash
cd client
pnpm install
pnpm dev
```

This starts:

- Electron main process
- Vite dev server (with hot reloading)
- Tailwind build watcher

### 🧹 Lint + Format

```bash
pnpm lint       # Run ESLint (flat config)
pnpm lint:fix   # Auto-fix
pnpm format     # Prettier write for all files
```

Note: Uses ESLint v9+ Flat Config (eslint.config.js), with support for:

• TypeScript
• React + Hooks
• Tailwind class ordering
• Prettier conflict resolution

### 📦 Build & Package App

```bash
pnpm build           # Build Vite renderer
pnpm dev:main        # Build Electron main process
pnpm run package     # Create .dmg, .exe, or .AppImage via electron-builder
```

### 🦀 Server Setup

```sh
cd server
cargo build
cargo run
```

Run Rust unit tests:

```sh
cargo test
```

## 🛡️ License

MIT (future dual-license AGPL for community editions)
