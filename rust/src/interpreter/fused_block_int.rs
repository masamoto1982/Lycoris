//! The integer tier of a fused walk (`fused_block`): every value a
//! machine-word integer or a Boolean.
//!
//! A block is straight-line, so once the types of the elements and the seed
//! are known, the type of every value it makes is too. The block's program is
//! checked against them before the walk starts — `compile` — and every op
//! that would leave this tier (an operand of the wrong domain, a `DIV` that
//! makes a fraction) sends the walk to the next tier before anything has run. What
//! remains runs on `i64`, a Boolean as 0 or 1, with checked arithmetic: an
//! overflow is the one thing left to discover while running, and it, too,
//! falls to the general tier.
//!
//! The same typing settles what each run is charged, so the whole walk's
//! charges are known up front: every operand here is a `Small` fraction,
//! which the meter prices at one limb, so each arithmetic Word costs
//! `binary_numeric_work(1, 1)` and takes the scalar fast path, as `LT`/`GT`
//! and a scalar `EQ` do. Every result fits 64 bits, so the size ceiling can
//! refuse one only when it is set below that, and then this tier steps aside.
//!
//! `DIV FLOOR` (`Op::FloorDiv`) is floor division on integers, the quotient
//! half of `a - b * floor(a / b)`, still charged as the two Words it is. A zero divisor's quotient is one of
//! the two points over zero, which this tier does not hold: the walk goes
//! on to the next tier, which answers it.

use crate::interpreter::arithmetic::ExactArithmeticSchema;
use crate::interpreter::fused_block::{
    self as fb, Charges, Compare, FusedBlock, FusedWalk, Op, Plain,
};
use crate::interpreter::fused_block_general::promote;
use crate::interpreter::fused_block_reg::{Instr, Kind, RegProgram, Src};
use crate::interpreter::Interpreter;
use crate::types::fraction::{Fraction, FractionRepr};
use crate::types::{DenseTensor, Value, ValueData};
use std::borrow::Cow;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ty {
    Int,
    Bool,
    /// The quotient of a `DIV` not followed by `FLOOR`: an integer where the
    /// division is exact and otherwise [`POISON`], a fraction this tier
    /// cannot hold. It may only be bound, read back, or be a candidate of
    /// `SELECT` — so a fraction the block computes and then does not choose
    /// (the Collatz step's `N 2 DIV` for odd `N`) never sends the walk to the
    /// small-rational tier. A poisoned lane that reaches a `MAP`'s result
    /// does, from the start.
    Exact,
}

/// The lane value of an inexact quotient (`Ty::Exact`). An exact quotient
/// that happens to be `i64::MIN` reads as one too, which costs only a walk
/// sent on to the next tier.
pub(crate) const POISON: i64 = i64::MIN;

/// The block typed and lowered to register code, per input types, kept on
/// the block (`FusedBlock::int_programs`) so a block walked again and again —
/// the inner `FOLD` of `[ [ 0 ] [ ADD ] FOLD ] MAP` — is compiled once. Both
/// depend on the block and the input types alone; what a walk is charged is
/// checked against the ceilings on every walk.
#[derive(Debug, Default)]
pub(crate) struct IntPrograms(std::sync::Mutex<Vec<KeptProgram>>);

/// One input-type combination and what compiling the block for it gave.
type KeptProgram = (Vec<Ty>, Option<Arc<Compiled>>);

#[derive(Debug)]
struct Compiled {
    program: RegProgram,
    out: Ty,
    work_per_run: u64,
}

/// Input-type combinations kept per block; a block meets one or two.
const KEPT_PROGRAMS: usize = 4;

impl Clone for IntPrograms {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl IntPrograms {
    fn get(&self, block: &FusedBlock, inputs: &[Ty]) -> Option<Arc<Compiled>> {
        let mut kept = self
            .0
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if let Some((_, compiled)) = kept.iter().find(|(types, _)| types == inputs) {
            return compiled.clone();
        }
        let compiled = compile(block, inputs).map(Arc::new);
        if kept.len() < KEPT_PROGRAMS {
            kept.push((inputs.to_vec(), compiled.clone()));
        }
        compiled
    }
}

fn small_integer(f: &Fraction) -> Option<i64> {
    match f.repr {
        FractionRepr::Small(n, 1) => Some(n),
        _ => None,
    }
}

fn plain_int(p: &Plain) -> Option<(Ty, i64)> {
    match p {
        Plain::Num(f) => Some((Ty::Int, small_integer(f)?)),
        Plain::Bool(b) => Some((Ty::Bool, i64::from(*b))),
    }
}

/// The program as register code on machine words, for inputs of kinds
/// `inputs`, or `None` when it does not stay on them. `FLOOR` and `ROUND` of
/// an integer are the integer, so they compile to nothing; `DIV` without
/// `FLOOR` is `Ty::Exact`.
fn compile(block: &FusedBlock, inputs: &[Ty]) -> Option<Compiled> {
    let kinds: smallvec::SmallVec<[fb::Ty; 2]> = inputs
        .iter()
        .map(|t| match t {
            Ty::Bool => fb::Ty::Bool,
            _ => fb::Ty::Num,
        })
        .collect();
    let (_, work_per_run) = block.types(&kinds)?;
    let mut values: Vec<(Src, Ty)> = (0..inputs.len())
        .map(|i| (Src::Reg(i), inputs[i]))
        .collect();
    let mut instrs = Vec::with_capacity(block.instrs.len());
    let none = (Src::Const(0), Ty::Int);
    for ins in &block.instrs {
        let get = |s: fb::Src| match s {
            fb::Src::Reg(r) => Some(values[r]),
            fb::Src::Const(k) => {
                let (ty, n) = plain_int(&block.consts[k])?;
                Some((Src::Const(n), ty))
            }
        };
        let arity = ins.op.arity();
        let (a, ta) = get(ins.a)?;
        let (b, tb) = if arity > 1 { get(ins.b)? } else { none };
        let (c, tc) = if arity > 2 { get(ins.c)? } else { none };
        // A quotient that may be a fraction may only be chosen by `SELECT`.
        let exact = ta == Ty::Exact || tb == Ty::Exact;
        if tc == Ty::Exact || (exact && !matches!(ins.op, Op::Select)) {
            return None;
        }
        let (kind, ty) = match ins.op {
            Op::Floor | Op::Round => {
                values.push((a, Ty::Int));
                continue;
            }
            Op::Arith(ExactArithmeticSchema::Add) => (Kind::Add, Ty::Int),
            Op::Arith(ExactArithmeticSchema::Sub) => (Kind::Sub, Ty::Int),
            Op::Arith(ExactArithmeticSchema::Mul) => (Kind::Mul, Ty::Int),
            Op::Arith(ExactArithmeticSchema::Div) => (Kind::ExactDiv, Ty::Exact),
            Op::FloorDiv => (Kind::FloorDiv, Ty::Int),
            Op::Pow => (Kind::Pow, Ty::Int),
            Op::Extremum { max: false } => (Kind::Min, Ty::Int),
            Op::Extremum { max: true } => (Kind::Max, Ty::Int),
            Op::Compare(Compare::Lt) => (Kind::Lt, Ty::Bool),
            Op::Compare(Compare::Gt) => (Kind::Gt, Ty::Bool),
            Op::Compare(Compare::Eq) => (Kind::Eq, Ty::Bool),
            Op::Not => (Kind::Not, Ty::Bool),
            Op::And => (Kind::And, Ty::Bool),
            Op::Select => (Kind::Select, if exact { Ty::Exact } else { ta }),
            Op::Kernel(_) => return None,
        };
        let dst = inputs.len() + instrs.len();
        instrs.push(Instr { kind, dst, a, b, c });
        values.push((Src::Reg(dst), ty));
    }
    let (out, out_ty) = match block.out {
        fb::Src::Reg(r) => values[r],
        fb::Src::Const(k) => {
            let (ty, n) = plain_int(&block.consts[k])?;
            (Src::Const(n), ty)
        }
    };
    Some(Compiled {
        program: RegProgram::new(instrs, inputs.len(), out),
        out: out_ty,
        work_per_run,
    })
}

/// Every element as `(type, i64)`, borrowed in place from a pure-integer
/// dense Tensor's numerator column, or `None`. A dense Tensor holds numbers
/// only (`dense_columns`), and `is_pure_integer` is false when any lane is a
/// fraction or a point over zero, so a column it vouches for is integers.
fn elements(target: &Value) -> Option<(Ty, Cow<'_, [i64]>)> {
    match &target.data {
        ValueData::Tensor { data, shape } if shape.len() == 1 && data.is_pure_integer => {
            Some((Ty::Int, Cow::Borrowed(&data.numerators)))
        }
        ValueData::Vector(items) => {
            let lanes: Vec<(Ty, i64)> = items
                .iter()
                .map(|v| plain_int(&Plain::of(v)?))
                .collect::<Option<_>>()?;
            let ty = lanes.first()?.0;
            lanes.iter().all(|(t, _)| *t == ty).then_some(())?;
            Some((ty, Cow::Owned(lanes.into_iter().map(|(_, n)| n).collect())))
        }
        _ => None,
    }
}

fn plain(ty: Ty, n: i64) -> Plain {
    match ty {
        Ty::Int | Ty::Exact => Plain::Num(Fraction::from_repr(FractionRepr::Small(n, 1))),
        Ty::Bool => Plain::Bool(n != 0),
    }
}

/// `Value::from_vector_promoted` of a run of values of one type: for a
/// non-empty run of integers, `from_fractions` of `n/1` lanes is exactly
/// these columns.
fn promote_lanes(ty: Ty, lanes: Vec<i64>) -> Value {
    if ty == Ty::Int && !lanes.is_empty() {
        let shape = vec![lanes.len()];
        let ones = vec![1; lanes.len()];
        let tensor = DenseTensor::from_columns(lanes, ones, shape.clone(), true);
        return Value::new(
            ValueData::Tensor {
                data: Arc::new(tensor),
                shape: Arc::new(shape),
            },
            None,
        );
    }
    // Booleans are never promoted: a Vector of them is what `from_truths`
    // builds, without a `Plain` and a pass over the children per lane.
    if ty == Ty::Bool {
        return Value::from_truths(lanes.into_iter().map(|n| n != 0));
    }
    promote(lanes.into_iter().map(|n| plain(ty, n)).collect())
}

pub(crate) fn run(
    block: &FusedBlock,
    interp: &Interpreter,
    walk: FusedWalk,
    target: &Value,
    seed: Option<&Plain>,
) -> Option<(Value, Charges)> {
    if interp.runtime_limits.max_bigint_bits < 64 {
        return None;
    }
    let (elem_ty, elements) = elements(target)?;
    let seed = match seed {
        Some(p) => Some(plain_int(p)?),
        None => None,
    };
    let inputs: smallvec::SmallVec<[Ty; 2]> =
        seed.iter().map(|(t, _)| *t).chain([elem_ty]).collect();
    let compiled = block.int_programs.get(block, &inputs)?;
    let program = &compiled.program;
    match walk {
        FusedWalk::Filter if compiled.out != Ty::Bool => return None,
        FusedWalk::Fold | FusedWalk::Scan if Some(compiled.out) != seed.map(|(t, _)| t) => {
            return None
        }
        _ => {}
    }

    let runs = elements.len() as u64;
    let steps = block.steps_within_ceiling(interp, runs)?;
    let work = runs.checked_mul(compiled.work_per_run)?;
    if interp.numeric_work_used.checked_add(work)? > interp.runtime_limits.max_numeric_work {
        return None;
    }

    let value = match (walk, seed) {
        (FusedWalk::Map, _) => {
            let mut results = Vec::with_capacity(elements.len());
            program.run_columns(&elements, |_, out| results.extend_from_slice(out))?;
            match compiled.out {
                // A fraction was chosen for some lane: the next tier's walk.
                Ty::Exact if results.contains(&POISON) => return None,
                Ty::Exact => promote_lanes(Ty::Int, results),
                ty => promote_lanes(ty, results),
            }
        }
        (FusedWalk::Filter, _) => {
            // Every element is written and only a kept one is counted, so
            // the loop has no branch on the predicate.
            let mut kept = vec![0; elements.len()];
            let mut count = 0;
            program.run_columns(&elements, |xs, keep| {
                for (&x, &k) in xs.iter().zip(keep) {
                    kept[count] = x;
                    count += usize::from(k != 0);
                }
            })?;
            kept.truncate(count);
            promote_lanes(elem_ty, kept)
        }
        (FusedWalk::Scan, Some((_, acc))) => {
            let mut visited = Vec::with_capacity(elements.len());
            program.run_accumulating(&elements, acc, |a| {
                visited.push(plain(compiled.out, a).into_value());
            })?;
            Value::from_vector(visited)
        }
        (FusedWalk::Fold, Some((_, acc))) => plain(
            compiled.out,
            program.run_accumulating(&elements, acc, |_| {})?,
        )
        .into_value(),
        (FusedWalk::Fold | FusedWalk::Scan, None) => return None,
    };
    let charges = Charges { steps, work };
    Some((value, charges))
}
