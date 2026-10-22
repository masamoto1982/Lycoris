//! The sample language: a stack, a handful of Core words, and User words
//! defined with `DEF`.
//!
//! ```text
//! 1 2 ADD                 # 3
//! [ 1 2 3 ] 10 MUL        # [ 10 20 30 ]
//! [ DUP MUL ] 'SQUARE' DEF
//! [ 1 2 3 ] [ SQUARE ] MAP
//! ```
//!
//! `[ ... ]` builds a Vector without running anything inside it: words become
//! symbols. `DEF` turns such a Vector into a User word, and `MAP` runs one on
//! each element.

use crate::tokenizer::{tokenize, Token};
use crate::value::{Rational, Value};
use std::collections::BTreeMap;

pub const DEFAULT_MAX_STEPS: u64 = 1_000_000;
/// How deeply User words may call each other before the run is refused,
/// so runaway recursion fails cleanly instead of overflowing the native stack.
pub const MAX_CALL_DEPTH: usize = 128;

/// A failed run. `category` is the machine-readable name a host branches on;
/// `message` is display text.
#[derive(Debug, Clone, PartialEq)]
pub struct ExecError {
    pub category: &'static str,
    pub message: String,
}

impl ExecError {
    fn new(category: &'static str, message: impl Into<String>) -> Self {
        ExecError {
            category,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

fn err(message: impl Into<String>) -> ExecError {
    ExecError::new("runtimeError", message)
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserWord {
    pub body: Vec<Value>,
    pub description: Option<String>,
}

impl UserWord {
    /// The body as source, without the surrounding brackets.
    pub fn definition(&self) -> String {
        self.body
            .iter()
            .map(Value::display)
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn calls(&self, name: &str) -> bool {
        fn walk(values: &[Value], name: &str) -> bool {
            values.iter().any(|v| match v {
                Value::Symbol(s) => s == name,
                Value::Vector(items) => walk(items, name),
                _ => false,
            })
        }
        walk(&self.body, name)
    }
}

/// `(name, summary, syntax)` for the dictionary panel.
pub const CORE_WORDS: &[(&str, &str, &str)] = &[
    ("ADD", "ADD — add two numbers or vectors", "1 2 ADD"),
    ("SUB", "SUB — subtract the top from the second", "5 2 SUB"),
    ("MUL", "MUL — multiply two numbers or vectors", "3 4 MUL"),
    ("DIV", "DIV — divide the second by the top", "1 3 DIV"),
    ("EQ", "EQ — whether two values are equal", "1 1 EQ"),
    (
        "LT",
        "LT — whether the second is less than the top",
        "1 2 LT",
    ),
    ("DUP", "DUP — copy the top value", "1 DUP"),
    ("DROP", "DROP — discard the top value", "1 DROP"),
    ("SWAP", "SWAP — exchange the top two values", "1 2 SWAP"),
    (
        "PRINT",
        "PRINT — write the top value to the output",
        "'hello' PRINT",
    ),
    (
        "MAP",
        "MAP — run a block on each element",
        "[ 1 2 ] [ DUP MUL ] MAP",
    ),
    (
        "DEF",
        "DEF — define a User word",
        "[ DUP MUL ] 'SQUARE' DEF",
    ),
    ("DEL", "DEL — delete a User word", "'SQUARE' DEL"),
];

fn is_core(name: &str) -> bool {
    CORE_WORDS.iter().any(|(n, _, _)| *n == name)
}

/// Parse source into the values a run executes, with `[ ... ]` already built.
pub fn parse(source: &str) -> Result<Vec<Value>, ExecError> {
    let tokens = tokenize(source).map_err(|m| ExecError::new("syntaxError", m))?;
    let mut groups: Vec<Vec<Value>> = vec![Vec::new()];
    for token in tokens {
        let item = match token {
            Token::Str(s) => Value::Str(s),
            Token::Word(w) if w == "[" => {
                groups.push(Vec::new());
                continue;
            }
            Token::Word(w) if w == "]" => {
                if groups.len() == 1 {
                    return Err(ExecError::new("syntaxError", "unmatched ]"));
                }
                Value::Vector(groups.pop().unwrap())
            }
            Token::Word(w) => match w.to_uppercase().as_str() {
                "TRUE" => Value::Bool(true),
                "FALSE" => Value::Bool(false),
                "NIL" => Value::Nil,
                upper => match Rational::parse(&w) {
                    Some(r) => Value::Number(r),
                    None => Value::Symbol(upper.to_string()),
                },
            },
        };
        groups.last_mut().unwrap().push(item);
    }
    if groups.len() > 1 {
        return Err(ExecError::new("syntaxError", "unclosed ["));
    }
    Ok(groups.pop().unwrap())
}

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub stack: Vec<Value>,
    pub words: BTreeMap<String, UserWord>,
    output: String,
    max_steps: u64,
    steps: u64,
    depth: usize,
    changes: Vec<String>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            stack: Vec::new(),
            words: BTreeMap::new(),
            output: String::new(),
            max_steps: DEFAULT_MAX_STEPS,
            steps: 0,
            depth: 0,
            changes: Vec::new(),
        }
    }

    pub fn max_steps(&self) -> u64 {
        self.max_steps
    }

    pub fn set_max_steps(&mut self, steps: u64) {
        self.max_steps = if steps == 0 { DEFAULT_MAX_STEPS } else { steps };
    }

    /// Run `source`. A failed run leaves the stack and dictionary as they were
    /// before it started; output printed before the failure is kept.
    pub fn execute(&mut self, source: &str) -> Result<(), ExecError> {
        self.steps = 0;
        self.depth = 0;
        self.changes.clear();
        let program = parse(source)?;
        let stack_before = self.stack.clone();
        let words_before = self.words.clone();
        let result = self.run(&program);
        if result.is_err() {
            self.stack = stack_before;
            self.words = words_before;
        }
        result
    }

    /// Text written by `PRINT` since the last call.
    pub fn take_output(&mut self) -> String {
        std::mem::take(&mut self.output)
    }

    /// Words defined or deleted by the most recent run.
    pub fn changes_this_run(&self) -> &[String] {
        &self.changes
    }

    pub fn reset(&mut self) {
        let max_steps = self.max_steps;
        *self = Interpreter::new();
        self.max_steps = max_steps;
    }

    /// Define a word from its definition text, as `restore_user_words` needs.
    pub fn define(
        &mut self,
        name: &str,
        definition: &str,
        description: Option<String>,
    ) -> Result<(), String> {
        let name = name.to_uppercase();
        if is_core(&name) {
            return Err("a Core word cannot be redefined".to_string());
        }
        let body = parse(definition).map_err(|e| e.message)?;
        self.words.insert(name, UserWord { body, description });
        Ok(())
    }

    fn run(&mut self, items: &[Value]) -> Result<(), ExecError> {
        for item in items {
            self.steps += 1;
            if self.steps > self.max_steps {
                return Err(ExecError::new("resourceLimit", "step limit exceeded"));
            }
            match item {
                Value::Symbol(name) => self.call(name)?,
                other => self.stack.push(other.clone()),
            }
        }
        Ok(())
    }

    fn pop(&mut self) -> Result<Value, ExecError> {
        self.stack
            .pop()
            .ok_or_else(|| ExecError::new("stackUnderflow", "stack underflow"))
    }

    fn pop_string(&mut self) -> Result<String, ExecError> {
        match self.pop()? {
            Value::Str(s) => Ok(s.to_uppercase()),
            other => Err(err(format!("expected a string, got {}", other.display()))),
        }
    }

    fn call(&mut self, name: &str) -> Result<(), ExecError> {
        match name {
            "ADD" => self.binary(Rational::checked_add),
            "SUB" => self.binary(Rational::checked_sub),
            "MUL" => self.binary(Rational::checked_mul),
            "DIV" => self.binary(Rational::checked_div),
            "EQ" => {
                let (b, a) = (self.pop()?, self.pop()?);
                self.stack.push(Value::Bool(a == b));
                Ok(())
            }
            "LT" => match (self.pop()?, self.pop()?) {
                (Value::Number(b), Value::Number(a)) => {
                    self.stack.push(Value::Bool(a.lt(b)));
                    Ok(())
                }
                _ => Err(err("LT compares two numbers")),
            },
            "DUP" => {
                let top = self.pop()?;
                self.stack.push(top.clone());
                self.stack.push(top);
                Ok(())
            }
            "DROP" => self.pop().map(|_| ()),
            "SWAP" => {
                let (b, a) = (self.pop()?, self.pop()?);
                self.stack.push(b);
                self.stack.push(a);
                Ok(())
            }
            "PRINT" => {
                let top = self.pop()?;
                let text = match top {
                    Value::Str(s) => s,
                    other => other.display(),
                };
                self.output.push_str(&text);
                self.output.push('\n');
                Ok(())
            }
            "MAP" => {
                let (block, target) = (self.pop()?, self.pop()?);
                let (Value::Vector(block), Value::Vector(items)) = (block, target) else {
                    return Err(err("MAP takes a vector and a block"));
                };
                let base = self.stack.len();
                let mut results = Vec::with_capacity(items.len());
                for item in items {
                    self.stack.push(item);
                    self.run(&block)?;
                    if self.stack.len() != base + 1 {
                        return Err(err("a MAP block must leave exactly one value"));
                    }
                    results.push(self.pop()?);
                }
                self.stack.push(Value::Vector(results));
                Ok(())
            }
            "DEF" => {
                let name = self.pop_string()?;
                let Value::Vector(body) = self.pop()? else {
                    return Err(err("DEF takes a block and a name"));
                };
                if is_core(&name) || Rational::parse(&name).is_some() || name.is_empty() {
                    return Err(ExecError::new(
                        "definitionConflict",
                        format!("{name} cannot be defined"),
                    ));
                }
                self.words.insert(
                    name.clone(),
                    UserWord {
                        body,
                        description: None,
                    },
                );
                self.output.push_str(&format!("Defined word: {name}\n"));
                self.changes.push(name);
                Ok(())
            }
            "DEL" => {
                let name = self.pop_string()?;
                if !self.words.contains_key(&name) {
                    return Err(err(format!("{name} is not a User word")));
                }
                if let Some((caller, _)) = self
                    .words
                    .iter()
                    .find(|(n, w)| **n != name && w.calls(&name))
                {
                    return Err(ExecError::new(
                        "definitionConflict",
                        format!("{name} is still used by {caller}"),
                    ));
                }
                self.words.remove(&name);
                self.output.push_str(&format!("Deleted word: {name}\n"));
                self.changes.push(name);
                Ok(())
            }
            _ => match self.words.get(name) {
                Some(word) => {
                    if self.depth >= MAX_CALL_DEPTH {
                        return Err(ExecError::new("resourceLimit", "call depth limit exceeded"));
                    }
                    let body = word.body.clone();
                    self.depth += 1;
                    let result = self.run(&body);
                    self.depth -= 1;
                    result
                }
                None => Err(ExecError::new(
                    "unknownWord",
                    format!("unknown word: {name}"),
                )),
            },
        }
    }

    /// Arithmetic on two numbers, or element by element when either side is
    /// a vector (a number pairs with every element).
    fn binary(
        &mut self,
        op: fn(Rational, Rational) -> Result<Rational, String>,
    ) -> Result<(), ExecError> {
        let (b, a) = (self.pop()?, self.pop()?);
        let result = lift(&a, &b, op).map_err(err)?;
        self.stack.push(result);
        Ok(())
    }
}

fn lift(
    a: &Value,
    b: &Value,
    op: fn(Rational, Rational) -> Result<Rational, String>,
) -> Result<Value, String> {
    match (a, b) {
        (Value::Number(x), Value::Number(y)) => op(*x, *y).map(Value::Number),
        (Value::Vector(xs), Value::Vector(ys)) => {
            if xs.len() != ys.len() {
                return Err("vectors differ in length".to_string());
            }
            xs.iter()
                .zip(ys)
                .map(|(x, y)| lift(x, y, op))
                .collect::<Result<_, _>>()
                .map(Value::Vector)
        }
        (Value::Vector(xs), y) => xs
            .iter()
            .map(|x| lift(x, y, op))
            .collect::<Result<_, _>>()
            .map(Value::Vector),
        (x, Value::Vector(ys)) => ys
            .iter()
            .map(|y| lift(x, y, op))
            .collect::<Result<_, _>>()
            .map(Value::Vector),
        _ => Err(format!(
            "cannot do arithmetic on {} and {}",
            a.display(),
            b.display()
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(source: &str) -> Interpreter {
        let mut interp = Interpreter::new();
        interp.execute(source).unwrap();
        interp
    }

    fn top(source: &str) -> String {
        run(source).stack.last().unwrap().display()
    }

    #[test]
    fn arithmetic_is_exact_and_lifts_over_vectors() {
        assert_eq!(top("1 3 DIV 1 6 DIV ADD"), "1/2");
        assert_eq!(top("[ 1 2 3 ] 10 MUL"), "[ 10/1 20/1 30/1 ]");
        assert_eq!(top("[ 1 2 ] [ 3 4 ] ADD"), "[ 4/1 6/1 ]");
    }

    #[test]
    fn user_words_and_map() {
        let interp = run("[ DUP MUL ] 'SQUARE' DEF [ 1 2 3 ] [ SQUARE ] MAP");
        assert_eq!(interp.stack.last().unwrap().display(), "[ 1/1 4/1 9/1 ]");
        assert_eq!(interp.words["SQUARE"].definition(), "DUP MUL");
    }

    #[test]
    fn a_failed_run_changes_nothing() {
        let mut interp = run("1");
        let error = interp.execute("2 [ X ] 'A' DEF 0 DIV").unwrap_err();
        assert_eq!(error.message, "division by zero");
        assert_eq!(interp.stack, vec![Value::Number(Rational::integer(1))]);
        assert!(interp.words.is_empty());
        assert_eq!(interp.changes_this_run(), ["A"]);
    }

    #[test]
    fn del_refuses_a_word_still_in_use() {
        let mut interp = run("[ 1 ] 'A' DEF [ A ] 'B' DEF");
        assert_eq!(
            interp.execute("'A' DEL").unwrap_err().category,
            "definitionConflict"
        );
        interp.execute("'B' DEL 'A' DEL").unwrap();
        assert!(interp.words.is_empty());
    }

    #[test]
    fn step_limit_stops_runaway_recursion() {
        let mut interp = run("[ LOOP ] 'LOOP' DEF");
        assert_eq!(
            interp.execute("LOOP").unwrap_err().message,
            "call depth limit exceeded"
        );
        interp.set_max_steps(100);
        assert_eq!(interp.execute("[ 1 ] [ DUP DROP ] MAP 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1").unwrap_err().message, "step limit exceeded");
    }
}
