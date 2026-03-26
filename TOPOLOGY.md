<!-- SPDX-License-Identifier: PMPL-1.0-or-later -->
<!-- TOPOLOGY.md — Project architecture map and completion dashboard -->
<!-- Last updated: 2026-02-20 -->

# intsoc-transactor — Project Topology

## System Architecture

```
                    ┌─────────────────────────────────┐
                    │         intsoc-cli (binary)      │
                    │   check │ fix │ submit │ status  │
                    └──────┬──┴──┬──┴──┬──┴──┬────────┘
                           │     │     │     │
            ┌──────────────┼─────┼─────┼─────┼──────────────┐
            │              ▼     ▼     ▼     ▼              │
            │  ┌──────────────────────────────────────────┐ │
            │  │              intsoc-core                  │ │
            │  │  Organization ─ Stream ─ Document ─ Fix  │ │
            │  │  StateMachine ─ Validation ─ Transaction  │ │
            │  └──────────────────────────────────────────┘ │
            │       ▲        ▲        ▲        ▲            │
            │       │        │        │        │            │
            │  ┌────┴───┐┌───┴────┐┌──┴──┐┌───┴───┐       │
            │  │ parser  ││ fixer  ││ git ││  api  │       │
            │  │ XML v3  ││ engine ││ gix ││ DT+   │       │
            │  │ plain   ││ 6 gens ││     ││ IANA  │       │
            │  │ idnits  ││ diff   ││     ││       │       │
            │  └────┬────┘└────┬───┘└──┬──┘└───┬───┘       │
            │       │          │       │       │            │
            │       ▼          ▼       ▼       ▼            │
            │  ┌──────────────────────────────────┐        │
            │  │          intsoc-nickel            │        │
            │  │  contracts ─ templates ─ policies │        │
            │  └──────────────────────────────────┘        │
            │                                               │
            │  R u s t   W o r k s p a c e                 │
            └───────────────────────────────────────────────┘

            ┌───────────────────────────────────────────────┐
            │              GUI (Gossamer)                    │
            │  ┌──────────────────────────────────────────┐ │
            │  │         ReScript TEA Frontend            │ │
            │  │  Editor ─ Checker ─ Fixer ─ Submitter    │ │
            │  └──────────────────────────────────────────┘ │
            │  ┌──────────────────────────────────────────┐ │
            │  │          backend/ (Rust+Gossamer)         │ │
            │  │  Gossamer commands ←→ core/parser/fixer   │ │
            │  └──────────────────────────────────────────┘ │
            └───────────────────────────────────────────────┘

            ┌─────────────────────┐  ┌──────────────────────┐
            │  Haskell (Phase 2)  │  │ Idris2+Zig (Phase 3) │
            │  megaparsec ABNF    │  │ ABI Types ─ Zig FFI  │
            │  RFC XML deep       │  │ Parser bridge         │
            └─────────────────────┘  └──────────────────────┘

                         ┌──────────────────────┐
            External:    │  IETF Datatracker API │
                         │  IANA Registry API    │
                         │  RFC Editor API       │
                         └──────────────────────┘
```

## Completion Dashboard

| Component              | Progress                      | Status      |
|------------------------|-------------------------------|-------------|
| intsoc-core            | `████████░░` 80%              | Phase 1 MVP |
| intsoc-parser          | `████████░░` 80%              | Phase 1 MVP |
| intsoc-fixer           | `███████░░░` 70%              | Phase 1 MVP |
| intsoc-nickel          | `█████░░░░░` 50%              | Phase 1 MVP |
| intsoc-cli             | `██████░░░░` 60%              | Phase 1 MVP |
| intsoc-git             | `███░░░░░░░` 30%              | Phase 1     |
| intsoc-api             | `████░░░░░░` 40%              | Phase 1     |
| GUI (Gossamer+ReScript)| `██░░░░░░░░` 20%              | Phase 1     |
| Nickel templates       | `█████░░░░░` 50%              | Phase 1     |
| Haskell parser         | `░░░░░░░░░░` 5%               | Phase 2     |
| Idris2 ABI + Zig FFI   | `░░░░░░░░░░` 5%               | Phase 3     |
| Tests                  | `█░░░░░░░░░` 10%              | Ongoing     |
| RSR compliance         | `████████░░` 80%              | Ongoing     |

## Key Dependencies

| Dependency   | Version | Purpose                           |
|-------------|---------|-----------------------------------|
| winnow      | 0.6     | Plain-text document parsing       |
| quick-xml   | 0.37    | RFC XML v3 parsing (read+write)   |
| similar     | 2       | Unified diff generation           |
| gix         | 0.68    | Git integration (pure Rust)       |
| reqwest     | 0.12    | HTTP client (rustls-tls)          |
| clap        | 4       | CLI argument parsing              |
| gossamer-rs | 0.1     | Gossamer webview shell bindings   |
| nickel      | CLI     | Template rendering + contracts    |
| megaparsec  | 9.6     | Haskell parser combinators (Ph.2) |
