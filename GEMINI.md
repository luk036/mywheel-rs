# GEMINI.md: Project Overview and Development Guide

## Project Overview

This project, `mywheel-rs`, is a Rust library focused on implementing various data structures, essentially "reinventing the wheel" for learning and demonstration purposes. The library includes modules for data structures such as a doubly linked list (`dllist`), a priority queue (`bpqueue`), and others. It is set up as a library crate, with a simple `main.rs` for basic checks.

**Key Technologies:**
*   Language: Rust
*   Build Tool: Cargo

**Project Structure:**
*   `src/lib.rs`: The main library crate entry point, exporting the data structure modules.
*   `src/main.rs`: A simple binary entry point.
*   `src/`: Contains the implementations of the different data structures.
*   `Cargo.toml`: The package manifest defining project metadata and dependencies.
*   `.github/workflows/`: Contains CI/CD workflows for testing, formatting, and linting.

## Building and Running

The following commands are essential for developing and interacting with this project.

### Running Tests
To execute the entire test suite:
```sh
cargo test --all-features --workspace
```

### Building the Project
To build the project in release mode:
```sh
cargo build --release
```

### Running the Executable
To run the simple executable included in the project:
```sh
cargo run --release
```

## Development Conventions

The project follows standard Rust development practices and uses CI workflows to enforce them.

### Code Formatting
The project uses `rustfmt` for code formatting. To check for formatting issues:
```sh
cargo fmt --all --check
```
To automatically format the code:
```sh
cargo fmt --all
```

### Linting
The project uses `clippy` for linting. To run the linter:
```sh
cargo clippy --all-targets --all-features --workspace
```

### Documentation
The project uses `rustdoc` to generate and check documentation. To check for documentation warnings:
```sh
cargo doc --no-deps --document-private-items --all-features --workspace --examples
```

### Contribution Guidelines
Contributions should follow the guidelines in `CONTRIBUTING.md`. Key points include:
*   Discussing significant changes by creating an issue before starting work.
*   Creating one pull request per change.
*   Updating the `CHANGELOG.md` file for any user-facing changes.
