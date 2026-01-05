# trace-doc

A Rust-based framework for compile-time, zero-cost traceability and specification modeling.

## Overview

`trace-doc` enables you to model requirements, realizations, and tests directly in Rust's type system. It provides zero-runtime-cost traceability for safety-critical and high-assurance software.

## Features

- **Compile-time traceability**: Link requirements, realizations, and tests using Rust traits.
- **Zero runtime cost**: All traceability is enforced at compile time.

### Future Feature ideas

- **Artifact Model Extraction & Visualization**: Extract types and their trait implementations (e.g., using rustdoc's JSON format) to reconstruct the artifact model. This will enable generating a complete traceability graph of requirements, realizations, and tests.
- **Rustdoc Integration**: Display the reconstructed artifact model and traceability graph directly in the generated rustdoc HTML, making documentation interactive and navigable.
- **Sphinx-Needs Export**: Export requirements, realizations, tests, and interfaces to [Sphinx-Needs](https://sphinx-needs.readthedocs.io/) for integration with Sphinx-based documentation workflows.

## Example

```rust
/// The system shall be able to calculate the mathematical constant π.
pub struct ReqCalculatePI;
impl Artifact for ReqCalculatePI {
    const STATUS: Status = Status::Accepted;
}
impl Requirement for ReqCalculatePI {
    const TITLE: &str = "Calculate mathematical constant π";
}

/// This realization implements the calculation of π.
pub struct PICalculation; // implements pub fn calculate_pi() -> f64
impl Artifact for PICalculation {}
impl Realization for PICalculation {}
impl Realizes<ReqCalculatePI> for PICalculation {}
impl PICalculation {
    /// Calculates PI
    pub fn calculate_pi() -> f64 {
        // let's cheat a bit and return the PI constant directly for simplicity
        std::f64::consts::PI
    }
}
```

## How to Use

1. Model your requirements, realizations, and tests as Rust types and traits.
2. Use the provided traits to link them (e.g., `Realizes`, `Tests`).
3. Build your project as usual. The Rust compiler enforces traceability at compile time.

## Related Traceability Tools

**[mantra](https://crates.io/crates/mantra)**: An alternative approach to requirement tracing that uses string-based requirement IDs with automatic source code scanning and database-backed reporting.
