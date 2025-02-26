# Session Maker Development Guide

## Build & Run Commands
- `pnpm run dev` - Run frontend dev server
- `pnpm run tauri dev` - Run complete app with hot reload
- `pnpm run build` - Build frontend
- `pnpm run tauri build` - Build complete app
- `pnpm run check` - Type check TypeScript
- `cargo test` - Run Rust tests (in src-tauri directory)
- `cargo test <test_name>` - Run a specific Rust test

## Code Style Guidelines
- **TypeScript**: Strict mode enabled, use explicit types for function parameters and returns
- **Svelte**: Use Svelte 5 features and typed stores
- **Rust**: Follow standard Rust conventions (snake_case, doc comments)
- **Project Structure**:
  - Frontend: SvelteKit with $lib/ for shared code
  - Backend: Tauri with modular Rust components
- **Imports**: Group imports by source (stdlib, external libs, internal)
- **Error Handling**: 
  - TypeScript: Use typed error handling with proper user feedback
  - Rust: Use Result/Option with thiserror for custom errors
- **Naming**: PascalCase for components, camelCase for variables/functions, UPPER_CASE for constants