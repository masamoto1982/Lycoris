use crate::error::{AjisaiError, NilReason, Result};
use crate::interpreter::lane_lift::lift_lanes;
use crate::interpreter::Interpreter;
use crate::semantic::Recoverability;
use crate::types::{Value, ValueData};
use std::sync::Arc;

/// One of the four truth values of LANG.VALUES.TRUTH, read from a lane in
/// truth position.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Truth {
    True,
    False,
    Both,
    /// A NIL read in truth position, whatever its reason.
    Unknown,
}

/// The truth value of a truth-order connective's operand.
///
/// TRUE, FALSE and BOTH are the Boolean data values and NIL reads as UNKNOWN
/// (LANG.VALUES.TRUTH); nothing else is a truth value, so a scalar, a String
/// or a Vector standing in truth position is the `nonTruthValue` every Word
/// of the family registers. `0` and `1` are not truth values
/// (LANG.VALUES.DISJOINT): a caller who means a numeric test writes it,
/// `0 EQ NOT`.
///
/// A truth operand lifts (LANG.COLLECTIONS.LIFT), so this sees one lane at a
/// time: [`lift_lanes`] has already aligned the operands, and a Vector
/// reaching here is a Vector standing where a truth value belongs.
fn operand_truth(value: &Value) -> Result<Truth> {
    match &value.data {
        ValueData::Boolean(true) => Ok(Truth::True),
        ValueData::Boolean(false) => Ok(Truth::False),
        ValueData::Both => Ok(Truth::Both),
        ValueData::Nil => Ok(Truth::Unknown),
        _ => Err(AjisaiError::declared(
            "nonTruthValue",
            format!("expected a truth value, got {}", value.domain_name()),
        )),
    }
}

/// Conjunction under Belnap's table (LANG.VALUES.TRUTH), the meet of the
/// truth order FALSE < UNKNOWN, BOTH < TRUE. FALSE absorbs everything, TRUE
/// is the identity, and UNKNOWN and BOTH, neither below the other, meet at
/// FALSE. Restricted to TRUE, FALSE and UNKNOWN it is the strong Kleene
/// table. Where the answer is an operand it is that operand whole, so an
/// UNKNOWN keeps its reason — the left one's, when both are UNKNOWN.
fn compute_conjunction(a: &Value, b: &Value) -> Result<Value> {
    use Truth::*;
    Ok(match (operand_truth(a)?, operand_truth(b)?) {
        (False, _) | (_, False) | (Unknown, Both) | (Both, Unknown) => Value::from_bool(false),
        (True, _) | (Both, Both) | (Unknown, Unknown) => b.clone(),
        (_, True) => a.clone(),
    })
}

/// `AND` over whole operands: the scalar law above, applied lane by lane
/// (LANG.COLLECTIONS.LIFT).
fn lifted_conjunction(a: &Value, b: &Value) -> Result<Value> {
    lift_lanes([a, b], |[x, y]| compute_conjunction(x, y))
}

/// `SELECT`'s scalar law: TRUE and FALSE choose one of the two values it was
/// handed, UNKNOWN chooses neither, and BOTH chooses both — the two
/// candidates reconciled, so they answer what they agree on.
///
/// Nothing is evaluated here. Both candidates are values the program already
/// built, so the work that produced them happened before `SELECT` ran, once
/// each and in the order they were written — which is why `SELECT` is `pure`
/// and `const` on the step axis where `COND` was `unbounded` on all three.
fn compute_selection(when_true: &Value, when_false: &Value, mask: &Value) -> Result<Value> {
    Ok(match operand_truth(mask)? {
        Truth::True => when_true.clone(),
        Truth::False => when_false.clone(),
        Truth::Both => reconcile(when_true, when_false),
        // UNKNOWN chooses neither: the answer is the absence the truth
        // operand carried, reason intact, so `NIL-REASON` can still say why.
        Truth::Unknown => mask.clone(),
    })
}

fn compute_inverted_value(val: &Value) -> Result<Value> {
    // NOT swaps TRUE and FALSE and leaves the two values between them where
    // they are: UNKNOWN flows out unchanged, keeping its reason, and BOTH
    // stays BOTH (LANG.VALUES.TRUTH's NOT row).
    Ok(match operand_truth(val)? {
        Truth::True => Value::from_bool(false),
        Truth::False => Value::from_bool(true),
        Truth::Both | Truth::Unknown => val.clone(),
    })
}

/// `RECONCILE`'s scalar law: what two sources agree on (LANG.VALUES.TRUTH).
///
/// It is the join of the information order — an absence knows least, a
/// conflict most. A conflict absorbs, an absence yields to the other source,
/// equal values agree, two different truth values are BOTH, and any other
/// two different values project NIL(conflict). Making the conflict absorbing
/// rather than one more absence that yields is what keeps the Word
/// associative: `1 2 RECONCILE 3 RECONCILE` and `1 2 3 RECONCILE RECONCILE`
/// are both the conflict.
///
/// Among numbers `0/0` knows least, as UNKNOWN does among truth values: its
/// zero shape is FOUR's bottom (LANG.VALUES.SHAPE), so it yields to the
/// other number as an absence yields to any source. It yields only to a
/// number — a String or a truth value tells nothing about which number is
/// meant — so the order stays a join semilattice and the Word associative.
pub(crate) fn reconcile(a: &Value, b: &Value) -> Value {
    let is_conflict = |v: &Value| v.nil_reason() == Some(&NilReason::Conflict);
    if is_conflict(a) || b.is_nil() && !is_conflict(b) {
        return a.clone();
    }
    if is_conflict(b) || a.is_nil() || a == b {
        return b.clone();
    }
    let is_number = |v: &Value| matches!(v.data, ValueData::Scalar(_) | ValueData::ExactScalar(_));
    let is_nullity = |v: &Value| matches!(&v.data, ValueData::Scalar(f) if f.is_nullity());
    if is_nullity(b) && is_number(a) {
        return a.clone();
    }
    if is_nullity(a) && is_number(b) {
        return b.clone();
    }
    let is_truth = |v: &Value| matches!(v.data, ValueData::Boolean(_) | ValueData::Both);
    if is_truth(a) && is_truth(b) {
        Value::both()
    } else {
        Value::nil_with_reason(NilReason::Conflict, Recoverability::Recoverable)
    }
}

/// `RECONCILE` — what two sources agree on. Like `EQ` it reads its
/// operands whole, so two Vectors agree when they are one value.
pub fn op_reconcile(interp: &mut Interpreter) -> Result<()> {
    if interp.stack.len() < 2 {
        return Err(AjisaiError::stack_underflow());
    }
    let b_val = interp.stack.pop().ok_or(AjisaiError::stack_underflow())?;
    let a_val = interp.stack.pop().ok_or(AjisaiError::stack_underflow())?;
    interp.stack.push(reconcile(&a_val, &b_val));
    Ok(())
}

/// The lanes of a flat Vector of truth values, open for rewriting where they
/// lie: the answer of a connective whose operand is such a Vector is that
/// Vector with its lanes replaced, so when nothing else holds it the answer
/// needs no new Vector and frees no old one. `None` for anything else — a
/// lane that is not a truth value, whose ERROR the lifted path words — and
/// nothing has been touched.
fn sole_truth_lanes(value: &mut Value) -> Option<&mut [Value]> {
    let ValueData::Vector(items) = &mut value.data else {
        return None;
    };
    let truths = items.iter().all(|lane| {
        matches!(
            lane.data,
            ValueData::Boolean(_) | ValueData::Both | ValueData::Nil
        )
    });
    // A Vector something else still holds is copied once, here, and the copy
    // rewritten: cheaper than building the answer lane by lane.
    truths.then(|| Arc::make_mut(items).as_mut_slice())
}

/// The top of the stack, open for rewriting in place.
fn top_mut(interp: &mut Interpreter) -> Option<&mut Value> {
    let top = interp.stack.len().checked_sub(1)?;
    Some(&mut interp.stack[top])
}

pub fn op_not(interp: &mut Interpreter) -> Result<()> {
    if let Some(lanes) = top_mut(interp).and_then(sole_truth_lanes) {
        for lane in lanes {
            if let ValueData::Boolean(b) = &mut lane.data {
                *b = !*b;
            }
        }
        return Ok(());
    }
    let val = interp.stack.pop().ok_or(AjisaiError::stack_underflow())?;

    let result = match lift_lanes([&val], |[x]| compute_inverted_value(x)) {
        Ok(v) => v,
        Err(e) => {
            interp.stack.push(val);
            return Err(e);
        }
    };

    interp.stack.push(result);
    Ok(())
}

pub fn op_and(interp: &mut Interpreter) -> Result<()> {
    if interp.stack.len() < 2 {
        return Err(AjisaiError::stack_underflow());
    }

    let b_val = interp.stack.pop().ok_or(AjisaiError::stack_underflow())?;
    if let Some(answer) = conjoin_in_place(interp, &b_val) {
        return answer;
    }
    let a_val = interp.stack.pop().ok_or(AjisaiError::stack_underflow())?;

    let result = match lifted_conjunction(&a_val, &b_val) {
        Ok(v) => v,
        Err(e) => {
            interp.stack.push(a_val);
            interp.stack.push(b_val);
            return Err(e);
        }
    };
    interp.stack.push(result);
    Ok(())
}

/// `AND` written into its left operand when that is a flat Vector of truth
/// values and the right one is a truth value or a Vector of them of the same
/// length: each lane becomes the conjunction it would have been in a new
/// Vector. `None` leaves both operands as they were.
fn conjoin_in_place(interp: &mut Interpreter, b_val: &Value) -> Option<Result<()>> {
    let is_truth = |v: &Value| {
        matches!(
            v.data,
            ValueData::Boolean(_) | ValueData::Both | ValueData::Nil
        )
    };
    let right: &[Value] = match &b_val.data {
        ValueData::Vector(items) if items.iter().all(is_truth) => items,
        _ if is_truth(b_val) => std::slice::from_ref(b_val),
        _ => return None,
    };
    let width = interp.stack.last()?.as_vector_view()?.len();
    if right.len() != 1 && right.len() != width {
        return None;
    }
    let lanes = top_mut(interp).and_then(sole_truth_lanes)?;
    let reused = right.len() == 1;
    for (index, lane) in lanes.iter_mut().enumerate() {
        let other = &right[if reused { 0 } else { index }];
        // Every lane is a truth value, so the law cannot refuse one.
        *lane = compute_conjunction(lane, other).expect("both lanes are truth values");
    }
    Some(Ok(()))
}

/// `SELECT` written into its truth operand when that is a flat Vector of
/// truth values and each candidate is a leaf or a Vector of leaves as long
/// as the mask (or one lane long): each mask lane becomes the candidate it
/// chooses, as the lifted law would have built it. The candidates are taken
/// off the stack only when it answers; `false` leaves everything in place.
fn select_in_place(interp: &mut Interpreter, mask: &mut Value) -> bool {
    let Some(width) = mask.as_vector_view().map(|lanes| lanes.len()) else {
        return false;
    };
    let fits = |v: &Value| match &v.data {
        ValueData::Vector(items) => {
            (items.len() == 1 || items.len() == width)
                && items.iter().all(|lane| !is_container(lane))
        }
        _ => !is_container(v),
    };
    let depth = interp.stack.len();
    if depth < 2 || !fits(&interp.stack[depth - 1]) || !fits(&interp.stack[depth - 2]) {
        return false;
    }
    // Only a mask nothing else holds: copying a shared one first costs more
    // than building the answer, since every lane is overwritten anyway.
    if !matches!(&mask.data, ValueData::Vector(items) if Arc::strong_count(items) == 1) {
        return false;
    }
    let Some(lanes) = sole_truth_lanes(mask) else {
        return false;
    };
    let when_false = interp.stack.pop().expect("checked above");
    let when_true = interp.stack.pop().expect("checked above");
    fn pick(v: &Value, index: usize) -> &Value {
        match &v.data {
            ValueData::Vector(items) => &items[if items.len() == 1 { 0 } else { index }],
            _ => v,
        }
    }
    for (index, lane) in lanes.iter_mut().enumerate() {
        let (t, f) = (pick(&when_true, index), pick(&when_false, index));
        *lane = compute_selection(t, f, lane).expect("every mask lane is a truth value");
    }
    true
}

fn is_container(v: &Value) -> bool {
    matches!(
        v.data,
        ValueData::Vector(_) | ValueData::Tensor { .. } | ValueData::Record(_)
    )
}

/// `SELECT` — the conditional.
///
/// `[ whenTrue ] [ whenFalse ] [ mask ] SELECT` answers one of the two
/// candidates per lane. The truth operand comes last because that is where
/// every Word puts the operand that decides what it does, and because
/// `NIL?` leaves its answer exactly there: `[ 0 ] X X NIL? SELECT` reads as
/// "0 if X is absent, else X" with nothing moved on the stack.
///
/// Unlike the `COND` this replaces, `SELECT` evaluates nothing and holds no
/// frame: its operands are ordinary values, aligned by the one lifting rule
/// (LANG.COLLECTIONS.LIFT), so branching is no longer a construct with laws
/// of its own. There is no else-clause to reach and no clause set to exhaust
/// — two candidates and a truth are total by construction.
pub fn op_select(interp: &mut Interpreter) -> Result<()> {
    if interp.stack.len() < 3 {
        return Err(AjisaiError::stack_underflow());
    }

    let mut mask = interp.stack.pop().ok_or(AjisaiError::stack_underflow())?;
    if select_in_place(interp, &mut mask) {
        interp.stack.push(mask);
        return Ok(());
    }
    let when_false = interp.stack.pop().ok_or(AjisaiError::stack_underflow())?;
    let when_true = interp.stack.pop().ok_or(AjisaiError::stack_underflow())?;

    let result = match lift_lanes([&when_true, &when_false, &mask], |[t, f, m]| {
        compute_selection(t, f, m)
    }) {
        Ok(v) => v,
        Err(e) => {
            interp.stack.push(when_true);
            interp.stack.push(when_false);
            interp.stack.push(mask);
            return Err(e);
        }
    };

    interp.stack.push(result);
    Ok(())
}

#[cfg(test)]
mod in_place_tests {
    use super::*;
    use crate::error::NilReason;

    fn lanes() -> Vec<Value> {
        let unknown = |reason| Value::nil_with_reason(reason, Recoverability::Recoverable);
        vec![
            Value::from_bool(true),
            Value::from_bool(false),
            Value::both(),
            unknown(NilReason::NotFound),
            unknown(NilReason::DomainMiss),
        ]
    }

    fn right_lanes(all: &[Value]) -> Vec<Value> {
        all.iter().flat_map(|_| all.iter().cloned()).collect()
    }

    fn run(stack: Vec<Value>, op: fn(&mut Interpreter) -> Result<()>) -> Vec<Value> {
        let mut interp = Interpreter::new();
        interp.stack.extend(stack);
        op(&mut interp).expect("truth operands");
        interp.stack.into_values()
    }

    /// The rewritten lanes are the ones the lifted law builds, reasons
    /// included, whether or not anything else holds the operand.
    #[test]
    fn rewriting_in_place_answers_what_the_lifted_law_builds() {
        let all = lanes();
        let left: Vec<Value> = all
            .iter()
            .flat_map(|a| all.iter().map(move |_| a.clone()))
            .collect();
        let right = right_lanes(&all);
        let (left, right) = (Value::from_vector(left), Value::from_vector(right));
        let expected = lifted_conjunction(&left, &right).unwrap();
        for shared in [false, true] {
            let keep = shared.then(|| left.clone());
            assert_eq!(
                run(vec![left.clone(), right.clone()], op_and),
                vec![expected.clone()]
            );
            drop(keep);
        }
        for scalar in all.iter() {
            let expected = lifted_conjunction(&left, scalar).unwrap();
            assert_eq!(
                run(vec![left.clone(), scalar.clone()], op_and),
                vec![expected]
            );
        }
        let expected = lift_lanes([&left], |[x]| compute_inverted_value(x)).unwrap();
        assert_eq!(run(vec![left.clone()], op_not), vec![expected]);
        let numbers =
            |k: i64| Value::from_vector((0..25).map(|i| Value::from_int(i * k)).collect());
        let one = |v: Value| Value::from_vector(vec![v]);
        for (t, f) in [
            (Value::from_int(1), Value::from_int(0)),
            (numbers(1), numbers(-1)),
            (one(Value::from_int(1)), numbers(2)),
            (Value::from_bool(true), Value::both()),
        ] {
            let mask = || Value::from_vector(right_lanes(&all));
            let expected = lift_lanes([&t, &f, &mask()], |[t, f, m]| compute_selection(t, f, m));
            let answer = run(vec![t.clone(), f.clone(), mask()], op_select);
            assert_eq!(answer, vec![expected.unwrap()], "{t:?} {f:?}");
        }
    }
}
