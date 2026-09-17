# binary_toolkit

A Rust toolkit for studying and implementing efficient arithmetic over binary extension fields, with the long-term goal of building the algebraic components of a binary STARK stack.

The project is developed incrementally: each module is introduced only after the corresponding algebra has been derived and understood.

## Current direction

The main path is:

$$
\mathbb F_2
\longrightarrow
\mathbb F_2[T]
\longrightarrow
\mathbb F_{2^{128}}
\longrightarrow
\text{additive FFT}
\longrightarrow
\text{Reed--Solomon / LDE}
\longrightarrow
\text{FRI}
\longrightarrow
\text{binary STARK}.
$$

The project is not intended to become a general-purpose computer algebra system. General algebraic results are studied when needed, while the implementation is specialized toward efficient arithmetic in characteristic two and, in particular, toward a fixed \(128\)-bit binary extension field.

## Implemented

### Bit coordinates

`src/bits.rs`

Basic operations on 64-bit words interpreted as vectors over \(\mathbb F_2\):

* read a bit;
* set a bit;
* toggle a bit;
* convert between `u64` and `[bool; 64]`.

These functions deal only with binary coordinates. They do not define a field multiplication.

### Direct GF(4) reference model

`src/gf4.rs`

A small direct implementation of

$$
\mathbb F_4
=
\mathbb F_2[\alpha]/(\alpha^2+\alpha+1).
$$

It is intentionally simple and independent from the later field implementation, so it can also serve as a small reference model.

Implemented operations:

* addition;
* multiplication.

### Binary polynomials

`src/poly.rs`

Binary polynomials are encoded by storing the coefficient of \(T^i\) in bit \(i\).

Implemented primitives:

* polynomial degree;
* \(64\times64\to128\) carry-less multiplication;
* \(128\times128\to256\) carry-less multiplication.

The current wide multiplication API is

```rust
pub fn clmul128(a: u128, b: u128) -> (u128, u128)
```

with output convention

```text
(lo, hi)
```

representing

$$
A(T)B(T)
=
\operatorname{poly}(\texttt{lo})
+
T^{128}\operatorname{poly}(\texttt{hi}).
$$

The 128-bit multiplication uses a Karatsuba decomposition into three 64-bit carry-less products.

No reduction modulo a field polynomial is performed by `clmul128`.

## Tests

Integration tests are organized by the corresponding study stage:

```text
tests/
├── m00_bits.rs
├── m01_gf4.rs
├── m02_poly.rs
└── m03_product.rs
```

The goal is to verify both concrete examples and the algebraic properties from which each implementation is derived.

Run the complete test suite with:

```bash
cargo test
```

## Next milestone: GF(2^128)

The next step is to turn the wide polynomial product into multiplication in a fixed field

$$
\mathbb F_{2^{128}}
=
\mathbb F_2[T]/(p(T)),
$$

using a fixed irreducible polynomial of degree \(128\).

The intended implementation path is:

```text
clmul128
    ↓
specialized reduction modulo p(T)
    ↓
Gf128 multiplication
```

Because the modulus is fixed in advance, the hot path will use a reduction specialized to that polynomial rather than a generic polynomial-division engine.

The initial `Gf128` backend will provide:

* addition by XOR;
* carry-less multiplication followed by specialized reduction;
* efficient squaring;
* inversion specialized to binary extension fields.

## After GF(2^128)

Once the field backend is complete, development will move directly toward additive FFT algorithms.

Planned progression:

```text
GF(2^128)
    ↓
additive subspaces
    ↓
subspace polynomials
    ↓
reference evaluation/interpolation
    ↓
additive FFT
    ↓
modern Cantor-basis / Frobenius-aware optimizations
    ↓
packed / SIMD implementation
    ↓
Reed-Solomon low-degree extension
    ↓
FRI
    ↓
binary STARK components
```

The project will keep mathematically general explanations where they are useful, but implementation work will remain focused on this pipeline.

## Design principles

* Algebra first: algorithms are derived from the corresponding mathematical identities.
* Binary specialization: exploit characteristic two whenever it reduces the implementation cost.
* No accidental integer arithmetic: polynomial multiplication, field multiplication, integer multiplication and bitwise operations are kept conceptually distinct.
* Independent reference implementations are preferred when they make testing stronger.
* Optimized algorithms must be checked against simpler independent oracles.
* General abstractions are introduced only when they are needed by more than one real implementation.
* Performance work comes after correctness.

## Project structure

```text
.
├── Cargo.toml
├── README.md
├── src/
│   ├── bits.rs
│   ├── gf4.rs
│   ├── lib.rs
│   └── poly.rs
├── tests/
│   ├── m00_bits.rs
│   ├── m01_gf4.rs
│   ├── m02_poly.rs
│   └── m03_product.rs
└── examples/
    └── s00_word.rs
```

Additional modules will be added only when their stage of the mathematical pipeline is reached.

## Status

Current implementation milestone:

$$
\boxed{
\text{binary coordinates}
\rightarrow
\text{binary polynomials}
\rightarrow
\text{wide carry-less multiplication}
}
$$

Next:

$$
\boxed{
\text{specialized reduction}
\rightarrow
\mathbb F_{2^{128}}
}
$$
