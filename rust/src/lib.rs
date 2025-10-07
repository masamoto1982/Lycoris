use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use num_bigint::BigInt;
use num_traits::{Zero, One, Signed};
use std::str::FromStr;
use num_integer::Integer;
use regex::Regex;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
/**
* Value types used in Lycoris
*/
export interface Value {
  type: 'number' | 'string' | 'boolean' | 'vector' | 'symbol' | 'nil';
  value: any;
}
"#;

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
        if d.is_zero() { return (n, d); }
        let common = n.gcd(&d);
        (n / common, d / common)
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
        let values = self.parse(&tokens);

        if let Err(e) = self.eval_tokens(&values) {
            return Ok(serde_wasm_bindgen::to_value(&serde_json::json!({
                "status": "ERROR", "message": e, "error": true
            })).unwrap());
        }

        Ok(serde_wasm_bindgen::to_value(&serde_json::json!({
            "status": "OK", "output": self.output.clone(),
        })).unwrap())
    }

    fn tokenize(&self, code: &str) -> Vec<String> {
        let code_without_comments = code.split('#').next().unwrap_or("").trim();
        let re = Regex::new(r#"'[^']*'|\[|\]|\S+"#).unwrap();
        re.find_iter(code_without_comments).map(|mat| mat.as_str().to_string()).collect()
    }

    fn parse(&self, tokens: &[String]) -> Vec<Value> {
        // This function now only parses tokens into non-vector Values
        tokens.iter().map(|token| {
            match token.as_str() {
                s if self.is_number(s) => {
                    let (n, d) = self.parse_number(s);
                    Value::num(n, d)
                }
                s if s.starts_with('\'') && s.ends_with('\'') => {
                    Value::String(s[1..s.len() - 1].to_string())
                }
                "TRUE" => Value::Boolean(true),
                "FALSE" => Value::Boolean(false),
                "NIL" => Value::Nil,
                _ => Value::Symbol(token.to_string()),
            }
        }).collect()
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
            let denom = BigInt::from(10).pow(frac_part_str.len() as u32);
            let mut numer = int_part * &denom + frac_part;
             if s.starts_with('-') { numer = -numer.abs(); }
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
                        if balance != 0 { return Err("Mismatched brackets".to_string()); }
                        self.stack.push(Value::Vector(values[start..end].to_vec()));
                        i = end;
                    }
                    _ => self.eval_value(value)?,
                }
            } else {
                 self.stack.push(Value::Vector(vec![value.clone()]));
            }
            i += 1;
        }
        Ok(())
    }

    fn eval_value(&mut self, value: &Value) -> Result<(), String> {
        if let Value::Symbol(s) = value {
            if let Some(def) = self.dictionary.get(&s.to_uppercase()) {
                return self.eval_tokens(&def.clone());
            }

            match s.as_str() {
                "@" | "EVAL" => self.op_eval(),
                "DUP" => self.op_dup(), "DROP" => self.op_drop(), "SWAP" => self.op_swap(),
                "+" | "-" | "*" | "/" => self.op_arithmetic(s),
                "=" | "<" | ">" | "<=" | ">=" => self.op_comparison(s),
                "AND" | "OR" | "NOT" => self.op_logic(s),
                "PRINT" => self.op_print(), "DEF" => self.op_def(), "?" => self.op_lookup(),
                ":" => self.op_if(), "MAP" => self.op_map(), "RESET" => self.op_reset(),
                _ => self.stack.push(Value::Vector(vec![value.clone()])),
            }?;
        } else {
             self.stack.push(Value::Vector(vec![value.clone()]));
        }
        Ok(())
    }

    // Helper to extract the first element from a vector on the stack
    fn pop_first_value(&mut self) -> Result<Value, String> {
        let vec_val = self.stack.pop().ok_or("Stack underflow")?;
        if let Value::Vector(mut v) = vec_val {
            if v.is_empty() { Ok(Value::Nil) } else { Ok(v.remove(0)) }
        } else {
            Err("Expected a vector on the stack".to_string())
        }
    }

    fn op_arithmetic(&mut self, op: &str) -> Result<(), String> {
        let b = self.pop_first_value()?;
        let a = self.pop_first_value()?;
        let (an, ad) = self.extract_number(&a)?;
        let (bn, bd) = self.extract_number(&b)?;
        let (rn, rd) = match op {
            "+" => (an * &bd + bn * &ad, ad * bd),
            "-" => (an * &bd - bn * &ad, ad * bd),
            "*" => (an * bn, ad * bd),
            "/" => {
                if bn.is_zero() { return Err("Division by zero".into()); }
                (an * bd, ad * bn)
            }
            _ => return Err("Unknown operator".into())
        };
        self.stack.push(Value::Vector(vec![Value::num(rn, rd)]));
        Ok(())
    }

    fn op_eval(&mut self) -> Result<(), String> {
        let val = self.stack.pop().ok_or("Stack underflow for EVAL")?;
        if let Value::Vector(v) = val { self.eval_tokens(&v) } 
        else { Err("EVAL requires a vector".to_string()) }
    }

    fn op_dup(&mut self) -> Result<(), String> {
        let val = self.stack.last().ok_or("Stack underflow for DUP")?.clone();
        self.stack.push(val); Ok(())
    }

    fn op_drop(&mut self) -> Result<(), String> {
        self.stack.pop().ok_or("Stack underflow for DROP")?; Ok(())
    }

    fn op_swap(&mut self) -> Result<(), String> {
        if self.stack.len() < 2 { return Err("Stack underflow for SWAP".to_string()); }
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(a); self.stack.push(b); Ok(())
    }
    
    fn op_comparison(&mut self, op: &str) -> Result<(), String> {
        let b = self.pop_first_value()?;
        let a = self.pop_first_value()?;
        let result = match (a, b) {
            (Value::Number{numerator: an, ..}, Value::Number{numerator: bn, ..}) => {
                let (a_val, b_val) = self.extract_number(&a)?;
                let (b_val_n, b_val_d) = self.extract_number(&b)?;
                match op {
                    "=" => a_val.0 * &b_val_d == b_val_n * &a_val.1,
                    "<" => (a_val.0 * &b_val_d) < (b_val_n * &a_val.1),
                    ">" => (a_val.0 * &b_val_d) > (b_val_n * &a_val.1),
                    "<=" => (a_val.0 * &b_val_d) <= (b_val_n * &a_val.1),
                    ">=" => (a_val.0 * &b_val_d) >= (b_val_n * &a_val.1),
                    _ => false
                }
            },
            (Value::String(sa), Value::String(sb)) => sa == sb,
            _ => return Err("Comparison requires two numbers or two strings".to_string())
        };
        self.stack.push(Value::Vector(vec![Value::Boolean(result)])); Ok(())
    }

    fn op_logic(&mut self, op: &str) -> Result<(), String> {
        match op {
            "NOT" => {
                let val = self.pop_first_value()?;
                self.stack.push(Value::Vector(vec![Value::Boolean(!self.is_truthy(&val))]));
            }
            "AND" | "OR" => {
                let b = self.pop_first_value()?;
                let a = self.pop_first_value()?;
                let result = if op == "AND" { self.is_truthy(&a) && self.is_truthy(&b) } else { self.is_truthy(&a) || self.is_truthy(&b) };
                self.stack.push(Value::Vector(vec![Value::Boolean(result)]));
            }
            _ => {}
        }
        Ok(())
    }

    fn op_print(&mut self) -> Result<(), String> {
        let val = self.stack.pop().ok_or("Stack underflow for PRINT")?;
        self.output.push_str(&self.value_to_string(&val)); self.output.push(' '); Ok(())
    }

    fn op_def(&mut self) -> Result<(), String> {
        let name_val = self.pop_first_value()?;
        let body_val = self.stack.pop().ok_or("Stack underflow for DEF body")?;
        if let (Value::String(name), Value::Vector(body)) = (name_val, body_val) {
            self.dictionary.insert(name.to_uppercase(), body); Ok(())
        } else {
            Err("DEF requires a string name and a vector body".to_string())
        }
    }
    
    fn op_lookup(&mut self) -> Result<(), String> {
        let name = self.pop_first_value()?;
        if let Value::String(n) = name {
            let body = self.dictionary.get(&n.to_uppercase()).cloned().unwrap_or_default();
            self.stack.push(Value::Vector(body)); Ok(())
        } else { Err("? requires a string name".to_string()) }
    }
    
    fn op_if(&mut self) -> Result<(), String> {
        let else_branch = self.stack.pop().ok_or("Stack underflow for : (else branch)")?;
        let then_branch = self.stack.pop().ok_or("Stack underflow for : (then branch)")?;
        let cond = self.pop_first_value()?;
        let branch_to_eval = if self.is_truthy(&cond) { then_branch } else { else_branch };
        if let Value::Vector(v) = branch_to_eval { self.eval_tokens(&v) } 
        else { Err("Branches for : must be vectors".to_string()) }
    }

    fn op_map(&mut self) -> Result<(), String> {
        let func = self.stack.pop().ok_or("Stack underflow for MAP (function)")?;
        let data = self.stack.pop().ok_or("Stack underflow for MAP (data)")?;
        if let (Value::Vector(f_vec), Value::Vector(d_vec)) = (func, data) {
            let mut results = Vec::new();
            for item in d_vec {
                self.stack.push(Value::Vector(vec![item]));
                self.eval_tokens(&f_vec)?;
                results.push(self.stack.pop().unwrap_or(Value::Vector(vec![Value::Nil])));
            }
            self.stack.push(Value::Vector(results)); Ok(())
        } else { Err("MAP requires two vectors".to_string()) }
    }

    fn op_reset(&mut self) -> Result<(), String> {
        self.stack.clear(); self.dictionary = default_dictionary(); self.output = "System reset.".into(); Ok(())
    }

    fn extract_number(&self, val: &Value) -> Result<(BigInt, BigInt), String> {
        if let Value::Number { numerator, denominator } = val {
            Ok((numerator.clone(), denominator.clone()))
        } else { Err(format!("Expected a number, but got {}", self.value_to_string_inner(val))) }
    }
    
    fn is_truthy(&self, val: &Value) -> bool {
        match val {
            Value::Boolean(b) => *b,
            Value::Number { numerator, .. } => !numerator.is_zero(),
            Value::Nil => false,
            _ => true,
        }
    }

    fn value_to_string(&self, val: &Value) -> String {
        match val {
            Value::Vector(v) => {
                let items: Vec<String> = v.iter().map(|item| self.value_to_string_inner(item)).collect();
                if v.len() == 1 { items.join(" ") } else { format!("[ {} ]", items.join(" ")) }
            }
            _ => self.value_to_string_inner(val),
        }
    }

    fn value_to_string_inner(&self, val: &Value) -> String {
        match val {
            Value::Number { numerator, denominator } => {
                if denominator == &One::one() { numerator.to_string() } else { format!("{}/{}", numerator, denominator) }
            }
            Value::String(s) => format!("'{}'", s),
            Value::Boolean(b) => if *b { "TRUE".to_string() } else { "FALSE".to_string() },
            Value::Vector(v) => {
                let items: Vec<String> = v.iter().map(|item| self.value_to_string_inner(item)).collect();
                format!("[ {} ]", items.join(" "))
            },
            Value::Symbol(s) => s.clone(),
            Value::Nil => "NIL".to_string(),
        }
    }
    
    #[wasm_bindgen]
    pub fn get_stack(&self) -> JsValue { serde_wasm_bindgen::to_value(&self.stack).unwrap() }

    #[wasm_bindgen]
    pub fn get_custom_words_info(&self) -> JsValue {
        let words: Vec<Vec<String>> = self.dictionary.iter()
            .map(|(k, v)| vec![k.clone(), self.value_to_string_inner(&Value::Vector(v.clone()))])
            .collect();
        serde_wasm_bindgen::to_value(&words).unwrap()
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) -> JsValue {
        self.op_reset().unwrap();
        serde_wasm_bindgen::to_value(&serde_json::json!({ "status": "OK", "output": self.output.clone() })).unwrap()
    }
}
