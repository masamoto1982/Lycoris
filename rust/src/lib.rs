use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use num_bigint::BigInt;
use num_traits::{Zero, One};
use std::str::FromStr;
use num_integer::Integer;

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

// BigIntを文字列としてシリアライズ/デシリアライズするヘルパー
fn serialize_bigint<S>(bigint: &BigInt, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&bigint.to_string())
}

fn deserialize_bigint<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    BigInt::from_str(&s).map_err(serde::de::Error::custom)
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Value {
    #[serde(rename_all = "camelCase")]
    Number { 
        #[serde(serialize_with = "serialize_bigint", deserialize_with = "deserialize_bigint")]
        numerator: BigInt, 
        #[serde(serialize_with = "serialize_bigint", deserialize_with = "deserialize_bigint")]
        denominator: BigInt 
    },
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
        (n / common.clone(), d / common)
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
    dict.insert("DUP".to_string(), vec![Value::Symbol("DUP".to_string())]);
    dict.insert("DROP".to_string(), vec![Value::Symbol("DROP".to_string())]);
    dict.insert("SWAP".to_string(), vec![Value::Symbol("SWAP".to_string())]);
    dict.insert("PRINT".to_string(), vec![Value::Symbol("PRINT".to_string())]);
    dict.insert("DEF".to_string(), vec![Value::Symbol("DEF".to_string())]);
    dict.insert("IF".to_string(), vec![Value::Symbol(":".to_string())]);
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
        
        // 改行でコードブロックを分割
        let lines: Vec<&str> = code.split('\n').collect();
        
        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            
            let tokens = self.tokenize(line);
            let values = self.parse(&tokens);

            if let Err(e) = self.eval_tokens(&values) {
                return Ok(serde_wasm_bindgen::to_value(&serde_json::json!({
                    "status": "ERROR", "message": e, "error": true
                })).unwrap());
            }
        }

        Ok(serde_wasm_bindgen::to_value(&serde_json::json!({
            "status": "OK", "output": self.output.clone(),
        })).unwrap())
    }

    fn tokenize(&self, code: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut in_string = false;
        let mut escape_next = false;

        for c in code.chars() {
            if escape_next {
                current.push(c);
                escape_next = false;
                continue;
            }

            match c {
                '\\' => {
                    escape_next = true;
                    continue;
                }
                '"' => {
                    if in_string {
                        current.push('"');
                        tokens.push(current.clone());
                        current.clear();
                        in_string = false;
                    } else {
                        if !current.is_empty() {
                            tokens.push(current.clone());
                            current.clear();
                        }
                        current.push('"');
                        in_string = true;
                    }
                    continue;
                }
                ' ' | '\t' if !in_string => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                    continue;
                }
                _ => {
                    current.push(c);
                }
            }
        }

        if !current.is_empty() {
            tokens.push(current);
        }

        tokens
    }

    fn parse(&self, tokens: &[String]) -> Vec<Value> {
        let mut values = Vec::new();
        
        for token in tokens {
            // 辞書ワードをチェック
            if self.dictionary.contains_key(&token.to_uppercase()) {
                values.push(Value::Symbol(token.to_string()));
                continue;
            }
            
            // 文字列
            if token.starts_with('"') && token.ends_with('"') {
                let content = &token[1..token.len()-1];
                values.push(Value::String(content.to_string()));
                continue;
            }
            
            // 真偽値
            if token == "TRUE" {
                values.push(Value::Boolean(true));
                continue;
            }
            if token == "FALSE" {
                values.push(Value::Boolean(false));
                continue;
            }
            
            // Nil
            if token == "NIL" {
                values.push(Value::Nil);
                continue;
            }
            
            // 数値（整数、小数、分数）
            if let Ok((n, d)) = self.parse_number(token) {
                values.push(Value::num(n, d));
                continue;
            }
            
            // その他はシンボル
            values.push(Value::Symbol(token.to_string()));
        }
        
        values
    }

    fn parse_number(&self, s: &str) -> Result<(BigInt, BigInt), ()> {
        // 分数の処理 (a/b形式)
        if s.contains('/') {
            let parts: Vec<&str> = s.split('/').collect();
            if parts.len() == 2 {
                if let (Ok(n), Ok(d)) = (
                    BigInt::from_str(parts[0]),
                    BigInt::from_str(parts[1])
                ) {
                    return Ok((n, d));
                }
            }
            return Err(());
        }
        
        // 小数の処理
        if s.contains('.') {
            let parts: Vec<&str> = s.split('.').collect();
            if parts.len() == 2 {
                if let (Ok(int_part), Ok(frac_part)) = (
                    BigInt::from_str(parts[0]),
                    BigInt::from_str(parts[1])
                ) {
                    let frac_str = parts[1];
                    let denom = BigInt::from(10).pow(frac_str.len() as u32);
                    let sign = if int_part.is_negative() { -BigInt::one() } else { BigInt::one() };
                    let numer = if int_part.is_zero() {
                        if s.starts_with('-') { -BigInt::from_str(frac_str).unwrap() } else { BigInt::from_str(frac_str).unwrap() }
                    } else {
                        &int_part.abs() * &denom + BigInt::from_str(frac_str).unwrap()
                    } * sign;
                    return Ok((numer, denom));
                }
            }
            return Err(());
        }
        
        // 整数の処理
        if let Ok(n) = BigInt::from_str(s) {
            return Ok((n, BigInt::one()));
        }
        
        Err(())
    }

    fn eval_tokens(&mut self, values: &[Value]) -> Result<(), String> {
        let mut i = 0;
        while i < values.len() {
            let value = &values[i];
            self.eval_value(value)?;
            i += 1;
        }
        Ok(())
    }

    fn eval_value(&mut self, value: &Value) -> Result<(), String> {
        if let Value::Symbol(s) = value {
            let s_upper = s.to_uppercase();
            
            // 辞書をチェック
            if let Some(def) = self.dictionary.get(&s_upper) {
                return self.eval_tokens(&def.clone());
            }

            match s_upper.as_str() {
                "@" | "EVAL" => self.op_eval(),
                "DUP" => self.op_dup(), 
                "DROP" => self.op_drop(), 
                "SWAP" => self.op_swap(),
                "+" | "-" | "*" | "/" => self.op_arithmetic(&s_upper),
                "=" | "<" | ">" | "<=" | ">=" => self.op_comparison(&s_upper),
                "AND" | "OR" | "NOT" => self.op_logic(&s_upper),
                "PRINT" => self.op_print(), 
                "DEF" => self.op_def(), 
                "?" => self.op_lookup(),
                ":" => self.op_if(), 
                "MAP" => self.op_map(), 
                "RESET" => self.op_reset(),
                _ => { 
                    self.stack.push(value.clone()); 
                    Ok(()) 
                }
            }
        } else {
            self.stack.push(value.clone());
            Ok(())
        }
    }

    fn pop_value(&mut self) -> Result<Value, String> {
        self.stack.pop().ok_or("Stack underflow".to_string())
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
            Value::Vector(v) => !v.is_empty(),
            Value::Nil => false,
            _ => true,
        }
    }

    fn op_arithmetic(&mut self, op: &str) -> Result<(), String> {
        let b_val = self.pop_value()?;
        let a_val = self.pop_value()?;
        let (an, ad) = self.extract_number(&a_val)?;
        let (bn, bd) = self.extract_number(&b_val)?;
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
            _ => unreachable!(),
        };
        self.stack.push(Value::num(rn, rd));
        Ok(())
    }
    
    fn op_comparison(&mut self, op: &str) -> Result<(), String> {
        let b = self.pop_value()?;
        let a = self.pop_value()?;
        let result = match (&a, &b) {
            (Value::Number{numerator: an, denominator: ad}, Value::Number{numerator: bn, denominator: bd}) => {
                match op {
                    "=" => an * bd == bn * ad,
                    "<" => an * bd < bn * ad,
                    ">" => an * bd > bn * ad,
                    "<=" => an * bd <= bn * ad,
                    ">=" => an * bd >= bn * ad,
                    _ => false,
                }
            },
            (Value::String(sa), Value::String(sb)) => match op {
                "=" => sa == sb,
                "<" => sa < sb,
                ">" => sa > sb,
                "<=" => sa <= sb,
                ">=" => sa >= sb,
                _ => false,
            },
            _ => return Err("Comparison requires two numbers or two strings".to_string()),
        };
        self.stack.push(Value::Boolean(result));
        Ok(())
    }

    fn op_logic(&mut self, op: &str) -> Result<(), String> {
        match op {
            "NOT" => {
                let val = self.pop_value()?;
                self.stack.push(Value::Boolean(!self.is_truthy(&val)));
            }
            "AND" | "OR" => {
                let b = self.pop_value()?;
                let a = self.pop_value()?;
                let result = if op == "AND" { 
                    self.is_truthy(&a) && self.is_truthy(&b) 
                } else { 
                    self.is_truthy(&a) || self.is_truthy(&b) 
                };
                self.stack.push(Value::Boolean(result));
            }
            _ => {}
        }
        Ok(())
    }

    fn op_eval(&mut self) -> Result<(), String> {
        let val = self.stack.pop().ok_or("Stack underflow for EVAL")?;
        if let Value::Vector(v) = val { 
            self.eval_tokens(&v) 
        } else { 
            Err("EVAL requires a vector".to_string()) 
        }
    }

    fn op_dup(&mut self) -> Result<(), String> {
        if let Some(val) = self.stack.last() {
            self.stack.push(val.clone());
            Ok(())
        } else {
            Err("Stack underflow for DUP".to_string())
        }
    }

    fn op_drop(&mut self) -> Result<(), String> {
        self.stack.pop().map(|_| ()).ok_or("Stack underflow for DROP".to_string())
    }

    fn op_swap(&mut self) -> Result<(), String> {
        if self.stack.len() < 2 { 
            return Err("Stack underflow for SWAP".to_string()); 
        }
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(a); 
        self.stack.push(b); 
        Ok(())
    }

    fn op_print(&mut self) -> Result<(), String> {
        let val = self.stack.pop().ok_or("Stack underflow for PRINT")?;
        self.output.push_str(&self.value_to_string(&val)); 
        self.output.push(' '); 
        Ok(())
    }

    fn op_def(&mut self) -> Result<(), String> {
        let name_val = self.pop_value()?;
        let body_val = self.stack.pop().ok_or("Stack underflow for DEF body")?;
        if let (Value::String(name), Value::Vector(body)) = (name_val, body_val) {
            self.dictionary.insert(name.to_uppercase(), body); 
            Ok(())
        } else { 
            Err("DEF requires a string name and a vector body".to_string()) 
        }
    }
    
    fn op_lookup(&mut self) -> Result<(), String> {
        let name = self.pop_value()?;
        if let Value::String(n) = name {
            let body = self.dictionary.get(&n.to_uppercase()).cloned().unwrap_or_default();
            self.stack.push(Value::Vector(body)); 
            Ok(())
        } else { 
            Err("? requires a string name".to_string()) 
        }
    }
    
    fn op_if(&mut self) -> Result<(), String> {
        let else_branch = self.stack.pop().ok_or("Stack underflow for : (else branch)")?;
        let then_branch = self.stack.pop().ok_or("Stack underflow for : (then branch)")?;
        let cond_val = self.pop_value()?;
        let branch_to_eval = if self.is_truthy(&cond_val) { then_branch } else { else_branch };
        if let Value::Vector(v) = branch_to_eval { 
            self.eval_tokens(&v) 
        } else { 
            Err("Branches for : must be vectors".to_string()) 
        }
    }

    fn op_map(&mut self) -> Result<(), String> {
        let func = self.stack.pop().ok_or("Stack underflow for MAP (function)")?;
        let data = self.stack.pop().ok_or("Stack underflow for MAP (data)")?;
        if let (Value::Vector(f_vec), Value::Vector(d_vec)) = (func, data) {
            let mut results = Vec::new();
            for item in d_vec {
                self.stack.push(item);
                self.eval_tokens(&f_vec)?;
                results.push(self.stack.pop().unwrap_or(Value::Nil));
            }
            self.stack.push(Value::Vector(results)); 
            Ok(())
        } else { 
            Err("MAP requires two vectors".to_string()) 
        }
    }

    fn op_reset(&mut self) -> Result<(), String> {
        self.stack.clear(); 
        self.dictionary = default_dictionary(); 
        self.output = "System reset.".into(); 
        Ok(())
    }

    fn value_to_string(&self, val: &Value) -> String {
        match val {
            Value::Number { numerator, denominator } if denominator == &One::one() => {
                numerator.to_string()
            },
            Value::Number { numerator, denominator } => {
                format!("{}/{}", numerator, denominator)
            },
            Value::String(s) => format!("\"{}\"", s),
            Value::Boolean(b) => if *b { "TRUE" } else { "FALSE" }.to_string(),
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
        let words: Vec<_> = self.dictionary.iter()
            .filter(|(k, _)| !default_dictionary().contains_key(*k))
            .map(|(k, v)| vec![k.clone(), self.value_to_string(&Value::Vector(v.clone()))])
            .collect();
        serde_wasm_bindgen::to_value(&words).unwrap()
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) -> JsValue {
        self.op_reset().unwrap();
        serde_wasm_bindgen::to_value(&serde_json::json!({ 
            "status": "OK", 
            "output": self.output.clone() 
        })).unwrap()
    }
}
