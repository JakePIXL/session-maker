# Session Maker

A desktop application for session management built with Tauri, SvelteKit, and TypeScript.

## About

This project was created to test the capabilities of Claude Code/Claude Sonnet 3.7 Thinking. It demonstrates how to build a cross-platform desktop application with a modern frontend framework and Rust backend.

## Features

- Global hotkey support for quick session creation
- Session history tracking and visualization
- System tray integration
- Cross-platform (macOS, Windows, Linux)

## Development

### Prerequisites

- Node.js (v18+)
- Rust (latest stable)
- pnpm

### Setup

```bash
# Install dependencies
pnpm install
```

### Development Commands

```bash
# Run frontend dev server only
pnpm run dev

# Run complete app with hot reload
pnpm run tauri dev

# Type check
pnpm run check

# Run Rust tests (in src-tauri directory)
cargo test
```

### Building

```bash
# Build frontend
pnpm run build

# Build complete app
pnpm run tauri build
```

## Project Structure

- `src/` - SvelteKit frontend code
  - `lib/` - Shared code and components
  - `routes/` - SvelteKit routes
- `src-tauri/` - Rust backend code
  - `src/` - Main Rust source files
  - `Cargo.toml` - Rust dependencies

## Code Style Guidelines

- **TypeScript**: Strict mode enabled, use explicit types
- **Svelte**: Svelte 5 features and typed stores
- **Rust**: Follow standard Rust conventions (snake_case, doc comments)
- **Error Handling**:
  - TypeScript: Use typed error handling with proper user feedback
  - Rust: Use Result/Option with thiserror for custom errors

## License

MIT
