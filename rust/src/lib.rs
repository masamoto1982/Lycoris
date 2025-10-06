use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use num_bigint::BigInt;
use num_traits::{Zero, One, ToPrimitive};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Value {
    #[serde(rename = "type")]
    val_type: String,
    value: serde_json::Value,
}

impl Value {
    fn num(n: BigInt, d: BigInt) -> Self {
        Value {
            val_type: "number".into(),
            value: serde_json::json!({
                "numerator": n.to_string(),
                "denominator": d.to_string()
            })
        }
    }
    
    fn vec(v: Vec<Value>) -> Self {
        Value {
            val_type: "vector".into(),
            value: serde_json::to_value(v).unwrap()
        }
    }
    
    fn string(s: String) -> Self {
        Value {
            val_type: "string".into(),
            value: serde_json::json!(s)
        }
    }
    
    fn boolean(b: bool) -> Self {
        Value {
            val_type: "boolean".into(),
            value: serde_json::json!(b)
        }
    }
    
    fn nil() -> Self {
        Value {
            val_type: "nil".into(),
            value: serde_json::Value::Null
        }
    }
}

#[wasm_bindgen]
pub struct LycorisInterpreter {
    stack: Vec<Value>,
    dictionary: HashMap<String, String>,
    output: String,
}

#[wasm_bindgen]
impl LycorisInterpreter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut dict = HashMap::new();
        // Built-in meta operations
        dict.insert("@".into(), "EVAL".into());
        dict.insert("!".into(), "APPLY".into());
        
        Self {
            stack: Vec::new(),
            dictionary: dict,
            output: String::new(),
        }
    }

    #[wasm_bindgen]
    pub async fn execute(&mut self, code: &str) -> Result<JsValue, JsValue> {
        self.output.clear();
        
        let tokens = self.tokenize(code);
        for token in tokens {
            if let Err(e) = self.eval_token(&token) {
                return Ok(serde_wasm_bindgen::to_value(&serde_json::json!({
                    "status": "ERROR",
                    "message": e,
                    "error": true
                })).unwrap());
            }
        }
        
        Ok(serde_wasm_bindgen::to_value(&serde_json::json!({
            "status": "OK",
            "output": self.output.clone(),
            "stack": self.stack.clone()
        })).unwrap())
    }

    fn tokenize(&self, code: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut in_string = false;
        let mut bracket_depth = 0;
        
        for ch in code.chars() {
            match ch {
                '\'' if !in_string => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                    in_string = true;
                    current.push(ch);
                }
                '\'' if in_string => {
                    current.push(ch);
                    tokens.push(current.clone());
                    current.clear();
                    in_string = false;
                }
                '[' | '{' | '(' if !in_string => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                    tokens.push(ch.to_string());
                    bracket_depth += 1;
                }
                ']' | '}' | ')' if !in_string => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                    tokens.push(ch.to_string());
                    bracket_depth -= 1;
                }
                ' ' | '\n' | '\t' if !in_string && bracket_depth == 0 => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                }
                '#' if !in_string => {
                    // Comment - skip to end of line
                    break;
                }
                _ => current.push(ch)
            }
        }
        
        if !current.is_empty() {
            tokens.push(current);
        }
        
        tokens
    }

    fn eval_token(&mut self, token: &str) -> Result<(), String> {
        match token {
            // Meta operations
            "EVAL" | "@" => {
                if let Some(val) = self.stack.pop() {
                    if val.val_type == "string" {
                        if let Some(s) = val.value.as_str() {
                            let tokens = self.tokenize(s);
                            for t in tokens {
                                self.eval_token(&t)?;
                            }
                        }
                    }
                }
            }
            // Stack operations
            "DUP" => {
                if let Some(val) = self.stack.last() {
                    self.stack.push(val.clone());
                }
            }
            "DROP" => {
                self.stack.pop();
            }
            "SWAP" => {
                if self.stack.len() >= 2 {
                    let len = self.stack.len();
                    self.stack.swap(len - 1, len - 2);
                }
            }
            // Arithmetic
            "+" | "-" | "*" | "/" => {
                if self.stack.len() >= 2 {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    // Simplified arithmetic
                    self.stack.push(self.do_arithmetic(a, b, token)?);
                }
            }
            // Comparison
            "=" | "<" | ">" | "<=" | ">=" => {
                if self.stack.len() >= 2 {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(self.do_comparison(a, b, token)?);
                }
            }
            // Logic
            "AND" | "OR" | "NOT" => {
                self.do_logic(token)?;
            }
            // Vectors
            "[" | "{" | "(" => {
                self.stack.push(Value::vec(Vec::new()));
            }
            "]" | "}" | ")" => {
                let mut items = Vec::new();
                while let Some(val) = self.stack.pop() {
                    if val.val_type == "vector" && val.value.as_array().unwrap().is_empty() {
                        break;
                    }
                    items.push(val);
                }
                items.reverse();
                self.stack.push(Value::vec(items));
            }
            // I/O
            "PRINT" => {
                if let Some(val) = self.stack.pop() {
                    self.output.push_str(&self.value_to_string(&val));
                    self.output.push(' ');
                }
            }
            // Dictionary
            "DEF" => {
                if self.stack.len() >= 2 {
                    let name = self.stack.pop().unwrap();
                    let body = self.stack.pop().unwrap();
                    if let Some(n) = self.extract_string(&name) {
                        self.dictionary.insert(n.to_uppercase(), self.value_to_string(&body));
                    }
                }
            }
            "DEL" => {
                if let Some(name) = self.stack.pop() {
                    if let Some(n) = self.extract_string(&name) {
                        self.dictionary.remove(&n.to_uppercase());
                    }
                }
            }
            // Control flow
            ":" | ";" => {
                // Gate operations
                if self.stack.len() >= 2 {
                    let action = self.stack.pop().unwrap();
                    let condition = self.stack.pop().unwrap();
                    
                    if self.is_truthy(&condition) {
                        let tokens = self.tokenize(&self.value_to_string(&action));
                        for t in tokens {
                            self.eval_token(&t)?;
                        }
                    } else {
                        self.stack.push(condition);
                    }
                }
            }
            // Higher-order functions
            "MAP" | "FILTER" | "REDUCE" | "EACH" => {
                self.do_higher_order(token)?;
            }
            // Constants
            "TRUE" => self.stack.push(Value::vec(vec![Value::boolean(true)])),
            "FALSE" => self.stack.push(Value::vec(vec![Value::boolean(false)])),
            "NIL" => self.stack.push(Value::vec(vec![Value::nil()])),
            // System
            "RESET" => {
                self.stack.clear();
                self.dictionary.clear();
                self.output = "System reset.".into();
            }
            // String literals
            s if s.starts_with('\'') && s.ends_with('\'') => {
                let string = s[1..s.len()-1].to_string();
                self.stack.push(Value::vec(vec![Value::string(string)]));
            }
            // Numbers
            s if s.parse::<f64>().is_ok() || s.contains('/') => {
                let (n, d) = self.parse_number(s);
                self.stack.push(Value::vec(vec![Value::num(n, d)]));
            }
            // User-defined words or fallback
            s => {
                if let Some(def) = self.dictionary.get(&s.to_uppercase()) {
                    let tokens = self.tokenize(&def.clone());
                    for t in tokens {
                        self.eval_token(&t)?;
                    }
                } else {
                    // Unknown token - push as string
                    self.stack.push(Value::string(s.to_string()));
                }
            }
        }
        Ok(())
    }

    fn parse_number(&self, s: &str) -> (BigInt, BigInt) {
        if s.contains('/') {
            let parts: Vec<_> = s.split('/').collect();
            (
                BigInt::from_str_radix(parts[0], 10).unwrap_or_else(|_| Zero::zero()),
                BigInt::from_str_radix(parts[1], 10).unwrap_or_else(|_| One::one())
            )
        } else if s.contains('.') {
            // Convert decimal to fraction
            let parts: Vec<_> = s.split('.').collect();
            let int_part = BigInt::from_str_radix(parts[0], 10).unwrap_or_else(|_| Zero::zero());
            let frac_part = BigInt::from_str_radix(parts[1], 10).unwrap_or_else(|_| Zero::zero());
            let denom = BigInt::from(10).pow(parts[1].len() as u32);
            (int_part * &denom + frac_part, denom)
        } else {
            (BigInt::from_str_radix(s, 10).unwrap_or_else(|_| Zero::zero()), One::one())
        }
    }

    fn do_arithmetic(&self, a: Value, b: Value, op: &str) -> Result<Value, String> {
        // Extract numbers from vectors
        let (an, ad) = self.extract_number(&a)?;
        let (bn, bd) = self.extract_number(&b)?;
        
        let (rn, rd) = match op {
            "+" => (an * &bd + bn * &ad, ad * bd),
            "-" => (an * &bd - bn * &ad, ad * bd),
            "*" => (an * bn, ad * bd),
            "/" => {
                if bn.is_zero() {
                    return Err("Division by zero".into());
                }
                (an * bd, ad * bn)
            }
            _ => return Err("Unknown operator".into())
        };
        
        Ok(Value::vec(vec![Value::num(rn, rd)]))
    }

    fn do_comparison(&self, a: Value, b: Value, op: &str) -> Result<Value, String> {
        let (an, ad) = self.extract_number(&a)?;
        let (bn, bd) = self.extract_number(&b)?;
        
        let result = match op {
            "=" => an * &bd == bn * &ad,
            "<" => (an * &bd) < (bn * &ad),
            ">" => (an * &bd) > (bn * &ad),
            "<=" => (an * &bd) <= (bn * &ad),
            ">=" => (an * &bd) >= (bn * &ad),
            _ => false
        };
        
        Ok(Value::vec(vec![Value::boolean(result)]))
    }

    fn do_logic(&mut self, op: &str) -> Result<(), String> {
        match op {
            "NOT" => {
                if let Some(val) = self.stack.pop() {
                    let b = self.extract_bool(&val)?;
                    self.stack.push(Value::vec(vec![Value::boolean(!b)]));
                }
            }
            "AND" => {
                if self.stack.len() >= 2 {
                    let b = self.extract_bool(&self.stack.pop().unwrap())?;
                    let a = self.extract_bool(&self.stack.pop().unwrap())?;
                    self.stack.push(Value::vec(vec![Value::boolean(a && b)]));
                }
            }
            "OR" => {
                if self.stack.len() >= 2 {
                    let b = self.extract_bool(&self.stack.pop().unwrap())?;
                    let a = self.extract_bool(&self.stack.pop().unwrap())?;
                    self.stack.push(Value::vec(vec![Value::boolean(a || b)]));
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn do_higher_order(&mut self, op: &str) -> Result<(), String> {
        if self.stack.len() < 2 {
            return Err("Not enough arguments".into());
        }
        
        let func = self.stack.pop().unwrap();
        let func_str = self.extract_string(&func).ok_or("Expected function name")?;
        
        match op {
            "MAP" => {
                let vec = self.stack.pop().unwrap();
                if let Some(items) = vec.value.as_array() {
                    let mut results = Vec::new();
                    for item in items {
                        let v: Value = serde_json::from_value(item.clone()).unwrap();
                        self.stack.push(Value::vec(vec![v]));
                        
                        let tokens = self.tokenize(&func_str);
                        for t in tokens {
                            self.eval_token(&t)?;
                        }
                        
                        if let Some(result) = self.stack.pop() {
                            // Extract from vector wrapper
                            if result.val_type == "vector" {
                                if let Some(arr) = result.value.as_array() {
                                    if !arr.is_empty() {
                                        results.push(serde_json::from_value(arr[0].clone()).unwrap());
                                    }
                                }
                            } else {
                                results.push(result);
                            }
                        }
                    }
                    self.stack.push(Value::vec(results));
                }
            }
            "FILTER" => {
                let vec = self.stack.pop().unwrap();
                if let Some(items) = vec.value.as_array() {
                    let mut results = Vec::new();
                    for item in items {
                        let v: Value = serde_json::from_value(item.clone()).unwrap();
                        self.stack.push(Value::vec(vec![v.clone()]));
                        
                        let tokens = self.tokenize(&func_str);
                        for t in tokens {
                            self.eval_token(&t)?;
                        }
                        
                        if let Some(result) = self.stack.pop() {
                            if self.is_truthy(&result) {
                                results.push(v);
                            }
                        }
                    }
                    self.stack.push(Value::vec(results));
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn extract_number(&self, val: &Value) -> Result<(BigInt, BigInt), String> {
        if val.val_type == "vector" {
            if let Some(arr) = val.value.as_array() {
                if !arr.is_empty() {
                    let first: Value = serde_json::from_value(arr[0].clone()).unwrap();
                    if first.val_type == "number" {
                        if let Some(obj) = first.value.as_object() {
                            let n = obj.get("numerator").and_then(|v| v.as_str())
                                .ok_or("Missing numerator")?;
                            let d = obj.get("denominator").and_then(|v| v.as_str())
                                .ok_or("Missing denominator")?;
                            return Ok((
                                BigInt::from_str_radix(n, 10).unwrap(),
                                BigInt::from_str_radix(d, 10).unwrap()
                            ));
                        }
                    }
                }
            }
        }
        Err("Not a number".into())
    }

    fn extract_bool(&self, val: &Value) -> Result<bool, String> {
        if val.val_type == "vector" {
            if let Some(arr) = val.value.as_array() {
                if !arr.is_empty() {
                    let first: Value = serde_json::from_value(arr[0].clone()).unwrap();
                    if first.val_type == "boolean" {
                        return Ok(first.value.as_bool().unwrap());
                    }
                }
            }
        }
        Err("Not a boolean".into())
    }

    fn extract_string(&self, val: &Value) -> Option<String> {
        if val.val_type == "vector" {
            if let Some(arr) = val.value.as_array() {
                if !arr.is_empty() {
                    let first: Value = serde_json::from_value(arr[0].clone()).ok()?;
                    if first.val_type == "string" {
                        return first.value.as_str().map(|s| s.to_string());
                    }
                }
            }
        }
        None
    }

    fn is_truthy(&self, val: &Value) -> bool {
        if let Ok(b) = self.extract_bool(val) {
            b
        } else if let Ok((n, _)) = self.extract_number(val) {
            !n.is_zero()
        } else {
            false
        }
    }

    fn value_to_string(&self, val: &Value) -> String {
        match val.val_type.as_str() {
            "number" => {
                if let Some(obj) = val.value.as_object() {
                    let n = obj.get("numerator").and_then(|v| v.as_str()).unwrap_or("0");
                    let d = obj.get("denominator").and_then(|v| v.as_str()).unwrap_or("1");
                    if d == "1" {
                        n.to_string()
                    } else {
                        format!("{}/{}", n, d)
                    }
                } else {
                    "?".to_string()
                }
            }
            "string" => val.value.as_str().unwrap_or("").to_string(),
            "boolean" => if val.value.as_bool().unwrap_or(false) { "TRUE" } else { "FALSE" }.to_string(),
            "vector" => {
                if let Some(arr) = val.value.as_array() {
                    let items: Vec<String> = arr.iter()
                        .map(|v| {
                            let val: Value = serde_json::from_value(v.clone()).unwrap();
                            self.value_to_string(&val)
                        })
                        .collect();
                    format!("[{}]", items.join(" "))
                } else {
                    "[]".to_string()
                }
            }
            "nil" => "NIL".to_string(),
            _ => "?".to_string()
        }
    }

    #[wasm_bindgen]
    pub fn get_stack(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.stack).unwrap()
    }

    #[wasm_bindgen]
    pub fn get_custom_words_info(&self) -> JsValue {
        let words: Vec<Vec<serde_json::Value>> = self.dictionary.iter()
            .filter(|(k, _)| !["@", "!", "EVAL", "APPLY"].contains(&k.as_str()))
            .map(|(k, v)| vec![
                serde_json::json!(k),
                serde_json::json!(v),
                serde_json::json!(false)
            ])
            .collect();
        serde_wasm_bindgen::to_value(&words).unwrap()
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) -> JsValue {
        self.stack.clear();
        self.dictionary.clear();
        self.dictionary.insert("@".into(), "EVAL".into());
        self.dictionary.insert("!".into(), "APPLY".into());
        self.output = "System reset.".into();
        
        serde_wasm_bindgen::to_value(&serde_json::json!({
            "status": "OK",
            "output": self.output.clone()
        })).unwrap()
    }
}
