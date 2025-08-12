# 🧠 Triage - People-Centric Knowledge Intelligence Platform

**Triage** has evolved into a revolutionary **People-Centric Knowledge Intelligence Platform** that transforms scattered organizational interactions into actionable collaboration insights. What started as a local-first Jira analysis tool now provides comprehensive **people network intelligence** across Jira, Google Workspace, and Slack, with advanced relationship mapping, expert discovery, and team optimization capabilities.

The platform runs as a cross-platform desktop app powered by **Electron**, **React**, **TailwindCSS**, and **D3**, backed by a **Rust + Axum + DuckDB** backend with sophisticated people graph analytics and OAuth integration for secure content access.

> **🎯 Core Innovation**: Unlike traditional knowledge management systems that focus on documents, Triage puts **people and their collaborative relationships** at the center, answering critical questions like "Who knows what?", "Who works well together?", and "Where are our knowledge gaps?"

- [🧠 Triage - People-Centric Knowledge Intelligence Platform](#-triage---people-centric-knowledge-intelligence-platform)
  - [⚙️ Tech Stack](#️-tech-stack)
    - [🎛️ Client (`/client`)](#️-client-client)
    - [🦀 Server (`/server`)](#-server-server)
  - [📁 Project Structure](#-project-structure)
  - [🚀 Getting Started](#-getting-started)
    - [✅ Prerequisites](#-prerequisites)
    - [▶️ Client Setup](#️-client-setup)
    - [🧹 Lint + Format](#-lint--format)
    - [📦 Build \& Package App](#-build--package-app)
    - [🦀 Server Setup](#-server-setup)
  - [🌐 API Endpoints](#-api-endpoints)
    - [People Intelligence 🧑‍🤝‍🧑](#people-intelligence-)
    - [Universal Search 🔍](#universal-search-)
    - [Authentication 🔐](#authentication-)
    - [Content Management 📄](#content-management-)
  - [🔑 Key Features](#-key-features)
    - [👥 People Intelligence (NEW)](#-people-intelligence-new)
    - [🧠 Advanced Collaboration Analytics](#-advanced-collaboration-analytics)
    - [🔍 Universal Knowledge Search](#-universal-knowledge-search)
    - [📊 Content Intelligence](#-content-intelligence)
    - [🔐 Secure Authentication](#-secure-authentication)
  - [📊 Current Status](#-current-status)
    - [🎯 **Ready for Production**](#-ready-for-production)
    - [🚀 **Business Value Delivered**](#-business-value-delivered)
  - [🛡️ License](#️-license)

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

- [Rust](https://www.rust-lang.org/) with [Axum](https://github.com/tokio-rs/axum) web framework
- [DuckDB](https://duckdb.org/) for analytical SQL and local storage
- OAuth 2.0 integration for Google Workspace and Slack
- Cross-platform content extraction and relationship mapping
- RESTful API for unified search and authentication

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
├── server/               # Rust backend with Axum API server
│   ├── src/
│   │   ├── main.rs       # Application entry point
│   │   ├── server.rs     # API routes and handlers
│   │   ├── people_graph.rs      # People identity resolution & network analysis
│   │   ├── people_integration.rs # Unified cross-platform people intelligence
│   │   ├── people_routes.rs     # People intelligence API endpoints
│   │   ├── enhanced_jira_extractor.rs   # Advanced Jira collaboration analysis
│   │   ├── enhanced_google_extractor.rs # Google Docs collaboration tracking
│   │   ├── enhanced_slack_extractor.rs  # Slack conversation dynamics
│   │   ├── unified_search.rs    # Cross-platform search engine
│   │   ├── content_storage.rs   # Unified content storage schema
│   │   ├── google_auth.rs       # Google OAuth integration
│   │   ├── slack_auth.rs        # Slack OAuth integration
│   │   ├── content_extractor.rs # Content extraction service
│   │   └── link_detector.rs     # Cross-platform link detection
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
cargo run serve    # Start API server on http://127.0.0.1:3001
```

Available commands:

```sh
cargo run login           # Authenticate with Jira
cargo run projects        # Sync Jira projects and issues
cargo run serve          # Start web server with API endpoints
```

Run Rust unit tests:

```sh
cargo test
```

## 🌐 API Endpoints

### People Intelligence 🧑‍🤝‍🧑

- `POST /people/analyze` - Extract people insights from content
- `GET /people/profile/{person_id}` - Get comprehensive person profile
- `GET /people/recommendations/{person_id}` - Get collaboration recommendations
- `GET /people/overview` - Network statistics and top collaborators

### Universal Search 🔍

- `POST /api/search/unified` - Cross-platform unified search
- `GET /api/search` - Semantic search (legacy)

### Authentication 🔐

- `GET /api/auth/google` - Initiate Google OAuth
- `GET /api/auth/slack` - Initiate Slack OAuth
- `GET /api/auth/{platform}/callback` - OAuth callback handler

### Content Management 📄

- `POST /api/content/extract` - Trigger content extraction
- `GET /api/content/status` - Extraction job status
- `GET /api/sync/status` - Sync status dashboard

## 🔑 Key Features

### 👥 People Intelligence (NEW)

- **People Network Graph**: Interactive D3 visualization of collaboration relationships
- **Expert Discovery**: AI-powered search to find experts by topic, skill, or past contributions
- **Collaboration Timeline**: Track knowledge transfer events and team dynamics over time
- **Team Insights Dashboard**: Executive analytics on collaboration effectiveness and knowledge gaps
- **Cross-Platform Identity Resolution**: Merge identities across Jira, Google, Slack (sarah.smith = ssmith = U123456)

### 🧠 Advanced Collaboration Analytics

- **Knowledge Transfer Detection**: Identify when experts teach others and solutions get shared
- **Influence Scoring**: Calculate authority based on problem-solving contributions and peer validation  
- **Collaboration Patterns**: Discover how teams actually work together across platforms
- **Knowledge Gap Analysis**: Find critical expertise shortfalls with actionable recommendations
- **Solution Pattern Recognition**: Surface "Sarah figured it out!" breakthrough moments

### 🔍 Universal Knowledge Search

- **Cross-Platform Search**: Unified search across Jira, Google Docs, Slack conversations, and more
- **Semantic Matching**: AI-powered content similarity and relationship mapping
- **People-Filtered Results**: Search by expertise areas, collaboration patterns, and influence metrics
- **Faceted Results**: Organized by people, concepts, technologies, projects, and relationships

### 📊 Content Intelligence  

- **Link Detection**: Automatically discover and extract content from platform links in tickets
- **Knowledge Extraction**: Identify concepts, technologies, and solution patterns
- **Relationship Mapping**: Connect related content and people across different platforms
- **Engagement Analytics**: Track usage, sharing, collaboration impact, and knowledge flow

### 🔐 Secure Authentication

- **OAuth 2.0**: Secure authentication with Google Workspace and Slack
- **Token Management**: Automatic token refresh and secure storage
- **Permission-Aware**: Respects platform access controls and sharing settings

## 📊 Current Status

**People Intelligence**: ✅ **Complete** - Full backend analytics with interactive frontend UI  
**Frontend**: ✅ **Complete** - 5 specialized components with D3 visualizations and real-time processing  
**Backend**: ✅ **Complete** - People graph, identity resolution, enhanced extractors, and API endpoints  
**OAuth**: ✅ **Complete** - Google and Slack integration ready  
**Search Engine**: ✅ **Complete** - Cross-platform unified search implemented  
**Content Processing**: ✅ **Complete** - Jira, Google Docs, Slack content analysis for people insights  

### 🎯 **Ready for Production**

The platform now provides comprehensive people-centric collaboration intelligence that transforms how teams discover expertise, optimize collaborations, and preserve institutional knowledge.

### 🚀 **Business Value Delivered**

- **Find Experts Fast**: "Who knows React?" → Sarah Smith (92% expertise, 15 people helped)
- **Optimize Teams**: Data-driven collaboration recommendations based on past success patterns  
- **Prevent Knowledge Loss**: Track and preserve institutional knowledge through people relationships
- **Close Skill Gaps**: Identify critical expertise shortfalls with actionable hiring/training recommendations

See `CLAUDE.md` for detailed implementation documentation and usage examples.

## 🛡️ License

MIT (future dual-license AGPL for community editions)
