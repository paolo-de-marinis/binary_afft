# binary_toolkit

A Rust toolkit for studying and implementing efficient arithmetic over binary extension fields, with the long-term goal of building a high-performance additive FFT backend and, eventually, algebraic components for binary STARKs.

The project is developed incrementally: each implementation is introduced only after the corresponding algebra and representation have been derived and understood.

## Current direction

The main path is

\[
\mathbb F_2
\longrightarrow
\mathbb F_2[X]
\longrightarrow
\mathbb F_{2^{128}}
\longrightarrow
\text{Cantor basis}
\longrightarrow
\text{additive FFT}
\longrightarrow
\text{Reed--Solomon / LDE}
\longrightarrow
\text{FRI}
\longrightarrow
\text{binary STARK}.
\]

The project is not intended to become a general-purpose computer algebra system.

The implementation is deliberately specialized toward the fixed binary extension field

\[
\mathbb F_{2^{128}}
=
\mathbb F_2[X]/
\left(
X^{128}+X^7+X^2+X+1
\right),
\]

and toward additive FFT algorithms over binary fields.

General abstractions are introduced only when they are required by the actual implementation.

---

## Architecture

The project separates three different levels:

```text
low-level bit arithmetic
        ↓
field backend
        ↓
Gf128
        ↓
Cantor basis
        ↓
additive FFT
```

The intended long-term source layout is:

```text
src/
├── lib.rs
├── bits.rs
├── polynomial.rs
│
├── field/
│   ├── mod.rs
│   ├── gf128.rs
│   ├── portable.rs
│   └── pclmul.rs
│
├── cantor.rs
│
└── afft/
    ├── mod.rs
    ├── naive.rs
    ├── general.rs
    └── dyadic.rs
```

Not all of these files exist yet.
Files are added only when the corresponding development stage is reached.

The central architectural principle is:

```text
Gf128
  ↓
Cantor basis
  ↓
AFFT
```

Code above `Gf128` should not need to know whether field multiplication is implemented by portable Rust code, `PCLMULQDQ`, `VPCLMULQDQ`, or another backend.

## Current implementation

### Bit coordinates

`src/bits.rs`

Basic operations on `u64` words interpreted as vectors over \(\mathbb F_2\):

* read a bit;
* set a bit;
* toggle a bit;
* convert between `u64` and `[bool; 64]`.

These functions manipulate binary coordinates only.
They do not define polynomial or field multiplication.

### Direct GF(4) model

`src/gf4.rs`

A small didactic implementation of

\[
\mathbb F_4 = \mathbb F_2[\alpha_4]/(\alpha_4^2+\alpha_4+1).
\]

It is intentionally independent from the later `Gf128` implementation.

Implemented operations:

* addition;
* multiplication.

This module is mainly a small reference model for understanding how a polynomial relation induces field arithmetic.

### Polynomial utilities

`src/polynomial.rs`

This module contains operations that conceptually belong to polynomials rather than to a particular field-arithmetic backend.

Currently implemented:

```rust
pub fn poly_degree(p: u128) -> Option<usize>
```

A `u128` is interpreted as the coefficient vector of a binary polynomial

\[
a_0+a_1X+\cdots+a_{127}X^{127}, \qquad a_i\in\mathbb F_2,
\]

with bit \(i\) storing coefficient \(a_i\).

The module is intentionally small for now.
Future polynomial operations will be added only when they are needed by the additive FFT implementation.
No generic `Polynomial<F>` abstraction is currently planned.

### Portable GF(2^128) backend

`src/field/portable.rs`

This module contains the current pure-Rust low-level arithmetic used to build multiplication in the fixed binary field.
It works directly with `u64` and `u128` words.

#### 64 × 64 carry-less multiplication

```rust
pub fn clmul(a: u64, b: u64) -> u128
```

This computes multiplication in \(\mathbb F_2[X]\), not integer multiplication.

If

\[
a(X)=\sum_i a_iX^i, \qquad b(X)=\sum_j b_jX^j,
\]

then the coefficient of \(X^k\) in the result is

\[
\bigoplus_{i+j=k}(a_i\wedge b_j).
\]

No carries are generated.

#### 128 × 128 carry-less multiplication

```rust
pub fn clmul128(a: u128, b: u128) -> (u128, u128)
```

The output convention is

```text
(lo, hi)
```

and represents the complete polynomial product

\[
ab=(ab)_0+(ab)_1X^{128},
\]

where:

* `lo` encodes \((ab)_0\);
* `hi` encodes \((ab)_1\).

The implementation splits both operands into 64-bit halves and uses Karatsuba:

\[
a=a_0+a_1X^{64}, \qquad b=b_0+b_1X^{64},
\]

\[
p_0=a_0b_0, \qquad p_1=a_0b_1+a_1b_0, \qquad p_2=a_1b_1.
\]

Instead of computing four carry-less products directly,

\[
p_1 = (a_0+a_1)(b_0+b_1)+p_0+p_2,
\]

so only three calls to the 64-bit carry-less multiplication are required.

`clmul128` does not perform field reduction.

#### Specialized reduction

The field modulus is fixed to

\[
p_{128}(X) = X^{128}+X^7+X^2+X+1.
\]

The portable backend implements

```rust
pub fn reduce_p128(lo: u128, hi: u128) -> u128
```

which reduces the polynomial represented by

```text
(lo, hi)
```

modulo \(p_{128}\).

The key relation is

\[
X^{128} \equiv X^7+X^2+X+1 \pmod{p_{128}}.
\]

Because the low part of the modulus is sparse, reduction can be implemented using only shifts and XORs.

The optimized implementation is checked against an independent slow reference based on coefficient-by-coefficient polynomial reduction.

### Current module tree

After the current structural refactor:

```text
src/
├── bits.rs
├── gf4.rs
├── lib.rs
├── polynomial.rs
│
└── field/
    ├── mod.rs
    └── portable.rs
```

with

```rust
// src/lib.rs

pub mod bits;
pub mod field;
pub mod gf4;
pub mod polynomial;
```

and

```rust
// src/field/mod.rs

pub mod portable;
```

At this stage there is deliberately no `field/gf128.rs` yet.

## Tests

Integration tests are named after the component they exercise, not after a study chapter:

```text
tests/
├── common/
│   ├── slow_clmul.rs
│   └── slow_reduce.rs
│
├── bits.rs
├── gf4.rs
├── polynomial.rs
└── field_portable.rs
```

`field_portable.rs` covers the whole portable backend: `clmul`, `clmul128` and `reduce_p128`.

The slow implementations under `tests/common/` are deliberately simple independent oracles.
They are not production backends.

### Carry-less multiplication oracle

`slow_clmul` implements the binary convolution directly, coefficient by coefficient.
It is used to verify both `clmul` and `clmul128`.

### Reduction oracle

`slow_reduce` represents the complete polynomial with 256 boolean coefficients and performs the reduction one high-degree term at a time.
It is intentionally structurally different from the optimized two-fold reduction.

Run all tests with:

```bash
cargo test
```

Correctness is established incrementally before optimization work is introduced.

## Next milestone: the `Gf128` type

The next source file will be

```text
src/field/gf128.rs
```

and will introduce the mathematical field-element type

```rust
pub struct Gf128(u128);
```

The purpose of this type is to separate:

```text
u128
```

as a raw machine word from

```text
Gf128
```

as an element of the fixed field.

The intended API will include operations such as:

```text
ZERO
ONE

from_u128
to_u128

add
mul
square
inverse
```

and later the corresponding standard Rust operator traits.

`Gf128` should not contain Karatsuba or reduction logic directly.

Conceptually:

```text
Gf128(a)
   │
   │ multiplication
   ▼
field backend
   │
   ▼
Gf128(c)
```

The current backend is `field::portable`.

## Future hardware backend

A later stage will introduce

```text
src/field/pclmul.rs
```

using CPU carry-less multiplication instructions such as `PCLMULQDQ` and, where appropriate, wider vector variants.

The portable and hardware implementations should expose the same internal field-arithmetic contract.
Higher-level code must not contain branches such as:

```text
if pclmul {
    ...
} else {
    ...
}
```

Backend selection belongs below `Gf128`.
The additive FFT should only see field operations.

## Cantor basis

After the field backend is complete, the project will introduce

```text
src/cantor.rs
```

for the Cantor special basis and associated binary subspaces.

The relevant basis relation is

\[
\beta_0=1, \qquad \beta_i^2+\beta_i=\beta_{i-1}.
\]

The subspaces are

\[
W_i = \operatorname{span}_{\mathbb F_2} \{\beta_0,\ldots,\beta_{i-1}\}.
\]

Initially this will remain a single module.
It will be split into a directory only if the implementation becomes large enough to justify separate components.

## Additive FFT

The additive FFT layer is planned as:

```text
src/afft/
├── mod.rs
├── naive.rs
├── general.rs
└── dyadic.rs
```

### `naive.rs`

A deliberately simple direct-evaluation implementation.
Its main purpose is correctness checking.

### `general.rs`

The general recursive AFFT stage used as the reference optimized algorithm before the dyadic specialization.

### `dyadic.rs`

The dyadic AFFT specialization that is the main optimization target of the project.

The intended verification hierarchy is:

```text
direct evaluation
       ↑
      test
       │
general AFFT
       ↑
      test
       │
dyadic AFFT
```

This makes it easier to localize failures between:

* field arithmetic;
* Cantor-basis construction;
* the general AFFT;
* the dyadic specialization.

## Design principles

* Algebra first. Algorithms are derived from mathematical identities before implementation.
* Specialize deliberately. The current target is `GF(2^128)`, not an arbitrary runtime-defined field.
* Keep representations separated. Integer multiplication, carry-less polynomial multiplication and field multiplication are different operations.
* Hide backend details below `Gf128`. Cantor and AFFT code should operate on field elements, not inspect raw bits.
* Prefer independent test oracles. Optimized code is checked against simpler implementations with different structure.
* Avoid premature generic abstractions. No generic field hierarchy or generic polynomial container is introduced until a concrete use requires it.
* Correctness before performance. Hardware acceleration is added only after the portable implementation is verified.
* Do not create empty architecture. Planned files are added when their implementation stage begins.

## Development roadmap

```text
binary coordinates
        ↓
binary polynomial representation
        ↓
portable carry-less multiplication
        ↓
specialized GF(2^128) reduction
        ↓
Gf128 type
        ↓
specialized squaring
        ↓
Itoh–Tsujii inversion
        ↓
hardware PCLMUL backend
        ↓
Cantor special basis
        ↓
naive additive evaluation
        ↓
general AFFT
        ↓
dyadic AFFT
        ↓
Reed–Solomon / LDE
        ↓
FRI
        ↓
binary STARK components
```

## Status

Currently implemented:

\[
\boxed{
\mathbb F_2
\longrightarrow
\mathbb F_2[X]
\longrightarrow
\text{wide carry-less product}
\longrightarrow
\text{specialized reduction modulo }p_{128}
}
\]

Current code milestone:

```text
bits
  ↓
clmul
  ↓
clmul128
  ↓
reduce_p128
```

Next:

\[
\boxed{
\texttt{Gf128}
\longrightarrow
\texttt{square}
\longrightarrow
\texttt{inverse}
}
\]

After the field API is complete, development moves toward the Cantor basis and the dyadic additive FFT.
