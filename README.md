# binary_afft

A Rust toolkit for studying and implementing efficient arithmetic over binary extension fields, with the goal of building a high-performance additive FFT backend over $\mathbb F_{2^{128}}$.

The project is developed incrementally: each implementation is introduced only after the corresponding algebra and representation have been derived and understood.

## Current direction

The main path is

```math
\mathbb F_2
\longrightarrow
\mathbb F_2[X]
\longrightarrow
\mathbb F_{2^{128}}
\longrightarrow
\text{Cantor basis}
\longrightarrow
\text{additive FFT}.
```

The project is not intended to become a general-purpose computer algebra system.

The implementation is deliberately specialized toward the fixed binary extension field

```math
\mathbb F_{2^{128}}
=
\mathbb F_2[X]/
\left(
X^{128}+X^7+X^2+X+1
\right),
```

and toward additive FFT algorithms over binary fields.

General abstractions are introduced only when they are required by the actual implementation.

## Documentation

The current English study notes are available in:

`docs/binary_fields_rust.pdf`

They develop the algebra, representation, implementation, verification, and source-level constant-time discipline used by the project.

---

## Architecture

The implementation is organized in layers:

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

The intended source layout is:

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

The central architectural path is:

```text
Gf128
  ↓
Cantor basis
  ↓
AFFT
```

Code above `Gf128` should not need to know whether field multiplication is implemented by portable Rust code, `PCLMULQDQ`, `VPCLMULQDQ`, or another backend.

---

## Current implementation

### Bit coordinates

`src/bits.rs`

Basic operations on `u64` words interpreted as vectors over $\mathbb F_2$:

- read a bit as a Boolean predicate with `get_bit`;
- read a bit as a numeric control word with `get_bit_u128`;
- set a bit;
- toggle a bit;
- convert between `u64` and `[bool; 64]`.

The Boolean API remains useful for ordinary coordinate manipulation, while `get_bit_u128` is used where a sensitive bit must stay as data instead of becoming control flow.

These functions manipulate binary coordinates only.

They do not define polynomial or field multiplication.

### Direct GF(4) model

`examples/gf4.rs`

A small didactic implementation of

```math
\mathbb F_4
=
\mathbb F_2[\alpha_4]/
(\alpha_4^2+\alpha_4+1).
```

It is intentionally independent from the later `Gf128` implementation.

Implemented operations:

- addition;
- multiplication.

It is a small reference model for understanding how a polynomial relation induces field arithmetic, kept as a Cargo example. It is not part of the library and not exposed by the public API; its tests live in the example itself and run with `cargo test --example gf4`.

### Polynomial utilities

`src/polynomial.rs`

This module contains operations that conceptually belong to polynomials rather than to a particular field-arithmetic backend.

Currently implemented:

```rust
pub fn poly_degree(p: u128) -> Option<usize>
```

A `u128` is interpreted as the coefficient vector of a binary polynomial

```math
a_0+a_1X+\cdots+a_{127}X^{127},
\qquad
a_i\in\mathbb F_2,
```

with bit $i$ storing coefficient $a_i$.

The module is intentionally small for now.

Future polynomial operations will be added only when they are required by the additive FFT implementation.

No generic `Polynomial<F>` abstraction is introduced at this stage.

---

## Portable GF(2^128) backend

`src/field/portable.rs`

This module contains the current pure-Rust low-level arithmetic used to build multiplication in the fixed binary field.

It works directly with `u64` and `u128` words.

### 64 × 64 carry-less multiplication

```rust
pub fn clmul(a: u64, b: u64) -> u128
```

This computes multiplication in $\mathbb F_2[X]$, not integer multiplication.

If

```math
a(X)=\sum_i a_iX^i,
\qquad
b(X)=\sum_j b_jX^j,
```

then the coefficient of $X^k$ in the result is

```math
\bigoplus_{i+j=k}(a_i\wedge b_j).
```

No carries are generated.

The current implementation always performs the same 64 loop iterations. Each bit of `b` is read through `get_bit_u128`, expanded to a full-word mask with `wrapping_sub`, and used to select the corresponding shifted contribution without an explicit data-dependent branch.

The earlier branch-based implementation is retained only under `tests/common/clmul_vartime.rs` as a functional regression reference.

### 128 × 128 carry-less multiplication

```rust
pub fn clmul128(a: u128, b: u128) -> (u128, u128)
```

The output convention is

```text
(lo, hi)
```

and represents the complete polynomial product

```math
ab=(ab)_0+(ab)_1X^{128},
```

where:

- `lo` encodes $(ab)_0$;
- `hi` encodes $(ab)_1$.

The implementation splits both operands into 64-bit halves:

```math
a=a_0+a_1X^{64},
\qquad
b=b_0+b_1X^{64}.
```

Define

```math
p_0=a_0b_0,
\qquad
p_1=a_0b_1+a_1b_0,
\qquad
p_2=a_1b_1.
```

Karatsuba computes the middle term as

```math
p_1
=
(a_0+a_1)(b_0+b_1)+p_0+p_2,
```

so only three 64-bit carry-less products are required.

`clmul128` does not perform field reduction.

### Specialized reduction

The field modulus is fixed to

```math
p_{128}(X)
=
X^{128}+X^7+X^2+X+1.
```

The portable backend implements

```rust
pub fn reduce_p128(lo: u128, hi: u128) -> u128
```

which reduces the polynomial represented by `(lo, hi)` modulo $p_{128}$.

The key relation is

```math
X^{128}
\equiv
X^7+X^2+X+1
\pmod{p_{128}}.
```

Because the low part of the modulus is sparse, reduction can be implemented using only shifts and XORs.

The optimized implementation is checked against an independent slow reference based on coefficient-by-coefficient polynomial reduction.

### Field product on words

```rust
pub fn mul_p128(a: u128, b: u128) -> u128
```

This composes the two primitives above: `clmul128` builds the wide product, `reduce_p128` brings it back below degree 128. Its output is already the canonical representative, so it is the word-level operation the field type delegates to.

### Squaring by bit spreading

In characteristic two every mixed term of a square cancels, so

```math
A(X)^2=\sum_{i=0}^{127}c_iX^{2i}.
```

No coefficient has to be computed: each one only moves from index $i$ to index $2i$, and every odd position becomes zero.

```rust
pub fn spread(a: u64) -> u128
pub fn spread128(a: u128) -> (u128, u128)
pub fn square_p128(a: u128) -> u128
```

`spread` performs that relocation for one 64-bit half in six masking stages, with $s=32,16,8,4,2,1$:

```math
z \leftarrow (z \vee (z \ll s)) \wedge M_s.
```

The masks alternate $s$ zeros and $s$ ones and are computed once as compile-time constants from the identity

```math
M_s=\frac{2^{128}-1}{2^{s}+1}.
```

`spread128` splits the operand into halves and spreads each one, which is enough because the cross term of a square vanishes:

```math
A(X)^2=a_0(X)^2+a_1(X)^2X^{128}.
```

`square_p128` then reduces that pair. Spreading replaces the wide product, not the reduction.

---

## Current module tree

The current source tree is:

```text
src/
├── bits.rs
├── lib.rs
├── polynomial.rs
│
└── field/
    ├── mod.rs
    ├── gf128.rs
    └── portable.rs
```

`src/lib.rs` currently exposes:

```rust
pub mod bits;
pub mod field;
pub mod polynomial;
```

and `src/field/mod.rs` contains:

```rust
mod gf128;
pub mod portable;

pub use gf128::Gf128;
```

The `gf128` module itself is private: the type reaches callers through the re-export, as `binary_afft::field::Gf128`. The backend stays public so the tests and future benchmarks can measure it directly.

---

## Tests

Integration tests are named after the component they exercise:

```text
tests/
├── common/
│   ├── clmul_vartime.rs
│   ├── slow_clmul.rs
│   └── slow_reduce.rs
│
├── bits.rs
├── polynomial.rs
├── field_portable.rs
└── gf128.rs
```

`field_portable.rs` covers the word-level backend: `clmul` and `clmul128` against the convolution oracle, `clmul` against the earlier branch-based implementation, `reduce_p128` against the reduction oracle, and `spread`.

`gf128.rs` covers the public type: construction, constants and operators on fixed values; `square` against `a * a`; inversion through a `CtOption<Gf128>`; the invalid zero case; constant-time equality; and conditional selection. It compares the type against operations that are already verified, rather than repeating the oracles.

The slow implementations under `tests/common/` are deliberately simple independent oracles.

They are not production backends.

### Carry-less multiplication oracle

`slow_clmul` implements the binary convolution directly, coefficient by coefficient.

It is used to verify both `clmul` and `clmul128`.

### Reduction oracle

`slow_reduce` represents the complete polynomial with 256 Boolean coefficients and reduces one high-degree term at a time.

It is intentionally structurally different from the optimized two-fold reduction.

Run the complete test suite with:

```bash
cargo test
```

Correctness is established incrementally before optimization work is introduced.

---

## The `Gf128` type

`src/field/gf128.rs`

```rust
pub struct Gf128(u128);
```

A tuple struct with a private field: the representation is still one `u128`, but the type separates a raw machine word from an element of the field. `from_u128` and `to_u128` are the only way across that boundary, and neither changes basis nor reduces.

It derives `Clone`, `Copy`, `Debug`, `PartialEq` and `Eq`. Copies are implicit because the only datum is a word; equality compares that word, which is meaningful because representatives are canonical.

### Constants and conversions

```rust
pub const ZERO: Self;       // word 0
pub const ONE: Self;        // word 1
pub const GENERATOR: Self;  // word 0b10, the class alpha = [X]

pub fn from_u128(bits: u128) -> Self
pub fn to_u128(self) -> u128
```

`GENERATOR` names the element that generates $F$ as an algebra over $\mathbb F_2$, since $F=\mathbb F_2[\alpha]$. It does not claim to generate the multiplicative group.

### Operators

The four operations are defined once each, in the standard traits, so callers write `a + b` and `a * b`:

| Trait | Behaviour |
|---|---|
| `Add` | XOR of the two words |
| `Sub` | identical to `Add`: in characteristic two, subtracting is adding |
| `Neg` | the identity, because $-a=a$ |
| `Mul` | reads both words, calls `mul_p128`, wraps the result |

There is no inherent method with the same name as an operator, and no `Div`, `pow`, `AddAssign` or `MulAssign` yet.

### Squaring and inversion

```rust
pub fn square(self) -> Self
pub fn repeated_square(self, exp: usize) -> Self
pub fn inverse(self) -> CtOption<Gf128>
```

`square` delegates to `square_p128`, so the spreading stages stay in the backend. `repeated_square` applies it `exp` times, which is the Frobenius power $a^{2^{exp}}$ the inversion chain needs.

`inverse` uses Itoh–Tsujii. Writing $u_k=a^{2^k-1}$, the recurrence is

```math
u_{r+s}=u_r^{2^s}u_s,
\qquad
u_1=a,
\qquad
a^{-1}=u_{127}^2.
```

The implementation walks the addition chain

```text
1 → 2 → 3 → 6 → 7 → 8 → 15 → 30 → 60 → 120 → 127
```

reusing a single saved intermediate as the second factor: $u_1$ for the first steps, then $u_7$ once the chain reaches it. That costs ten field multiplications and 127 squarings, against the 126 multiplications of plain binary exponentiation.

The arithmetic core is a private `inverse_or_zero` method computing

```math
J(a)=a^{2^{128}-2}.
```

For nonzero inputs, $J(a)=a^{-1}$, while $J(0)=0$. The public `inverse` method therefore returns a `CtOption<Gf128>`: the candidate is always computed, and its validity is carried separately as a `subtle::Choice`. Inverting `ZERO` no longer panics; it returns an invalid `CtOption`.

`Gf128` also implements `ConstantTimeEq` and `ConditionallySelectable` by delegating to the internal `u128` through the `subtle` crate.

### Where the work happens

```text
Gf128(a)
   │
   │ field operation
   ▼
field backend
   │
   ▼
Gf128(c)
```

The current backend is `field::portable`. `Gf128` contains no Karatsuba, no spreading and no reduction of its own.

---

## Future hardware backend

Deferred until after the transforms, for the reason given in the roadmap. Sketched here only so the layer it belongs to is fixed in advance.

A later stage will introduce

```text
src/field/pclmul.rs
```

using CPU carry-less multiplication instructions such as `PCLMULQDQ` and, where appropriate, wider vector variants.

The portable and hardware implementations should expose the same internal field-arithmetic contract.

Backend selection belongs below `Gf128`.

The additive FFT should only see field operations.

---

## Cantor basis

After the field implementation is complete, the project will introduce

```text
src/cantor.rs
```

for the Cantor special basis and associated binary subspaces.

The relevant basis relation is

```math
\beta_0=1,
\qquad
\beta_i^2+\beta_i=\beta_{i-1}.
```

The subspaces are

```math
W_i
=
\mathrm{span}_{\mathbb F_2}
\{\beta_0,\ldots,\beta_{i-1}\}.
```

Initially this will remain a single module.

It will be split only if the implementation becomes large enough to justify it.

---

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

The Cantor-basis additive FFT with a general recursive split.

It provides the reference optimized implementation before the dyadic specialization.

### `dyadic.rs`

The dyadic additive FFT specialization.

This is the main algorithmic target of the project.

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

- field arithmetic;
- Cantor-basis construction;
- the general AFFT;
- the dyadic specialization.

---

## Design principles

- Algebra first. Algorithms are derived from mathematical identities before implementation.
- Specialize deliberately. The current target is `GF(2^128)`, not an arbitrary runtime-defined field.
- Keep representations separated. Integer multiplication, carry-less polynomial multiplication and field multiplication are different operations.
- Hide backend details below `Gf128`. Cantor and AFFT code should operate on field elements, not inspect raw bits.
- Prefer independent test oracles. Optimized code is checked against simpler implementations with different structure.
- Avoid premature generic abstractions.
- Correctness before performance.
- Add hardware acceleration only after the portable implementation is verified.

---

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
source-level constant-time portable backend
        ↓
Cantor special basis
        ↓
naive additive evaluation
        ↓
general AFFT
        ↓
dyadic AFFT
        ↓
hardware backend and performance work
```

Hardware multiplication is deliberately last. It changes how fast the field arithmetic runs, not what the transforms compute, so it is worth doing once there is something whose speed actually matters.

---

## Status

Currently implemented:

```text
binary coordinates
        ↓
portable carry-less multiplication
        ↓
specialized GF(2^128) reduction
        ↓
Gf128
        ↓
specialized squaring
        ↓
Itoh–Tsujii inversion
        ↓
source-level constant-time portable backend
```

The portable field layer is complete for the current milestone. `Gf128` supports addition, subtraction, negation, multiplication, squaring, constant-time equality, conditional selection, and inversion through `CtOption<Gf128>`. The portable carry-less multiplication uses a fixed loop with mask-based selection, and the implementation uses `subtle` for sensitive predicates exposed through the field API.

This is a source-level constant-time discipline, not a universal proof about generated machine code or a specific processor. Dedicated timing analysis such as `dudect-bencher` may be added later as an external validation step.

The Cantor basis, the transforms and the hardware backend are still ahead.

Next:

```text
Cantor special basis
        ↓
additive FFT
```

From here on the work moves above the field: `cantor.rs` will build the evaluation domain, and `afft/` the transforms. Both operate on `Gf128` values and never on the words inside them.

## References

The additive FFT part of the project is developed with reference to:

- S.-J. Lin, W.-H. Chung, Y. S. Han, *Novel Polynomial Basis and Its Application to Reed-Solomon Erasure Codes*, arXiv:1404.3458.
- N. Coxon, *Fast Transforms over Finite Fields of Characteristic Two*, arXiv:1807.07785.
- S. Samanta, M. Badakhshan, G. Gong, *On the Additive FFT Techniques over Binary Extension Fields*, arXiv:2608.20855.

## License

Unless otherwise noted, the code in this repository, including its prior revisions, is licensed under the MIT License.

See [LICENSE](LICENSE) for details.
