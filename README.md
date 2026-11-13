# rust-libs

Shared Rust crates for gerontosepp projects. Each crate is domain-neutral and can be used
independently by the Vinyl Tracker, the party/DJ app and future projects.

## Crates

| Crate | Purpose |
| :--- | :--- |
| `ai-compat` | OpenAI-compatible chat client with provider presets, tool calls, model list and connection test. |

## Workspace conventions

- Rust edition 2024, workspace MSRV 1.90 (the pinned toolchain is 1.100.0).
- `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings` and
  `cargo test --workspace` must pass.
- Crates must not depend on domain crates of a consuming application.
- Breaking changes get a new workspace version and an entry in `CHANGELOG.md`.

## Consuming a crate

```toml
ai-compat = { git = "https://github.com/gerontosepp/rust-libs", tag = "v0.1.0" }
```

Pin a tag or revision, never a moving branch.
