# binary_air_lab

A verified computer algebra toolkit and binary field arithmetic engine implemented in Rust, designed for binary-tower STARK architectures (Binius64 / generalized AIR).

## Overview

A standalone, dependency-minimalist Rust implementation exploring finite field arithmetic in characteristic 2 ($\mathbb{F}_{2^m}$), quadratic towers, normal bases, and word-level constraints for AIR arithmetization.

The framework bridges formal field theory and type-safe systems programming, enforcing mathematical invariants through Rust's static type system and custom sealed traits.

## Architectural Highlights

- **Field Arithmetic Engine**:
  - Carry-less polynomial arithmetic ($\mathbb{F}_2[T]$), Euclidean division, extended GCD, and modular inversion.
  - Certified irreducibility testing (Rabin's test) for moduli up to degree 64.
  - 128-bit wide polynomial container (`Poly256`) for large carry-less products and flat field model ($\mathbb{F}_{2^{128}}$).
- **Type-Level Tower Extensions**:
  - Recursive Artin-Schreier quadratic towers up to $\mathbb{F}_{2^{128}}$ (`Tower128Bits`).
  - Sealed traits (`QuadraticParameters`) ensuring compile-time irreducibility certificates.
  - Zero-cost abstractions: static dispatch (`Field`, `BinaryFieldElement`) and strictly monomorphized generics.
- **Coordinate Bases & Morphisms**:
  - Basis conversion and linear maps (polynomial basis, normal basis with cyclic Frobenius rotation, recursive tower basis, dual bases via trace pairings).
  - Explicit 128-bit isomorphism between flat polynomial representation and recursive tower construction.
- **Subfields & Relative Extensions**:
  - Linear algebraic kernel construction for intermediate subfields without full-field enumeration.
  - Relative minimal polynomials, sub-extension tracing, and structured norm computation.
- **Word-Level CI/AIR Constraints**:
  - Explicit handling of bitwise shifts, AND-constraints, and multi-bit addition carry witnesses.

## Project Structure

```text
├── src/            # Core library modules (algebra, towers, polynomials, linear solvers)
├── tests/          # Integration test suites for each mathematical stage
└── examples/       # Syntax runners and standalone comprehensive demos