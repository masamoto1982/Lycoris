use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use num_bigint::BigInt;
use num_traits::{Zero, One, ToPrimitive, Signed};
use std::str::FromStr;
use std::mem;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "camelCase")]
pub enum Value {
    Number { numerator: BigInt, denominator: BigInt },
    String(String),
    Boolean(bool),
    Vector(Vec<Value>),
    Symbol(String),
    Nil,
}

impl Value {
    fn simplify_fraction(n: BigInt, d: BigInt) -> (BigInt, BigInt) {
        if d.is_zero() {
            return (n, d); // Avoid division by zero
        }
        let common = n.gcd(&d);
        (n / &common, d / &common)
    }

    fn num(n: BigInt, d: BigInt) -> Self {
        let (num, den) = Self::simplify_fraction(n, d);
        Value::Number { numerator: num, denominator: den }
    }
}

#[wasm_bindgen]
pub struct LycorisInterpreter {
    stack: Vec<Value>,
    dictionary: HashMap<String, Vec<Value>>,
    output: String,
}

fn default_dictionary() -> HashMap<String, Vec<Value>> {
    let mut dict = HashMap::new();
    dict.insert("EVAL".to_string(), vec![Value::Symbol("@".to_string())]);
    dict.insert("APPLY".to_string(), vec![Value::Symbol("!".to_string())]);
    dict
}

#[wasm_bindgen]
impl LycorisInterpreter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            dictionary: default_dictionary(),
            output: String::new(),
        }
    }

    #[wasm_bindgen]
    pub fn execute(&mut self, code: &str) -> Result<JsValue, JsValue> {
        self.output.clear();
        let tokens = self.tokenize(code);
        let parsed_tokens = self.parse(&tokens);

        if let Err(e) = self.eval_tokens(&parsed_tokens) {
            return Ok(serde_wasm_bindgen::to_value(&serde_json::json!({
                "status": "ERROR",
                "message": e,
                "error": true
            })).unwrap());
        }

        Ok(serde_wasm_bindgen::to_value(&serde_json::json!({
            "status": "OK",
            "output": self.output.clone(),
        })).unwrap())
    }

    fn tokenize(&self, code: &str) -> Vec<String> {
        // Comments are handled by stopping at '#'
        let code_without_comments = code.split('#').next().unwrap_or("").trim();
        // Use regex to handle strings and other tokens
        let re = regex::Regex::new(r#"'[^']*'|\[|\]|\S+"#).unwrap();
        re.find_iter(code_without_comments)
          .map(|mat| mat.as_str().to_string())
          .collect()
    }

    fn parse(&self, tokens: &[String]) -> Vec<Value> {
        let mut values = Vec::new();
        for token in tokens {
            match token.as_str() {
                // Numbers
                s if self.is_number(s) => {
                    let (n, d) = self.parse_number(s);
                    values.push(Value::num(n,d));
                }
                // Strings
                s if s.starts_with('\'') && s.ends_with('\'') => {
                    values.push(Value::String(s[1..s.len() - 1].to_string()));
                }
                // Booleans & Nil
                "TRUE" => values.push(Value::Boolean(true)),
                "FALSE" => values.push(Value::Boolean(false)),
                "NIL" => values.push(Value.Nil),
                // Vectors or Symbols
                _ => values.push(Value::Symbol(token.to_string())),
            }
        }
        values
    }

    fn is_number(&self, s: &str) -> bool {
        s.parse::<f64>().is_ok() || (s.contains('/') && s.split('/').count() == 2)
    }

    fn parse_number(&self, s: &str) -> (BigInt, BigInt) {
        if s.contains('/') {
            let parts: Vec<_> = s.split('/').collect();
            let n = BigInt::from_str(parts[0]).unwrap_or_else(|_| Zero::zero());
            let d = BigInt::from_str(parts[1]).unwrap_or_else(|_| One::one());
            if d.is_zero() { (n, d) } else { (n, d) }
        } else if s.contains('.') {
            let mut parts = s.split('.');
            let int_part_str = parts.next().unwrap_or("0");
            let frac_part_str = parts.next().unwrap_or("0");
            let int_part = BigInt::from_str(int_part_str).unwrap_or_else(|_| Zero::zero());
            let frac_part = BigInt::from_str(frac_part_str).unwrap_or_else(|_| Zero::zero());
            let mut denom = BigInt::from(10).pow(frac_part_str.len() as u32);
            let mut numer = int_part * &denom + frac_part;
             if s.starts_with('-') {
                numer = -numer.abs();
             }
            (numer, denom)
        } else {
            (BigInt::from_str(s).unwrap_or_else(|_| Zero::zero()), One::one())
        }
    }


    fn eval_tokens(&mut self, values: &[Value]) -> Result<(), String> {
        let mut i = 0;
        while i < values.len() {
            let value = &values[i];
            if let Value::Symbol(s) = value {
                match s.as_str() {
                    "[" => {
                        let start = i + 1;
                        let mut balance = 1;
                        let mut end = start;
                        while end < values.len() {
                            if let Value::Symbol(t) = &values[end] {
                                if t == "[" { balance += 1; }
                                if t == "]" { balance -= 1; }
                            }
                            if balance == 0 { break; }
                            end += 1;
                        }
                        if balance != 0 {
                            return Err("Mismatched brackets".to_string());
                        }
                        self.stack.push(Value::Vector(values[start..end].to_vec()));
                        i = end;
                    }
                    _ => self.eval_value(value)?,
                }
            } else {
                 self.stack.push(value.clone());
            }
            i += 1;
        }
        Ok(())
    }


    fn eval_value(&mut self, value: &Value) -> Result<(), String> {
        if let Value::Symbol(s) = value {
            if let Some(def) = self.dictionary.get(&s.to_uppercase()) {
                let tokens_to_eval = def.clone();
                return self.eval_tokens(&tokens_to_eval);
            }

            match s.as_str() {
                // Meta
                "@" | "EVAL" => {
                    let val = self.stack.pop().ok_or("Stack underflow for EVAL")?;
                    if let Value::Vector(v) = val {
                        self.eval_tokens(&v)?;
                    } else if let Value::String(s) = val {
                         let tokens = self.tokenize(&s);
                         let parsed = self.parse(&tokens);
                         self.eval_tokens(&parsed)?;
                    } else {
                        return Err("EVAL requires a vector or string".to_string());
                    }
                }
                // Stack
                "DUP" => {
                    let val = self.stack.last().ok_or("Stack underflow for DUP")?.clone();
                    self.stack.push(val);
                }
                "DROP" => {
                    self.stack.pop().ok_or("Stack underflow for DROP")?;
                }
                "SWAP" => {
                    if self.stack.len() < 2 { return Err("Stack underflow for SWAP".to_string()); }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a);
                    self.stack.push(b);
                }
                // Arithmetic
                "+" | "-" | "*" | "/" => {
                    if self.stack.len() < 2 { return Err(format!("Stack underflow for {}", s)); }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(self.do_arithmetic(a, b, s)?);
                }
                // Comparison
                "=" | "<" | ">" | "<=" | ">=" => {
                     if self.stack.len() < 2 { return Err(format!("Stack underflow for {}", s)); }
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(self.do_comparison(a, b, s)?);
                }
                 // Logic
                "AND" | "OR" | "NOT" => self.do_logic(s)?,
                // I/O
                "PRINT" => {
                    let val = self.stack.pop().ok_or("Stack underflow for PRINT")?;
                    self.output.push_str(&self.value_to_string(&val));
                    self.output.push(' ');
                }
                // Dictionary
                "DEF" => {
                    let name_val = self.stack.pop().ok_or("Stack underflow for DEF (name)")?;
                    let body_val = self.stack.pop().ok_or("Stack underflow for DEF (body)")?;
                    if let Value::String(name) = name_val {
                       if let Value::Vector(body) = body_val {
                           self.dictionary.insert(name.to_uppercase(), body);
                       } else {
                           return Err("DEF body must be a vector".to_string());
                       }
                    } else {
                       return Err("DEF name must be a string".to_string());
                    }
                }
                "?" => {
                    let name_val = self.stack.pop().ok_or("Stack underflow for ?")?;
                     if let Value::String(name) = name_val {
                         if let Some(def) = self.dictionary.get(&name.to_uppercase()) {
                             self.stack.push(Value::Vector(def.clone()));
                         } else {
                             self.stack.push(Value::Nil);
                         }
                     } else {
                        return Err("? requires a string name".to_string());
                     }
                }
                // Control Flow
                ":" => { // IF
                    let else_branch = self.stack.pop().ok_or("Stack underflow for : (else branch)")?;
                    let then_branch = self.stack.pop().ok_or("Stack underflow for : (then branch)")?;
                    let cond = self.stack.pop().ok_or("Stack underflow for : (condition)")?;
                    
                    let branch_to_eval = if self.is_truthy(&cond) { then_branch } else { else_branch };

                    if let Value::Vector(v) = branch_to_eval {
                        self.eval_tokens(&v)?;
                    } else {
                        return Err("Branches for : must be vectors".to_string());
                    }
                }
                 // Higher-order functions
                "MAP" => {
                    let func = self.stack.pop().ok_or("Stack underflow for MAP (function)")?;
                    let vec = self.stack.pop().ok_or("Stack underflow for MAP (vector)")?;

                    if let (Value::Vector(f_vec), Value::Vector(v_vec)) = (func, vec) {
                        let mut results = Vec::new();
                        for item in v_vec {
                            self.stack.push(item);
                            self.eval_tokens(&f_vec)?;
                            results.push(self.stack.pop().ok_or("Function in MAP did not produce a result")?);
                        }
                        self.stack.push(Value::Vector(results));
                    } else {
                        return Err("MAP requires a function vector and a data vector".to_string());
                    }
                }
                // System
                "RESET" => {
                    self.stack.clear();
                    self.dictionary = default_dictionary();
                    self.output = "System reset.".into();
                }
                _ => self.stack.push(value.clone()), // Push unknown symbols to stack
            }
        } else {
             self.stack.push(value.clone());
        }
        Ok(())
    }

    fn do_arithmetic(&self, a: Value, b: Value, op: &str) -> Result<Value, String> {
        let (an, ad) = self.extract_number(&a)?;
        let (bn, bd) = self.extract_number(&b)?;

        let (rn, rd) = match op {
            "+" => (an * &bd + bn * &ad, ad.clone() * bd.clone()),
            "-" => (an * &bd - bn * &ad, ad.clone() * bd.clone()),
            "*" => (an * bn, ad.clone() * bd.clone()),
            "/" => {
                if bn.is_zero() { return Err("Division by zero".into()); }
                (an * bd, ad.clone() * bn)
            }
            _ => return Err("Unknown operator".into())
        };
        Ok(Value::num(rn, rd))
    }

    fn do_comparison(&self, a: Value, b: Value, op: &str) -> Result<Value, String> {
        if let (Value::Number{numerator: an, denominator: ad}, Value::Number{numerator: bn, denominator: bd}) = (a, b) {
            let result = match op {
                "=" => an * &bd == bn * &ad,
                "<" => (an * &bd) < (bn * &ad),
                ">" => (an * &bd) > (bn * &ad),
                "<=" => (an * &bd) <= (bn * &ad),
                ">=" => (an * &bd) >= (bn * &ad),
                _ => false
            };
            Ok(Value::Boolean(result))
        } else if let (Value::String(sa), Value::String(sb)) = (a,b) {
             Ok(Value::Boolean(sa == sb))
        } else {
             Err("Comparison requires two numbers or two strings".to_string())
        }
    }

    fn do_logic(&mut self, op: &str) -> Result<(), String> {
        match op {
            "NOT" => {
                let val = self.stack.pop().ok_or("Stack underflow for NOT")?;
                self.stack.push(Value::Boolean(!self.is_truthy(&val)));
            }
            "AND" | "OR" => {
                if self.stack.len() < 2 { return Err(format!("Stack underflow for {}", op)); }
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                let result = if op == "AND" { self.is_truthy(&a) && self.is_truthy(&b) } else { self.is_truthy(&a) || self.is_truthy(&b) };
                self.stack.push(Value::Boolean(result));
            }
            _ => {}
        }
        Ok(())
    }

    fn extract_number(&self, val: &Value) -> Result<(BigInt, BigInt), String> {
        if let Value::Number { numerator, denominator } = val {
            Ok((numerator.clone(), denominator.clone()))
        } else {
            Err(format!("Expected a number, but got {}", self.value_to_string(val)))
        }
    }
    
    fn is_truthy(&self, val: &Value) -> bool {
        match val {
            Value::Boolean(b) => *b,
            Value::Number { numerator, .. } => !numerator.is_zero(),
            Value::Nil => false,
            _ => true, // All other types are truthy
        }
    }

    fn value_to_string(&self, val: &Value) -> String {
        match val {
            Value::Number { numerator, denominator } => {
                if denominator == &One::one() {
                    numerator.to_string()
                } else {
                    format!("{}/{}", numerator, denominator)
                }
            }
            Value::String(s) => format!("'{}'", s),
            Value::Boolean(b) => if *b { "TRUE".to_string() } else { "FALSE".to_string() },
            Value::Vector(v) => {
                let items: Vec<String> = v.iter().map(|item| self.value_to_string(item)).collect();
                format!("[ {} ]", items.join(" "))
            },
            Value::Symbol(s) => s.clone(),
            Value::Nil => "NIL".to_string(),
        }
    }


    #[wasm_bindgen]
    pub fn get_stack(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.stack).unwrap()
    }

    #[wasm_bindgen]
    pub fn get_custom_words_info(&self) -> JsValue {
        let words: Vec<Vec<String>> = self.dictionary.iter()
            .map(|(k, v)| vec![
                k.clone(),
                self.value_to_string(&Value::Vector(v.clone()))
            ])
            .collect();
        serde_wasm_bindgen::to_value(&words).unwrap()
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) -> JsValue {
        self.stack.clear();
        self.dictionary = default_dictionary();
        self.output = "System reset.".into();

        serde_wasm_bindgen::to_value(&serde_json::json!({
            "status": "OK",
            "output": self.output.clone()
        })).unwrap()
    }
}

// Need to add regex dependency in Cargo.toml
// [dependencies]
// regex = "1"
