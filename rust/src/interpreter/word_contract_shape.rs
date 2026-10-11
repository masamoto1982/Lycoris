//! The zero shapes a body's numbers may take (LANG.CONTRACT.FIELD), inferred
//! by carrying a set of atoms (`zero_shape`) for every value on a simulated
//! stack through the body, never by running it.
//!
//! The walk assumes what a contract assumes — every number the body reads
//! from beneath its own pushes lies in the field — and asks each call what it
//! answers from the atoms its operands may hold: a Core Word by its declared
//! `zeroShapes` law, a User Word by its own inferred answer (or by anything at
//! all, when it is handed a point over zero, since its contract assumed it
//! would not be). A block a higher-order Word runs is walked the same way, its
//! inputs the atoms of the elements (and, for `FOLD` and `SCAN`, of the
//! accumulator, iterated until the set stops growing — six atoms bound the
//! iteration).
//!
//! What the walk learns is the set of every number the body pushes or
//! answers, and where a number first left the field: a call that answers a
//! point over zero from operands that hold none, or a literal written over
//! zero. `field` is the set's projection, so `[ 2 SQRT DIV ]` is `closed` —
//! the divisor is a positive number, wherever it was written — while
//! `[ LENGTH DIV ]` still leaves, a length being possibly zero.
//!
//! The walk gives up — and the contract falls back to the call-by-call join of
//! `word_contract_field` — wherever the stack is not knowable without running:
//! a Word of variable arity, `EXEC`, a block that is not the literal written
//! for it, a name nothing defines, an unbalanced literal, blocks nested past
//! `MAX_BLOCK_DEPTH`.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use super::word_contract::{AccumulatedContract, WordContract};
use super::word_contract_flow::{note_bound_names, BoundNames};
use super::zero_shape::{apply_law, ZeroAtoms};
use super::Interpreter;
use crate::coreword_registry::FieldClosure;
use crate::coreword_registry::{get_declared_word, ZeroShapeLaw};
use crate::kernel::generated::Arity;
use crate::types::Token;

/// The bound on `FOLD`'s accumulator iteration: each round that does not stop
/// adds an atom, and there are six.
const MAX_FOLD_ROUNDS: usize = 7;

/// How deep blocks may nest before the walk gives up: each `FOLD` level may
/// walk its block once per round, so the work grows with this power.
const MAX_BLOCK_DEPTH: usize = 6;

/// What the walk learned about one body.
#[derive(Clone, Debug, Default)]
pub(crate) struct ShapeOutcome {
    /// Every number the body pushes or answers, from finite operands.
    pub(crate) produced: ZeroAtoms,
    /// What the body leaves on the stack: the atoms a caller receives.
    pub(crate) answers: ZeroAtoms,
    /// Where a number first left the field, in body order.
    pub(crate) exits: Vec<String>,
}

#[derive(Clone)]
struct Slot {
    atoms: ZeroAtoms,
    /// The token range of the code literal this value was written as.
    block: Option<(usize, usize)>,
}

struct ShapeSim<'t> {
    tokens: &'t [Token],
    stack: Vec<Slot>,
    /// What a value read from beneath the body's own pushes may hold.
    input: ZeroAtoms,
    /// How many such values the body read.
    reads_below: usize,
    produced: ZeroAtoms,
    exits: Vec<String>,
    bound: HashMap<String, ZeroAtoms>,
    /// How many blocks enclose this walk.
    depth: usize,
}

fn note_exit(exits: &mut Vec<String>, exit: &str) {
    if !exits.iter().any(|e| e == exit) {
        exits.push(exit.to_string());
    }
}

/// The index of the `]` that closes the `[` at `open`.
fn matching_close(tokens: &[Token], open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (i, token) in tokens.iter().enumerate().skip(open) {
        match token {
            Token::VectorStart => depth += 1,
            Token::VectorEnd => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

fn literal_atoms(token: &Token) -> ZeroAtoms {
    match token {
        Token::Number(literal) => literal
            .value()
            .map_or(ZeroAtoms::NONE, |f| ZeroAtoms::of_fraction(&f)),
        Token::Value(value) => ZeroAtoms::of_value(value),
        _ => ZeroAtoms::NONE,
    }
}

fn literal_spelling(token: &Token) -> &str {
    match token {
        Token::Number(literal) => literal.lexeme(),
        _ => "a literal over zero",
    }
}

impl<'t> ShapeSim<'t> {
    fn new(
        tokens: &'t [Token],
        input: ZeroAtoms,
        bound: HashMap<String, ZeroAtoms>,
        depth: usize,
    ) -> Self {
        ShapeSim {
            depth,
            tokens,
            stack: Vec::new(),
            input,
            reads_below: 0,
            produced: ZeroAtoms::NONE,
            exits: Vec::new(),
            bound,
        }
    }

    fn push(&mut self, atoms: ZeroAtoms) {
        self.stack.push(Slot { atoms, block: None });
    }

    fn pop(&mut self) -> Slot {
        self.stack.pop().unwrap_or_else(|| {
            self.reads_below += 1;
            Slot {
                atoms: self.input,
                block: None,
            }
        })
    }

    /// Pop `n` operands, deepest first.
    fn pop_n(&mut self, n: usize) -> Vec<Slot> {
        let mut operands: Vec<Slot> = (0..n).map(|_| self.pop()).collect();
        operands.reverse();
        operands
    }

    /// Record a literal the body writes; any number it holds over zero is
    /// where the body leaves the field.
    fn note_literal(&mut self, token: &Token) -> ZeroAtoms {
        let atoms = literal_atoms(token);
        self.produced = self.produced.union(atoms);
        if !atoms.is_finite() {
            note_exit(&mut self.exits, literal_spelling(token));
        }
        atoms
    }

    /// Record a call's answer; it is an exit when it reaches past the field
    /// from operands that held nothing past it.
    fn note_answer(&mut self, name: &str, operands: ZeroAtoms, answer: ZeroAtoms) {
        self.produced = self.produced.union(answer);
        if operands.is_finite() && !answer.is_finite() {
            note_exit(&mut self.exits, name);
        }
    }

    fn run(&mut self, interp: &mut Interpreter, visiting: &mut HashSet<String>) -> Option<()> {
        let mut idx = 0;
        while idx < self.tokens.len() {
            match &self.tokens[idx] {
                token @ (Token::Number(_) | Token::String(_) | Token::Value(_)) => {
                    let atoms = self.note_literal(token);
                    self.push(atoms);
                }
                Token::VectorStart => {
                    let close = matching_close(self.tokens, idx)?;
                    let mut atoms = ZeroAtoms::NONE;
                    for token in &self.tokens[idx + 1..close] {
                        atoms = atoms.union(self.note_literal(token));
                    }
                    self.stack.push(Slot {
                        atoms,
                        block: Some((idx + 1, close)),
                    });
                    idx = close;
                }
                Token::VectorEnd => return None,
                Token::Symbol(symbol) => {
                    let canonical = crate::word_name::canonical_word_name(symbol);
                    if let Some(atoms) = self.bound.get(&canonical.to_uppercase()) {
                        let atoms = *atoms;
                        self.push(atoms);
                    } else if canonical == "BIND" {
                        self.bind(idx);
                    } else {
                        self.call(interp, visiting, &canonical)?;
                    }
                }
            }
            idx += 1;
        }
        Some(())
    }

    /// `BIND`: the names written just before it take what the value below
    /// them may hold (a destructured Vector gives each name all its atoms).
    fn bind(&mut self, idx: usize) {
        let mut names = BoundNames::new();
        note_bound_names(&mut names, "BIND", self.tokens, idx);
        let operands = self.pop_n(2);
        for name in names {
            self.bound.insert(name, operands[0].atoms);
        }
    }

    fn call(
        &mut self,
        interp: &mut Interpreter,
        visiting: &mut HashSet<String>,
        name: &str,
    ) -> Option<()> {
        let (dep_name, dep_def) = interp.resolve_word_entry(name)?;
        if dep_def.is_builtin {
            let word = get_declared_word(&dep_name)?;
            let (Arity::Fixed(inputs), Arity::Fixed(outputs)) =
                (word.stack_inputs, word.stack_outputs)
            else {
                return None;
            };
            return self.call_core(
                interp,
                visiting,
                &dep_name,
                word.zero_shapes,
                inputs.into(),
                outputs.into(),
            );
        }
        let contract = interp.infer_word_contract_inner(&dep_name, &dep_def, visiting)?;
        self.call_user(&dep_name, &contract)
    }

    fn call_user(&mut self, name: &str, contract: &Arc<WordContract>) -> Option<()> {
        let super::word_contract::ContractFlow::Fixed { consumes, produces } = contract.flow else {
            return None;
        };
        let operands = self
            .pop_n(consumes.into())
            .iter()
            .fold(ZeroAtoms::NONE, |acc, s| acc.union(s.atoms));
        // The callee's contract assumed operands in the field; handed a point
        // over zero, it may answer anything.
        let answer = if operands.is_finite() {
            contract.zero_answers
        } else {
            ZeroAtoms::ALL
        };
        self.note_answer(name, operands, answer);
        for _ in 0..produces {
            self.push(answer);
        }
        Some(())
    }

    fn call_core(
        &mut self,
        interp: &mut Interpreter,
        visiting: &mut HashSet<String>,
        name: &str,
        law: ZeroShapeLaw,
        inputs: usize,
        outputs: usize,
    ) -> Option<()> {
        let operands = self.pop_n(inputs);
        let all = operands
            .iter()
            .fold(ZeroAtoms::NONE, |acc, s| acc.union(s.atoms));
        let answer = match law {
            ZeroShapeLaw::Exec => return None,
            ZeroShapeLaw::Map | ZeroShapeLaw::Filter => {
                let [elements, block] = operands.as_slice() else {
                    return None;
                };
                let ran = self.run_block(interp, visiting, block, elements.atoms, 1)?;
                if law == ZeroShapeLaw::Map {
                    ran
                } else {
                    elements.atoms
                }
            }
            ZeroShapeLaw::Fold => {
                let [elements, init, block] = operands.as_slice() else {
                    return None;
                };
                let mut carried = elements.atoms.union(init.atoms);
                let mut settled = false;
                for _ in 0..MAX_FOLD_ROUNDS {
                    let ran = self.run_block(interp, visiting, block, carried, 2)?;
                    if ran.is_subset(carried) {
                        settled = true;
                        break;
                    }
                    carried = carried.union(ran);
                }
                if !settled {
                    return None;
                }
                carried
            }
            _ => apply_law(law, &operands.iter().map(|s| s.atoms).collect::<Vec<_>>()),
        };
        if matches!(law, ZeroShapeLaw::Map | ZeroShapeLaw::Fold) {
            // The block's own Words are where it left, already named.
            self.produced = self.produced.union(answer);
        } else {
            self.note_answer(name, all, answer);
        }
        for _ in 0..outputs {
            self.push(answer);
        }
        Some(())
    }

    /// Walk the code literal a higher-order Word runs, its `arity` operands
    /// each holding `input`, and answer what it leaves.
    fn run_block(
        &mut self,
        interp: &mut Interpreter,
        visiting: &mut HashSet<String>,
        block: &Slot,
        input: ZeroAtoms,
        arity: usize,
    ) -> Option<ZeroAtoms> {
        let (start, end) = block.block?;
        if self.depth >= MAX_BLOCK_DEPTH {
            return None;
        }
        let mut inner = ShapeSim::new(
            &self.tokens[start..end],
            input,
            self.bound.clone(),
            self.depth + 1,
        );
        inner.run(interp, visiting)?;
        self.produced = self.produced.union(inner.produced);
        for exit in &inner.exits {
            note_exit(&mut self.exits, exit);
        }
        let mut answer = inner
            .stack
            .iter()
            .fold(ZeroAtoms::NONE, |acc, s| acc.union(s.atoms));
        // An operand the block never took is still there for it to answer.
        if inner.reads_below < arity {
            answer = answer.union(input);
        }
        Some(answer)
    }
}

impl Interpreter {
    /// The zero shapes of a body, or `None` where the stack is not knowable
    /// without running it.
    pub(crate) fn infer_zero_shapes(
        &mut self,
        body: &[Token],
        visiting: &mut HashSet<String>,
    ) -> Option<ShapeOutcome> {
        let mut sim = ShapeSim::new(body, ZeroAtoms::FINITE, HashMap::new(), 0);
        sim.run(self, visiting)?;
        let answers = sim
            .stack
            .iter()
            .fold(ZeroAtoms::NONE, |acc, s| acc.union(s.atoms));
        Some(ShapeOutcome {
            produced: sim.produced.union(answers),
            answers,
            exits: sim.exits,
        })
    }
}

impl Interpreter {
    /// Settle a body's `field`, exits and zero shapes: walked over the stack
    /// where that can be done, and where it cannot, the call-by-call join in
    /// `acc` stands and the shapes are the widest set it admits. Answers the
    /// shapes, what the body leaves for a caller, and whether the walk ran.
    pub(super) fn settle_zero_shapes(
        &mut self,
        acc: &mut AccumulatedContract,
        body: &[Token],
        visiting: &mut HashSet<String>,
    ) -> (ZeroAtoms, ZeroAtoms, bool) {
        match self.infer_zero_shapes(body, visiting) {
            Some(outcome) => {
                acc.field = if outcome.produced.is_finite() {
                    FieldClosure::Closed
                } else {
                    FieldClosure::Leaving
                };
                acc.field_exits = outcome.exits;
                (outcome.produced, outcome.answers, true)
            }
            None => {
                let widest = match acc.field {
                    FieldClosure::Closed => ZeroAtoms::FINITE,
                    FieldClosure::Leaving => ZeroAtoms::ALL,
                };
                (widest, widest, false)
            }
        }
    }
}
