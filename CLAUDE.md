# CLAUDE.md

Lycoris is a template: a Rust core compiled to WebAssembly, driven by a
TypeScript playground (Vite, optional Tauri desktop build). It was cut down
from the Ajisai language repository so the stack and the playground GUI can
be reused in other projects. See `README.md` for the layout.

- The Rust core in `rust/src/` is a deliberately tiny sample language. Keep it
  small; it exists to exercise every playground surface.
- The contract between Rust and TypeScript is `src/wasm-interpreter-types.ts`.
  `rust/src/wasm.rs` must return the shapes declared there.
- After changing Rust code, run `npm run build:wasm` and commit
  `src/wasm/generated/`.
- Checks: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`
  and `cargo test` in `rust/`; `npm run check`, `npm run lint`, `npm test`
  and `npm run check:contrast` at the root.
