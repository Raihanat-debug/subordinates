# Subordinates

A Rust implementation of the CSES "Subordinates" problem that computes, for each employee, the number of subordinates in a company tree.

## Overview

The program reads a rooted tree from standard input:

- the first number is the number of employees $n$
- for employees $2$ to $n$, the next values give the parent of each employee

Two solutions are included:

- Recursive DFS in [src/bin/recursive_dfs.rs](src/bin/recursive_dfs.rs)
- Iterative DFS in [src/bin/iterative_dfs.rs](src/bin/iterative_dfs.rs)

Both produce the same output format: one integer per employee, representing the number of subordinates.

## Example

Input:

```text
5
1 1 2 3
```

Output:

```text
4 1 1 0 0
```

## Project structure

- [src/lib.rs](src/lib.rs) — library placeholder for the crate
- [src/bin/recursive_dfs.rs](src/bin/recursive_dfs.rs) — recursive solution
- [src/bin/iterative_dfs.rs](src/bin/iterative_dfs.rs) — iterative solution
- [tests/](tests/) — integration tests for both implementations
- [benchmark.md](benchmark.md) — explanation and comparison of the two approaches

## Build and run

Build the project:

```bash
cargo build
```

Run the recursive version:

```bash
cargo run --bin recursive_dfs
```

Run the iterative version:

```bash
cargo run --bin iterative_dfs
```

## Test

Run the test suite:

```bash
cargo test
```

## Notes

The repository also includes a benchmark write-up comparing recursive and iterative DFS approaches for this problem.
