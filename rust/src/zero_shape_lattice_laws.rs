//! LANG.VALUES.SHAPE's square of pair operations, held against the
//! implementation.
//!
//! Read over a number's two halves, a product is an AND and a sum an OR, so
//! the four ways to combine numerators and denominators by product or sum are
//! FOUR's four operations: `ac/bd` (`MUL`, ⊗), `(a+c)/(b+d)` (the mediant, ⊕),
//! `ac/(b+d)` (the truth meet ∧) and its reciprocal dual (the truth join ∨).
//! Each law is checked by running programs:
//!
//! 1. the pair meet `ac/(b+d)` and its dual join are the truth lattice on the
//!    shapes of every pair of rationals, signs included, with `1/0` the
//!    meet's unit and `0` absorbing — and the meet is not associative on
//!    values;
//! 2. `0/0` absorbs every operation the field writes, which is why only ⊗ —
//!    the one of FOUR's operations under which UNKNOWN absorbs — is a field
//!    operation;
//! 3. the field's shadows, `ADD` for ∨ and the parallel sum `1/(1/x + 1/y)`
//!    for ∧, miss the lattice exactly where `0/0` absorbs, where two operands
//!    cancel and where two points meet, and nowhere else.

use crate::interpreter::Interpreter;
use crate::types::{Value, ValueData};
use num_traits::Zero;

/// Every zero shape, both signs, and a pair that cancels (`2`, `-2`).
const RATIONALS: &[&str] = &["0", "1", "2", "-2", "1/2", "-3/4", "1/0", "-1/0", "0/0"];

const DEFINITIONS: &str = "\
    [ RATIO 'Y' BIND RATIO 'X' BIND X 0 GET Y 0 GET MUL X 1 GET Y 1 GET ADD DIV ] 'TMEET' DEF \
    [ 'B' BIND 'A' BIND 1 1 A DIV 1 B DIV TMEET DIV ] 'TJOIN' DEF \
    [ 'B' BIND 'A' BIND 1 1 A DIV 1 B DIV ADD DIV ] 'PARALLEL' DEF ";

/// `(numerator ≠ 0, denominator ≠ 0)`, read as a truth value: the numerator
/// half says it holds, the denominator half that it does not. FALSE is `0`,
/// TRUE `±1/0`, UNKNOWN `0/0` and BOTH every other number.
type Shape = (bool, bool);

const FALSE: Shape = (false, true);
const TRUE: Shape = (true, false);
const UNKNOWN: Shape = (false, false);
const BOTH: Shape = (true, true);

/// The truth order's meet: true where both are, false where either is.
fn truth_meet(a: Shape, b: Shape) -> Shape {
    (a.0 && b.0, a.1 || b.1)
}

fn truth_join(a: Shape, b: Shape) -> Shape {
    (a.0 || b.0, a.1 && b.1)
}

fn shape_of(value: &Value) -> Shape {
    match &value.data {
        ValueData::Scalar(f) => (!f.numerator().is_zero(), f.is_finite()),
        ValueData::ExactScalar(_) => BOTH,
        other => panic!("expected a number, got {other:?}"),
    }
}

async fn number(code: &str) -> Value {
    let mut interp = Interpreter::new();
    interp
        .execute(&format!("{DEFINITIONS}{code}"))
        .await
        .unwrap_or_else(|e| panic!("`{code}` must run: {e:?}"));
    let stack = interp.get_stack();
    assert_eq!(stack.len(), 1, "`{code}` must leave one value");
    stack[0].clone()
}

async fn shape(code: &str) -> Shape {
    shape_of(&number(code).await)
}

async fn equals(code: &str, expected: &str) -> bool {
    matches!(
        number(&format!("{code} {expected} EQ")).await.data,
        ValueData::Boolean(true)
    )
}

#[tokio::test]
async fn the_pair_meet_and_its_dual_join_are_the_truth_lattice_on_shapes() {
    for x in RATIONALS {
        let sx = shape(x).await;
        for y in RATIONALS {
            let sy = shape(y).await;
            assert_eq!(
                shape(&format!("{x} {y} TMEET")).await,
                truth_meet(sx, sy),
                "{x} ∧ {y}"
            );
            assert_eq!(
                shape(&format!("{x} {y} TJOIN")).await,
                truth_join(sx, sy),
                "{x} ∨ {y}"
            );
        }
        // TRUE is the meet's unit, value for value; FALSE absorbs it.
        assert!(
            equals(&format!("{x} 1/0 TMEET"), x).await,
            "{x} ∧ 1/0 = {x}"
        );
        assert!(equals(&format!("{x} 0 TMEET"), "0").await, "{x} ∧ 0 = 0");
    }
    // BOTH meets UNKNOWN at FALSE, as the truth table says.
    assert!(equals("3 0/0 TMEET", "0").await);
}

#[tokio::test]
async fn the_pair_meet_is_not_associative_on_values() {
    // The logic's meet folds in any order; the pair meet, like the mediant,
    // keeps the order in the value and agrees only on the shape.
    assert!(equals("1/2 1/3 TMEET 2 TMEET", "1/3").await);
    assert!(equals("1/2 1/3 2 TMEET TMEET", "1/4").await);
}

/// Every Word the field writes its arithmetic with sends `0/0` to `0/0` (or to
/// an absence), in every operand position. So no phrase of them can send
/// UNKNOWN anywhere else — and ∧, ∨ and ⊕ each do, while ⊗ alone keeps it.
#[tokio::test]
async fn nullity_absorbs_every_operation_the_field_writes() {
    let others = ["0", "3", "-2", "1/2", "2 SQRT", "1/0", "-1/0", "0/0"];
    for word in ["ADD", "SUB", "MUL", "DIV", "POW"] {
        for other in others {
            for code in [format!("0/0 {other} {word}"), format!("{other} 0/0 {word}")] {
                let value = number(&code).await;
                assert!(
                    value.is_nil() || shape_of(&value) == UNKNOWN,
                    "`{code}` must stay 0/0, got {value}"
                );
            }
        }
    }
    for word in ["SQRT", "FLOOR", "ROUND"] {
        assert_eq!(shape(&format!("0/0 {word}")).await, UNKNOWN, "0/0 {word}");
    }
    // FOUR's four operations against UNKNOWN: only ⊗ lets it absorb.
    assert_eq!(truth_meet(UNKNOWN, FALSE), FALSE);
    assert_eq!(truth_join(UNKNOWN, TRUE), TRUE);
    assert_eq!(
        ((UNKNOWN.0 || BOTH.0), (UNKNOWN.1 || BOTH.1)),
        BOTH,
        "⊕ yields it"
    );
    assert_eq!(
        ((UNKNOWN.0 && BOTH.0), (UNKNOWN.1 && BOTH.1)),
        UNKNOWN,
        "⊗ keeps it"
    );
}

/// Where a field shadow is predicted to miss its lattice operation: where
/// `0/0` absorbs an operand that would have decided the answer, where two
/// finite numbers cancel, and where two points over zero meet.
fn shadow_misses(op: &str, x: &str, sx: Shape, y: &str, sy: Shape) -> bool {
    let cancels = sx == BOTH
        && sy == BOTH
        && x.trim_start_matches('-') == y.trim_start_matches('-')
        && x.starts_with('-') != y.starts_with('-');
    let absorbed =
        |decider: Shape| (sx == UNKNOWN && sy == decider) || (sy == UNKNOWN && sx == decider);
    match op {
        // ∨: UNKNOWN with TRUE or BOTH is TRUE; two points sum to 0/0.
        "ADD" => cancels || absorbed(TRUE) || absorbed(BOTH) || (sx == TRUE && sy == TRUE),
        // ∧: UNKNOWN with FALSE or BOTH is FALSE; two zeros give 0/0.
        _ => cancels || absorbed(FALSE) || absorbed(BOTH) || (sx == FALSE && sy == FALSE),
    }
}

#[tokio::test]
async fn the_field_shadows_miss_the_lattice_only_where_predicted() {
    let mut misses = 0;
    for x in RATIONALS {
        let sx = shape(x).await;
        for y in RATIONALS {
            let sy = shape(y).await;
            for (op, lattice) in [
                ("ADD", truth_join(sx, sy)),
                ("PARALLEL", truth_meet(sx, sy)),
            ] {
                let missed = shape(&format!("{x} {y} {op}")).await != lattice;
                assert_eq!(
                    missed,
                    shadow_misses(op, x, sx, y, sy),
                    "{x} {y} {op}: missed the lattice = {missed}"
                );
                misses += usize::from(missed);
            }
        }
    }
    assert!(misses > 0, "the shadows do miss the lattice somewhere");
    // The two shadows are reciprocal duals, and each miss has its mirror.
    assert!(equals("1/0 1/0 ADD", "0/0").await);
    assert!(equals("0 0 PARALLEL", "0/0").await);
    assert!(equals("2 -2 ADD", "0").await);
    assert!(equals("2 -2 PARALLEL", "1/0").await);
    // Resistances: in parallel a short dominates and an open circuit is no
    // circuit at all; in series an open circuit dominates.
    assert!(equals("3 0 PARALLEL", "0").await);
    assert!(equals("3 1/0 PARALLEL", "3").await);
    assert!(equals("3 6 PARALLEL", "2").await);
    assert!(equals("3 1/0 ADD", "1/0").await);
}
