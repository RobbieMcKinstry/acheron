# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Acheron is a DPLL SAT solver written in Rust. It takes DIMACS CNF files as input and determines satisfiability using unit propagation (pure literal elimination is stubbed but not yet implemented).

## Build & Development Commands

This project uses `cargo-make` for task orchestration. Install it with `cargo install cargo-make`.

- **Full dev pipeline** (format, lint, build, docs, test): `cargo make dev-test-flow`
- **Build**: `cargo build`
- **Test all**: `cargo test`
- **Test single**: `cargo test <test_name>`
- **Format** (requires nightly): `cargo +nightly fmt`
- **Lint**: `cargo clippy -- --no-deps`
- **Coverage** (Docker-based): `cargo make cov`
- **Sort Cargo.toml deps**: `cargo make sort`
- **CI pipeline**: `cargo make ci-flow` (runs in a nightly Docker container)
- **Throughput benchmark**: `cargo run --release --bin throughput` — solves as many CNF problems as possible in 60 seconds, starting with the smallest (uf20) and progressing to larger instances. Outputs JSON (`{"solved":N,"elapsed_secs":X.XX}`) to stdout; human-readable progress goes to stderr.
- **Compare throughput across versions**: `cargo make benchmark-compare` — runs the throughput benchmark for each git tag listed in the `TAGS` array in `benchmarks/run-benchmarks.sh`, then generates an SVG bar chart at `benchmarks/throughput.svg` using D3 (requires `cd benchmarks && npm install` first).

## Architecture

### Solving Pipeline

The solver uses a work queue (not recursion) to explore the DPLL search tree:

1. `Solver` initializes with a `History` containing the input formula
2. `DecisionEngine` consults an ordered `decision_table` of `OpMaker`s
3. The first `OpMaker` that returns `Some(Operator)` wins (priority order: SAT check → UNSAT check → unit propagation → pure literal → splitting)
4. The `Operator` is wrapped in a `Job` and pushed onto the `WorkQueue`
5. Jobs are popped and applied; results are checked for termination (`Sat`/`Unsat`/`Unfinished`)

### Key Abstractions

- **`Operator` trait** (`ops/operator.rs`): Defines `apply()` — the interface for all formula transformations
- **`OpMaker` trait** (`ops/maker.rs`): Factory that conditionally constructs an `Operator` based on formula state. Returns `None` if the operation doesn't apply.
- **`DecisionEngine`** (`engine/decision_engine.rs`): Holds the ordered decision table of `OpMaker`s and selects which operation to apply next
- **`History`** (`work_queue/history.rs`): Immutable chain of formula states linked via `Arc<History>` parent pointers for backtracking without copying

### Type Distinctions

- **`Literal`** = variable + sign (polarity within a clause)
- **`Condition`** = truth assignment to a variable (semantically distinct from Literal)
- **`Formula`** = `im::Vector<Clause>` (immutable vector for structural sharing)
- **`Clause`** = `im::Vector<Literal>`

The `im` crate provides persistent/immutable data structures throughout, enabling efficient cloning and backtracking via structural sharing.

### Module Layout

- `core/` — SAT primitives: `Formula`, `Clause`, `Literal`, `Variable`, `Sign`, `Condition`, `Status`
- `ops/` — Operations: `Operator`/`OpMaker` traits, implementations in `sat/`, `unsat/`, `unit_prop/`, `pure_lit/`, `cond_app/`
- `engine/` — `DecisionEngine` with priority-ordered decision table
- `work_queue/` — `WorkQueue`, `Job`, `JobOutput`, `History`, `TerminationState`
- `parser.rs` — DIMACS CNF format parser

## Conventions

- Trait object safety is verified at compile time with `static_assertions::assert_obj_safe!`
- Tests use `pretty_assertions` for readable diffs
- Formatting uses nightly rustfmt
- Cargo.toml dependencies must be sorted (`cargo make sort`)
