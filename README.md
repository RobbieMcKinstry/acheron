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

## Comparing Throughput Across Versions

To track solver performance across tagged releases:

1. Edit the `TAGS` array at the top of `benchmarks/run-benchmarks.sh` with the git tags you want to compare.
2. Install the visualization dependencies:
   ```sh
   cd benchmarks && npm install
   ```
3. Run the comparison (requires a clean working tree):
   ```sh
   cargo make benchmark-compare
   ```

This checks out each tag, builds and runs the throughput benchmark, then generates an SVG bar chart at `benchmarks/throughput.svg`.
