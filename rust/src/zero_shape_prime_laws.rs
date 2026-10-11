//! LANG.VALUES.SHAPE's prime coordinates and its corrected trilemma, held
//! against the implementation.
//!
//! 1. Unique factorization extended to the three points: a number is a sign
//!    and an exponent per prime — `0` every exponent +∞ and no sign, a point
//!    over zero every exponent −∞ and its sign, `0/0` no exponent — and `MUL`
//!    multiplies signs and adds exponents (+∞ and −∞ summing to none) while
//!    the reciprocal negates them. Checked against what `MUL` and `DIV`
//!    answer.
//! 2. A sum lets the smaller exponent win and lets two equal ones cancel:
//!    `1/0 1/0 ADD` is `0/0` for the reason `1 -1 ADD` is `0`.
//! 3. With one zero, a signed infinity and an involutive reciprocal exclude
//!    each other outright: the reciprocal sends both points to the one zero.
//! 4. Keeping all three — signed infinity, involution and the additive wheel
//!    laws — is possible, but only in a domain with a mirror of every number:
//!    the wheel of fractions over positive denominators, modelled here.

use crate::interpreter::Interpreter;
use crate::types::{Value, ValueData};
use num_traits::{Signed, ToPrimitive, Zero};

const PRIMES: [i64; 4] = [2, 3, 5, 7];

/// Rationals of both signs and every zero shape.
const SAMPLE: &[&str] = &[
    "0", "1", "-2", "3/4", "-5/6", "12", "7/20", "1/0", "-1/0", "0/0",
];

/// An exponent: finite, +∞ (a zero) or −∞ (a point over zero).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Exp {
    NegInf,
    Finite(i64),
    PosInf,
}

/// A sign (absent for `0` and `0/0`) and one exponent per prime (absent for
/// `0/0`).
type Coords = (Option<i64>, Option<[Exp; 4]>);

fn coords(value: &Value) -> Coords {
    let ValueData::Scalar(f) = &value.data else {
        panic!("expected a rational, got {value}");
    };
    let (n, d) = (f.numerator(), f.denominator());
    match (n.is_zero(), d.is_zero()) {
        (true, true) => (None, None),
        (true, false) => (None, Some([Exp::PosInf; 4])),
        (false, true) => (
            Some(if n.is_positive() { 1 } else { -1 }),
            Some([Exp::NegInf; 4]),
        ),
        (false, false) => {
            let sign = if n.is_positive() { 1 } else { -1 };
            let (mut n, mut d) = (n.abs().to_i64().unwrap(), d.to_i64().unwrap());
            let mut exps = [Exp::Finite(0); 4];
            for (slot, p) in exps.iter_mut().zip(PRIMES) {
                let mut e = 0;
                while n % p == 0 {
                    n /= p;
                    e += 1;
                }
                while d % p == 0 {
                    d /= p;
                    e -= 1;
                }
                *slot = Exp::Finite(e);
            }
            (Some(sign), Some(exps))
        }
    }
}

fn add_exp(a: Exp, b: Exp) -> Option<Exp> {
    match (a, b) {
        (Exp::PosInf, Exp::NegInf) | (Exp::NegInf, Exp::PosInf) => None,
        (Exp::PosInf, _) | (_, Exp::PosInf) => Some(Exp::PosInf),
        (Exp::NegInf, _) | (_, Exp::NegInf) => Some(Exp::NegInf),
        (Exp::Finite(x), Exp::Finite(y)) => Some(Exp::Finite(x + y)),
    }
}

fn negate(e: Exp) -> Exp {
    match e {
        Exp::PosInf => Exp::NegInf,
        Exp::NegInf => Exp::PosInf,
        Exp::Finite(x) => Exp::Finite(-x),
    }
}

/// Signs multiply and exponents add; a zero keeps no sign.
fn coords_mul(x: Coords, y: Coords) -> Coords {
    let (Some(ex), Some(ey)) = (x.1, y.1) else {
        return (None, None);
    };
    let mut exps = [Exp::Finite(0); 4];
    for i in 0..4 {
        match add_exp(ex[i], ey[i]) {
            Some(e) => exps[i] = e,
            None => return (None, None),
        }
    }
    if exps[0] == Exp::PosInf {
        return (None, Some(exps));
    }
    (x.0.zip(y.0).map(|(a, b)| a * b), Some(exps))
}

/// Exponents negate; the reciprocal of a zero is the positive point.
fn coords_reciprocal(x: Coords) -> Coords {
    let Some(ex) = x.1 else {
        return (None, None);
    };
    let exps = ex.map(negate);
    if exps[0] == Exp::PosInf {
        return (None, Some(exps));
    }
    (Some(x.0.unwrap_or(1)), Some(exps))
}

async fn run(code: &str) -> Value {
    let mut interp = Interpreter::new();
    interp
        .execute(code)
        .await
        .unwrap_or_else(|e| panic!("`{code}` must run: {e:?}"));
    let stack = interp.get_stack();
    assert_eq!(stack.len(), 1, "`{code}` must leave one value");
    stack[0].clone()
}

#[tokio::test]
async fn mul_and_the_reciprocal_are_unique_factorization_extended() {
    for x in SAMPLE {
        let cx = coords(&run(x).await);
        assert_eq!(
            coords(&run(&format!("1 {x} DIV")).await),
            coords_reciprocal(cx),
            "1/{x} negates the exponents"
        );
        for y in SAMPLE {
            let cy = coords(&run(y).await);
            assert_eq!(
                coords(&run(&format!("{x} {y} MUL")).await),
                coords_mul(cx, cy),
                "{x} · {y} multiplies signs and adds exponents"
            );
        }
    }
}

#[tokio::test]
async fn a_sum_lets_the_smaller_exponent_win_and_equal_ones_cancel() {
    for x in SAMPLE.iter().filter(|x| **x != "0/0") {
        let cx = coords(&run(x).await);
        for y in SAMPLE.iter().filter(|y| **y != "0/0") {
            let cy = coords(&run(y).await);
            let sum = coords(&run(&format!("{x} {y} ADD")).await);
            let (Some(ex), Some(ey)) = (cx.1, cy.1) else {
                unreachable!("only 0/0 has no exponents");
            };
            let Some(es) = sum.1 else {
                // Only two points over zero — equal exponents, nothing finer
                // to settle them — sum to 0/0.
                assert!(
                    ex[0] == Exp::NegInf && ey[0] == Exp::NegInf,
                    "{x} + {y} reached 0/0 from operands that are not both points"
                );
                continue;
            };
            for i in 0..4 {
                let least = ex[i].min(ey[i]);
                assert!(
                    es[i] >= least,
                    "{x} + {y}: exponent of {} fell below both",
                    PRIMES[i]
                );
                if ex[i] != ey[i] {
                    assert_eq!(
                        es[i], least,
                        "{x} + {y}: the smaller exponent of {} wins",
                        PRIMES[i]
                    );
                }
            }
        }
    }
    // The two cancellations are one phenomenon.
    assert_eq!(
        coords(&run("1 -1 ADD").await),
        (None, Some([Exp::PosInf; 4]))
    );
    assert_eq!(coords(&run("1/0 1/0 ADD").await), (None, None));
}

#[tokio::test]
async fn one_zero_leaves_no_involutive_reciprocal_for_two_points() {
    // Both points go to the one zero, so the reciprocal cannot be undone on
    // both: the zero comes back as the positive point.
    let from_positive = run("1 1/0 DIV").await;
    let from_negative = run("1 -1/0 DIV").await;
    assert_eq!(coords(&from_positive), coords(&from_negative));
    assert_eq!(coords(&from_positive), (None, Some([Exp::PosInf; 4])));
    assert_eq!(
        coords(&run("1 1 -1/0 DIV DIV").await),
        (Some(1), Some([Exp::NegInf; 4]))
    );
}

// --- The wheel of fractions over positive denominators ---------------------
//
// Pairs of integers identified only under positive scaling. It keeps both
// signs of infinity and of zero, and a mirror of every number, and it is a
// wheel: every axiom holds, the involution included. It is the third vertex
// the corrected trilemma names, and it is not the field and three points.

type Pair = (i64, i64);

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn norm((x, y): Pair) -> Pair {
    match gcd(x, y) {
        0 => (0, 0),
        g => (x / g, y / g),
    }
}

fn add(a: Pair, b: Pair) -> Pair {
    norm((a.0 * b.1 + b.0 * a.1, a.1 * b.1))
}

fn mul(a: Pair, b: Pair) -> Pair {
    norm((a.0 * b.0, a.1 * b.1))
}

fn inv(a: Pair) -> Pair {
    norm((a.1, a.0))
}

#[test]
fn the_oriented_wheel_keeps_all_three_by_mirroring_every_number() {
    let sample: Vec<Pair> = [
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (-2, 1),
        (2, -1),
        (1, 2),
        (1, 0),
        (-1, 0),
        (0, 0),
    ]
    .into_iter()
    .map(norm)
    .collect();
    let zero = (0, 1);
    for &x in &sample {
        assert_eq!(inv(inv(x)), x, "//x = x at {x:?}");
        for &y in &sample {
            assert_eq!(inv(mul(x, y)), mul(inv(x), inv(y)), "W3 at {x:?} {y:?}");
            for &z in &sample {
                let zy = mul(zero, y);
                assert_eq!(
                    add(mul(x, z), mul(y, z)),
                    add(mul(add(x, y), z), mul(zero, z)),
                    "W4"
                );
                assert_eq!(
                    mul(add(x, mul(y, z)), inv(y)),
                    add(add(mul(x, inv(y)), z), zy),
                    "W5"
                );
                assert_eq!(mul(add(x, zy), z), add(mul(x, z), zy), "W7");
                assert_eq!(inv(add(x, zy)), add(inv(x), zy), "W8");
            }
        }
    }
    // The price: two zeros, and a mirror of every number.
    assert_ne!(norm((0, 1)), norm((0, -1)));
    assert_ne!(norm((1, 2)), norm((-1, -2)));
    assert_ne!(norm((1, 0)), norm((-1, 0)));
}
