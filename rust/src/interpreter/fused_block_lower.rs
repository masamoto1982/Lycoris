//! Lowering a block's compiled code (`engine`) to the straight-line ops of a
//! fused walk (`fused_block`), User Word calls included.
//!
//! A User Word called from a block is inlined: its body's code is lowered in
//! place, in a frame of its own, and the call costs the one execution step the
//! engine charges for it, carried in `steps_per_run`. The call stack, the
//! depth counter and failure attribution exist for runs that fail, and a fused
//! walk that meets a failure is abandoned. A Word body is a barrier frame: it
//! reads only the names it binds itself, so a name it did not bind is the
//! ordinary walk's to report, and the call depth is bounded exactly as
//! `execute_unbound_word` bounds it.

use std::collections::HashMap;

use crate::interpreter::arithmetic::ExactArithmeticSchema;
use crate::interpreter::engine::{Code, Op as CodeOp};
use crate::interpreter::fused_block::{Compare, FusedBlock, Op, Plain};
use crate::interpreter::interpreter_core::MAX_USER_WORD_DEPTH;
use crate::interpreter::Interpreter;
use crate::kernel::generated::WordId;
use crate::types::Value;

pub(crate) fn word_op(id: WordId) -> Option<Op> {
    Some(match id {
        WordId::Add => Op::Arith(ExactArithmeticSchema::Add),
        WordId::Sub => Op::Arith(ExactArithmeticSchema::Sub),
        WordId::Mul => Op::Arith(ExactArithmeticSchema::Mul),
        WordId::Div => Op::Arith(ExactArithmeticSchema::Div),
        WordId::Lt => Op::Compare(Compare::Lt),
        WordId::Gt => Op::Compare(Compare::Gt),
        WordId::Eq => Op::Compare(Compare::Eq),
        WordId::Min => Op::Extremum { max: false },
        WordId::Max => Op::Extremum { max: true },
        WordId::Floor => Op::Floor,
        WordId::Round => Op::Round,
        WordId::Not => Op::Not,
        WordId::And => Op::And,
        WordId::Select => Op::Select,
        WordId::Pow => Op::Pow,
        _ => Op::Kernel(crate::interpreter::fusion_contract::kernel(id)?),
    })
}

/// The one name a `BIND` of `value` would make, when it makes exactly one
/// name it may make.
pub(crate) fn bindable_name(interp: &Interpreter, value: &Value) -> Option<String> {
    let names = crate::interpreter::bindings::binding_names(value).ok()?;
    let [name] = names.as_slice() else {
        return None;
    };
    interp.check_bindable_name(name).ok()?;
    Some(name.to_uppercase())
}

struct Lowering<'a> {
    interp: &'a Interpreter,
    ops: Vec<Op>,
    slots: usize,
    depth: usize,
    calls: u64,
    reads_outer: bool,
    lane_pushes: Vec<usize>,
}

pub(crate) fn lower(code: &Code, interp: &Interpreter, inputs: usize) -> Option<FusedBlock> {
    let mut lowering = Lowering {
        interp,
        ops: Vec::with_capacity(code.ops.len()),
        slots: 0,
        depth: inputs,
        calls: 0,
        reads_outer: false,
        lane_pushes: Vec::new(),
    };
    lowering.line(&code.ops, &mut HashMap::new(), true, interp.call_depth)?;
    if lowering.depth != 1 {
        return None;
    }
    let words = lowering.ops.iter().filter(|op| op.is_word()).count() as u64;
    let mut block = FusedBlock {
        ops: lowering.ops,
        slots: lowering.slots,
        steps_per_run: words + lowering.calls,
        reads_outer: lowering.reads_outer,
        lane_pushes: lowering.lane_pushes,
        lanes_from_element: None,
        lanes_from_seed: None,
        int_programs: Default::default(),
    };
    block.lanes_from_element = block.lane_flow(&[false]);
    block.lanes_from_seed = block.lane_flow(&[true, false]);
    Some(block)
}

impl Lowering<'_> {
    fn line(
        &mut self,
        source: &[CodeOp],
        frame: &mut HashMap<String, usize>,
        sees_outer: bool,
        call_depth: usize,
    ) -> Option<()> {
        let mut i = 0;
        while i < source.len() {
            let next_is = |id: WordId| matches!(source.get(i + 1), Some(CodeOp::Word(w) | CodeOp::Scalar(_, w)) if w.id == id);
            let (op, pops, pushes) = match &source[i] {
                CodeOp::Push(value) => match Plain::of(value) {
                    Some(plain) => (Op::Push(plain), 0, 1),
                    None => {
                        if !next_is(WordId::Bind) {
                            return None;
                        }
                        let name = bindable_name(self.interp, value)?;
                        i += 1;
                        (Op::Bind(self.slot(frame, name)), 1, 0)
                    }
                },
                CodeOp::Bracket { value, .. } if next_is(WordId::Length) => {
                    if value.absence_metadata().is_some() || !value.is_vector() || value.is_nil() {
                        return None;
                    }
                    let len = crate::types::fraction::Fraction::from(value.len() as i64);
                    i += 1;
                    (Op::Const(Plain::Num(len)), 0, 1)
                }
                CodeOp::Bracket { value, .. } => {
                    let lane = crate::interpreter::fused_block::one_lane_of(value)?;
                    self.lane_pushes.push(self.ops.len());
                    (Op::Push(lane), 0, 1)
                }
                CodeOp::Constant(value, _) => match value.as_truth() {
                    Some(b) if value.absence_metadata().is_none() => (Op::PushWord(b), 0, 1),
                    _ => return None,
                },
                CodeOp::Bind { name, .. } => {
                    let name = bindable_name(self.interp, name)?;
                    (Op::Bind(self.slot(frame, name)), 1, 0)
                }
                CodeOp::Word(word) | CodeOp::Scalar(_, word) => {
                    let op = word_op(word.id)?;
                    let pops = match op {
                        Op::Floor | Op::Round | Op::Not => 1,
                        Op::Select => 3,
                        Op::Kernel(kernel) => kernel.arity,
                        _ => 2,
                    };
                    (op, pops, 1)
                }
                CodeOp::Name(name) | CodeOp::Load { key: name, .. } => {
                    let name = crate::word_name::canonical_word_name(name);
                    match frame.get(name.as_ref()) {
                        Some(slot) => (Op::Load(*slot), 0, 1),
                        None => match self.interp.lookup_binding(name.as_ref()) {
                            Some(value) if sees_outer => {
                                self.reads_outer = true;
                                (Op::Push(Plain::of(&value)?), 0, 1)
                            }
                            Some(_) => return None,
                            None => {
                                self.call(name.as_ref(), call_depth)?;
                                i += 1;
                                continue;
                            }
                        },
                    }
                }
                CodeOp::Raise(_) => return None,
            };
            if self.depth < pops {
                return None;
            }
            self.depth = self.depth - pops + pushes;
            self.ops.push(op);
            i += 1;
        }
        Some(())
    }

    /// The slot `name` is bound in, a fresh one the first time.
    fn slot(&mut self, frame: &mut HashMap<String, usize>, name: String) -> usize {
        let fresh = self.slots;
        let slot = *frame.entry(name).or_insert(fresh);
        if slot == fresh {
            self.slots += 1;
        }
        slot
    }

    fn call(&mut self, name: &str, call_depth: usize) -> Option<()> {
        if call_depth + 1 > MAX_USER_WORD_DEPTH {
            return None;
        }
        let def = self.interp.user_words.get(name)?.clone();
        if def.body.is_empty() {
            return None;
        }
        let code = self.interp.body_code(&def);
        self.calls += 1;
        self.line(&code.ops, &mut HashMap::new(), false, call_depth + 1)
    }
}
