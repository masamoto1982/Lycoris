//! The small-rational tier of a fused walk (`fused_block`): every number a
//! rational whose halves each fit a machine word, every other value a Boolean.
//!
//! The integer tier (`fused_block_int`) declines a block as soon as a value
//! can be a fraction — a `DIV` that is not a floor division, a fractional
//! literal or element — and the general tier then holds every value as a
//! `Fraction`, an enum with a heap-backed `BigInt` variant, dispatched op by
//! op. Most fractions a program meets are small, though, and a rational
//! whose numerator and denominator are each an `i64` is the `Small` form the
//! interpreted route stores them in too. This tier keeps them as `(i64, i64)`
//! pairs in registers and computes in `i128`, reducing each result with the
//! binary gcd, and steps aside the moment a result no longer fits a pair.
//!
//! Like the integer tier, it types the straight-line block against the input
//! types before the walk, so its charges are known up front: every number
//! here is `Small`, which the meter prices at one limb, so each arithmetic
//! Word costs `binary_numeric_work(1, 1)` and takes the scalar fast path, as
//! `LT`/`GT` and a numeric `EQ` do, and every result fits 64 bits for the
//! size ceiling.
//!
//! A number over zero — `(1, 0)`, `(-1, 0)`, `(0, 0)`, met as an element, a
//! literal or the quotient of a zero divisor — is a pair like any other: the
//! arithmetic that meets one is answered by the `Fraction`'s total law
//! (`small_rational`'s `_total` forms), and its answer is again one of the
//! two points or zero, so the walk stays here. The meter prices such a
//! pair at one limb as well, so the charges are the same. Each point is its
//! own floor and its own rounding. Only an order asked of `0/0` — `LT`, `GT`,
//! `MIN`, `MAX` — has no answer here: it is the ordinary walk's `domainMiss`
//! to project, and the walk steps aside.
//!
//! The results are the ones `Fraction` reaches: a rational's lowest-terms
//! form with a positive denominator is unique, so any correct reduction
//! agrees with it, which `fused_block_tests` holds the tier to.

use crate::interpreter::arithmetic::ExactArithmeticSchema;
use crate::interpreter::fused_block::{
    Charges, Compare, FusedBlock, FusedWalk, Op, Plain, Src, Ty,
};
use crate::interpreter::fused_block_general::{small, Elements, Gather};
use crate::interpreter::Interpreter;

use crate::types::small_rational::{self, add_total, div_total, mul_total, order_total};
use crate::types::Value;
use std::cmp::Ordering;

/// A number as a `small_rational` pair; a Boolean is `(0 or 1, 1)`.
type Pair = crate::types::small_rational::Pair;

#[derive(Debug, Clone, Copy)]
enum RSrc {
    Reg(usize),
    Const(Pair),
}

#[derive(Debug, Clone, Copy)]
enum Kind {
    Add,
    Sub,
    Mul,
    Div,
    FloorDiv,
    Min,
    Max,
    Floor,
    Round,
    Lt,
    Gt,
    Eq,
    Not,
    And,
    Select,
    Pow,
}

/// `dst = kind(a, b, c)`, each value written to a register of its own.
#[derive(Debug, Clone, Copy)]
struct Instr {
    kind: Kind,
    dst: usize,
    a: RSrc,
    b: RSrc,
    c: RSrc,
}

/// The block typed against its inputs and lowered to register code, with
/// what one run charges.
struct Program {
    instrs: Vec<Instr>,
    regs: usize,
    out: RSrc,
    out_ty: Ty,
    work_per_run: u64,
}

fn plain_pair(p: &Plain) -> Option<(Ty, Pair)> {
    match p {
        Plain::Num(f) => Some((Ty::Num, f.extract_i64_pair()?)),
        Plain::Bool(b) => Some((Ty::Bool, (i64::from(*b), 1))),
    }
}

fn compile(block: &FusedBlock, inputs: &[Ty]) -> Option<Program> {
    let (types, work_per_run) = block.types(inputs)?;
    let src = |s: Src| -> Option<RSrc> {
        Some(match s {
            Src::Reg(r) => RSrc::Reg(r),
            Src::Const(k) => RSrc::Const(plain_pair(&block.consts[k])?.1),
        })
    };
    let none = RSrc::Const((0, 1));
    let mut instrs = Vec::with_capacity(block.instrs.len());
    for (at, ins) in block.instrs.iter().enumerate() {
        let kind = match ins.op {
            Op::Arith(ExactArithmeticSchema::Add) => Kind::Add,
            Op::Arith(ExactArithmeticSchema::Sub) => Kind::Sub,
            Op::Arith(ExactArithmeticSchema::Mul) => Kind::Mul,
            Op::Arith(ExactArithmeticSchema::Div) => Kind::Div,
            Op::FloorDiv => Kind::FloorDiv,
            Op::Compare(Compare::Lt) => Kind::Lt,
            Op::Compare(Compare::Gt) => Kind::Gt,
            Op::Compare(Compare::Eq) => Kind::Eq,
            Op::Extremum { max: false } => Kind::Min,
            Op::Extremum { max: true } => Kind::Max,
            Op::Floor => Kind::Floor,
            Op::Round => Kind::Round,
            Op::Not => Kind::Not,
            Op::And => Kind::And,
            Op::Select => Kind::Select,
            Op::Pow => Kind::Pow,
            Op::Kernel(_) => return None,
        };
        let arity = ins.op.arity();
        instrs.push(Instr {
            kind,
            dst: inputs.len() + at,
            a: src(ins.a)?,
            b: if arity > 1 { src(ins.b)? } else { none },
            c: if arity > 2 { src(ins.c)? } else { none },
        });
    }
    let (out, out_ty) = match block.out {
        Src::Reg(r) => (RSrc::Reg(r), types[r]),
        Src::Const(k) => {
            let (ty, pair) = plain_pair(&block.consts[k])?;
            (RSrc::Const(pair), ty)
        }
    };
    Some(Program {
        regs: types.len(),
        instrs,
        out,
        out_ty,
        work_per_run,
    })
}

/// `DIV FLOOR` on pairs; a point over zero is its own floor. Kept out of
/// line, so the loop every other block runs does not carry a second division.
#[inline(never)]
fn floor_div(a: Pair, b: Pair) -> Option<Pair> {
    Some(match div_total(a, b)? {
        q if q.1 == 0 => q,
        (n, d) => (n.div_euclid(d), 1),
    })
}

impl Program {
    /// One run on `regs`, whose first registers hold the inputs. `None`
    /// when a result leaves the tier.
    fn run(&self, regs: &mut [Pair]) -> Option<Pair> {
        let read = |regs: &[Pair], s: RSrc| match s {
            RSrc::Reg(r) => regs[r],
            RSrc::Const(p) => p,
        };
        let truth = |b: bool| (i64::from(b), 1);
        for ins in &self.instrs {
            let (a, b) = (read(regs, ins.a), read(regs, ins.b));
            regs[ins.dst] = match ins.kind {
                Kind::Add => add_total(a, b, false)?,
                Kind::Sub => add_total(a, b, true)?,
                Kind::Mul => mul_total(a, b)?,
                Kind::Div => div_total(a, b)?,
                Kind::FloorDiv => floor_div(a, b)?,
                Kind::Pow => crate::interpreter::quickened::small_power(a, b)?,
                // The left operand on a tie, as MIN and MAX keep it.
                Kind::Min if order_total(b, a)? == Ordering::Less => b,
                Kind::Max if order_total(a, b)? == Ordering::Less => b,
                Kind::Min | Kind::Max => a,
                // A point over zero is its own floor and its own rounding.
                Kind::Floor | Kind::Round if a.1 == 0 => a,
                Kind::Floor => (a.0.div_euclid(a.1), 1),
                Kind::Round => (small_rational::round_half_away_from_zero(a.0, a.1), 1),
                Kind::Lt => truth(order_total(a, b)? == Ordering::Less),
                Kind::Gt => truth(order_total(a, b)? == Ordering::Greater),
                Kind::Eq => truth(a == b),
                Kind::Not => truth(a.0 == 0),
                Kind::And => truth(a.0 != 0 && b.0 != 0),
                Kind::Select => {
                    if read(regs, ins.c).0 != 0 {
                        a
                    } else {
                        b
                    }
                }
            };
        }
        Some(read(regs, self.out))
    }
}

/// Every element as a pair of one type, or `None` for an absent lane, a lane
/// wider than a pair, or mixed types.
fn elements(target: &Value) -> Option<(Ty, Elements<'_>)> {
    let elements = Elements::of(target)?;
    let Elements::Plains(plains) = &elements else {
        return Some((Ty::Num, elements));
    };
    let ty = plain_pair(plains.first()?)?.0;
    plains
        .iter()
        .all(|p| plain_pair(p).is_some_and(|(t, _)| t == ty))
        .then_some((ty, elements))
}

#[inline]
fn pair_at(elements: &Elements, i: usize) -> Pair {
    match elements {
        Elements::Columns(nums, dens) => (nums[i], dens[i]),
        Elements::Plains(plains) => plain_pair(&plains[i]).map_or((0, 1), |(_, p)| p),
    }
}

fn plain(ty: Ty, (n, d): Pair) -> Plain {
    match ty {
        Ty::Num => small((n, d)),
        Ty::Bool => Plain::Bool(n != 0),
    }
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
        Some(p) => Some(plain_pair(p)?),
        None => None,
    };
    let inputs: Vec<Ty> = seed.iter().map(|(t, _)| *t).chain([elem_ty]).collect();
    let program = compile(block, &inputs)?;
    match walk {
        FusedWalk::Filter if program.out_ty != Ty::Bool => return None,
        FusedWalk::Fold | FusedWalk::Scan if Some(program.out_ty) != seed.map(|(t, _)| t) => {
            return None
        }
        _ => {}
    }

    let runs = elements.len() as u64;
    let steps = block.steps_within_ceiling(interp, runs)?;
    let work = runs.checked_mul(program.work_per_run)?;
    if interp.numeric_work_used.checked_add(work)? > interp.runtime_limits.max_numeric_work {
        return None;
    }

    let mut regs = vec![(0i64, 1i64); program.regs];
    let mut accumulator = seed.map(|(_, p)| p);
    let mut gathered = Gather::with_capacity(match walk {
        FusedWalk::Map | FusedWalk::Filter => elements.len(),
        _ => 0,
    });
    let mut visited = Vec::new();
    let push = |gathered: &mut Gather, ty: Ty, pair: Pair| match ty {
        Ty::Num => gathered.push_pair(pair),
        Ty::Bool => gathered.push_truth(pair.0 != 0),
    };
    // `MAP` and `FILTER` over a Tensor's columns, the commonest walks, have a
    // loop of their own with nothing in it but the run and the result.
    let (columns, rest) = match (walk, &elements) {
        (FusedWalk::Map | FusedWalk::Filter, Elements::Columns(nums, dens)) => {
            (Some((nums, dens)), 0)
        }
        _ => (None, elements.len()),
    };
    if let Some((nums, dens)) = columns {
        let pairs = nums.iter().zip(dens.iter()).map(|(&n, &d)| (n, d));
        match (&mut gathered, walk, program.out_ty) {
            (Gather::Pairs(out_nums, out_dens), FusedWalk::Map, Ty::Num) => {
                for x in pairs {
                    regs[0] = x;
                    let (n, d) = program.run(&mut regs)?;
                    out_nums.push(n);
                    out_dens.push(d);
                }
            }
            (Gather::Pairs(out_nums, out_dens), FusedWalk::Filter, _) => {
                for x in pairs {
                    regs[0] = x;
                    if program.run(&mut regs)?.0 != 0 {
                        out_nums.push(x.0);
                        out_dens.push(x.1);
                    }
                }
            }
            _ => {
                for x in pairs {
                    regs[0] = x;
                    push(&mut gathered, program.out_ty, program.run(&mut regs)?);
                }
            }
        }
    }
    for i in 0..rest {
        let x = pair_at(&elements, i);
        match accumulator {
            Some(acc) => {
                regs[0] = acc;
                regs[1] = x;
            }
            None => regs[0] = x,
        }
        let result = program.run(&mut regs)?;
        match walk {
            FusedWalk::Map => push(&mut gathered, program.out_ty, result),
            FusedWalk::Filter => {
                if result.0 != 0 {
                    push(&mut gathered, elem_ty, x);
                }
            }
            FusedWalk::Scan => {
                visited.push(plain(program.out_ty, result).into_value());
                accumulator = Some(result);
            }
            FusedWalk::Fold => accumulator = Some(result),
        }
    }

    let value = match walk {
        FusedWalk::Map | FusedWalk::Filter => gathered.finish(),
        FusedWalk::Scan => Value::from_vector(visited),
        FusedWalk::Fold => plain(program.out_ty, accumulator?).into_value(),
    };
    let charges = Charges { steps, work };
    #[cfg(test)]
    RAT_RUNS.with(|c| c.set(c.get() + 1));
    Some((value, charges))
}

#[cfg(test)]
thread_local! {
    static RAT_RUNS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// Walks this tier answered on this thread, for tests that pin which tier ran.
#[cfg(test)]
pub(crate) fn rat_runs_on_this_thread() -> u64 {
    RAT_RUNS.with(|c| c.get())
}
