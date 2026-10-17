# Formal Verification

## What it adds

Ajisai's tests check behavior at the inputs someone wrote down (unit, MC/DC
and conformance tests) or at inputs a generator draws (the `proptest`
properties). Neither says anything about the inputs it did not reach, and the
numeric core's defects live at exactly the inputs nobody thinks to write:
`i64::MIN`, a sum one past `i64::MAX`, a cross product whose 128-bit width
the fast path forgot.

The harnesses here close that gap for the kernels small enough to decide. They
run under [Kani](https://github.com/model-checking/kani), a bit-precise
bounded model checker for Rust: each `kani::any()` value stands for **every**
value of its type, and the solver either proves the assertions hold for all of
them — and that no arithmetic overflow, out-of-bounds index or panic is
reachable — or returns the counterexample. A passing harness is a proof, not
a sample.

## Running

Kani pins its own nightly toolchain and solver, so it is installed apart from
the project's stable toolchain:

```sh
cargo install --locked kani-verifier
cargo kani setup
```

Then, in `rust/`:

```sh
cargo kani --jobs                 # every harness, in parallel
cargo kani --harness truth_ops    # the harnesses whose path matches
```

The harnesses compile only under `cfg(kani)`, which `cargo kani` sets: a plain
`cargo build` or `cargo test` never sees them, and `Cargo.toml` declares the
cfg so `clippy -D warnings` accepts it. CI runs every harness in the
`formal-verification` job of `.github/workflows/test.yml`.

## Two kinds of harness

- **Whole-range** harnesses quantify over every value of a machine type. They
  are possible where the code under proof has no loop, so the solver sees all
  of it at once.
- **Small-scope** harnesses take every input below a bound and unroll the
  loops in between to a fixed depth (`#[kani::unwind]`); Kani fails the
  harness if the bound is too small to reach a loop's exit, so the bound
  cannot silently cut a path off. They rest on the small-scope hypothesis: a
  defect in a gcd or a normalization shows itself on small operands too. The
  property tests remain the evidence for large ones.

Each harness states its law a second way — the strong Kleene truth order, or
exact `i128` arithmetic where no product of two words overflows — rather than
restating the kernel, so it can disagree with it.

## The harnesses

| File | Subject | Kind |
|---|---|---|
| `rust/src/interpreter/truth_ops_proofs.rs` | The `AND` table and `NOT` lane rule of LANG.VALUES.TRUTH: `AND` is the meet of F < U < T (so a bounded semilattice), `NOT` the order-reversing involution fixing UNKNOWN, and De Morgan gives the join | Whole-range (the three codes) |
| `rust/src/types/small_rational_proofs.rs` | The machine-word rational kernels (`small_rational`): the wasm32 multiply, rounding, the order, the integer routes of `add` and `mul`, the gcd's power-of-two shortcut | Whole-range (every `i64`) |
| `rust/src/types/small_rational_proofs.rs` | Stein's gcd against Euclid's; `add`, `mul` and `div` answering the exact value in lowest terms over a positive denominator; the order agreeing with equality of normalized pairs | Small-scope (halves under 2⁴, gcd operands under 2⁶) |

In `TRACEABILITY_MATRIX.md` the rational harnesses are `AQ-VER-001-T`, under
AQ-REQ-001, and the truth-table harnesses are listed among the verification
without a requirement ID.

## Adding a harness

- Put it in a `*_proofs.rs` file beside its subject, attached as the
  subject's `#[cfg(kani)] #[path = "..."] mod proofs;`, the way the test
  modules attach, so it can reach private items without widening them.
- Prefer a whole-range harness; when the subject loops, bound the input, set
  `#[kani::unwind]` just high enough, and say in the doc comment what the
  bound covers.
- Keep each harness to a few minutes of solver time (the slowest today,
  `add_is_the_normalized_sum`, takes about eight): CI runs them all on every
  pull request. Most of a small-scope harness's formula is its specification;
  compute it in the narrowest type the scope fits, as `small_rational_proofs`
  does in 16 and 32 bits.
