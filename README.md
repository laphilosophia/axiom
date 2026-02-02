# Axiom - Document Orchestrator

A local-first document lifecycle management application built with Tauri, Preact, and Rust.

## Features

- **Document Lifecycle Management**: Track documents through Draft → Active → Superseded/Archived states
- **Full-text Search**: Fast, Tantivy-powered search across all documents
- **Semantic Similarity**: ONNX-based embedding analysis for duplicate detection
- **Relationship Tracking**: Graph-based document relationships (supersedes, references)
- **Local-First**: All data stored locally - no cloud dependencies
- **Modern UI**: Dark-themed "Monolith" design with glassmorphism effects

## Tech Stack

| Layer | Technology |
|-------|------------|
| Frontend | Preact + TypeScript |
| Styling | Tailwind CSS |
| UI Components | Radix UI |
| Editor | Milkdown (Markdown) |
| Framework | Tauri v1.x |
| Backend | Rust |
| Database | SurrealDB (Embedded) |
| Search | Tantivy |
| ML | ONNX Runtime |

## Development

### Prerequisites

- Rust 1.75+
- Node.js 20+
- npm 10+

### Setup

```bash
# Install dependencies
npm install

# Run in development mode
cargo tauri dev

# Build for production
cargo tauri build
```

## Architecture

```
axiom/
├── src/                    # Frontend (Preact)
│   ├── components/         # UI components
│   ├── stores/            # State management
│   ├── types/             # TypeScript types
│   └── utils/             # Utilities
├── src-tauri/             # Backend (Rust)
│   ├── src/
│   │   ├── commands/      # Tauri commands
│   │   ├── core/          # Business logic
│   │   ├── db/            # SurrealDB layer
│   │   ├── fs/            # Filesystem management
│   │   ├── ml/            # ONNX Runtime
│   │   └── search/        # Tantivy integration
│   └── Cargo.toml
├── docs/                  # Documentation
└── plans/                 # Implementation plans
```

## Document Model

Documents follow a strict lifecycle:

- **Draft**: Work in progress, editable
- **Active**: Published/documented, editable
- **Superseded**: Replaced by newer version, read-only
- **Archived**: No longer relevant, read-only

## License

MIT
