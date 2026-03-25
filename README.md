# Acheron

[![Rust](https://github.com/RobbieMcKinstry/acheron/actions/workflows/rust.yml/badge.svg)](https://github.com/RobbieMcKinstry/acheron/actions/workflows/rust.yml)

Acheron is a SAT solver written in Rust.
It implements DPLL with pure-literal elimination.

## Usage

Solve a single DIMACS CNF file:

```sh
cargo run --release -- path/to/problem.cnf
```

## Throughput Benchmark

Measure how many problems the solver can handle in 60 seconds:

```sh
cargo run --release --bin throughput
```

This loads CNF files from `examples/satisfiable/uniform-random-3sat/`, sorted by problem size (uf20 through uf250), and solves them sequentially under a 60-second timer.

