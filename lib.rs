use wasm_bindgen::prelude::*;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Zero, One, ToPrimitive};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::str::FromStr;

// 数値型の定義（内部的には分数）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Number {
    value: BigRational,
}

impl Number {
    pub fn new(numerator: i64, denominator: i64) -> Self {
        Number {
            value: BigRational::new(
                BigInt::from(numerator),
                BigInt::from(denominator)
            ),
        }
    }

    pub fn from_float(f: f64) -> Self {
        // 浮動小数点数を分数に変換
        let s = format!("{:.10}", f);
        let parts: Vec<&str> = s.split('.').collect();
        
        if parts.len() == 1 {
            return Number::new(f as i64, 1);
        }
        
        let integer_part = parts[0].parse::<i64>().unwrap_or(0);
        let decimal_part = parts[1];
        let decimal_value = decimal_part.parse::<i64>().unwrap_or(0);
        let denominator = 10_i64.pow(decimal_part.len() as u32);
        
        let numerator = integer_part * denominator + decimal_value;
        Number::new(numerator, denominator)
    }

    pub fn from_string(s: &str) -> Result<Self, String> {
        // 科学記法のサポート
        if s.contains('e') || s.contains('E') {
            let parts: Vec<&str> = s.split(|c| c == 'e' || c == 'E').collect();
            if parts.len() != 2 {
                return Err("Invalid scientific notation".to_string());
            }
            
            let base = Number::from_string(parts[0])?;
            let exp = parts[1].parse::<u32>().map_err(|_| "Invalid exponent")?;
            
            let multiplier = BigRational::from_integer(
                BigInt::from(10).pow(exp)
            );
            Ok(Number { value: base.value * multiplier })
        } else if s.contains('/') {
            // 分数記法
            let parts: Vec<&str> = s.split('/').collect();
            if parts.len() != 2 {
                return Err("Invalid fraction".to_string());
            }
            let num = BigInt::from_str(parts[0]).map_err(|_| "Invalid numerator")?;
            let den = BigInt::from_str(parts[1]).map_err(|_| "Invalid denominator")?;
            Ok(Number { value: BigRational::new(num, den) })
        } else if s.contains('.') {
            // 小数
            Ok(Number::from_float(s.parse().map_err(|_| "Invalid decimal")?))
        } else {
            // 整数
            let num = BigInt::from_str(s).map_err(|_| "Invalid integer")?;
            Ok(Number { value: BigRational::from_integer(num) })
        }
    }

    pub fn to_string(&self) -> String {
        if self.value.is_integer() {
            self.value.numer().to_string()
        } else {
            format!("{}/{}", self.value.numer(), self.value.denom())
        }
    }
}

// Lycorisの値型
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Number(Number),
    Boolean(bool),
    String(String),
    Nil,
    Vector(Vec<Value>),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::String(s) => format!("\"{}\"", s),
            Value::Nil => "nil".to_string(),
            Value::Vector(v) => {
                let items: Vec<String> = v.iter().map(|val| val.to_string()).collect();
                format!("[{}]", items.join(" "))
            }
        }
    }
}

// インタープリタ
#[wasm_bindgen]
pub struct Interpreter {
    stack: Vec<Value>,
    dictionary: HashMap<String, Vec<String>>,
    output: Vec<String>,
}

#[wasm_bindgen]
impl Interpreter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Interpreter {
            stack: Vec::new(),
            dictionary: HashMap::new(),
            output: Vec::new(),
        }
    }

    pub fn execute(&mut self, input: String) -> Result<String, JsValue> {
        let tokens = self.tokenize(&input)?;
        
        for token in tokens {
            self.execute_token(token)?;
        }
        
        Ok(self.output.join("\n"))
    }

    fn tokenize(&self, input: &str) -> Result<Vec<String>, JsValue> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut in_string = false;
        let mut in_vector = false;
        let mut vector_depth = 0;
        let mut in_comment = false;
        
        for ch in input.chars() {
            if in_comment {
                if ch == '\n' {
                    in_comment = false;
                }
                continue;
            }
            
            match ch {
                '#' if !in_string && !in_vector => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                    in_comment = true;
                }
                '"' => {
                    in_string = !in_string;
                    current.push(ch);
                    if !in_string && !in_vector {
                        tokens.push(current.clone());
                        current.clear();
                    }
                }
                '[' if !in_string => {
                    in_vector = true;
                    vector_depth += 1;
                    current.push(ch);
                }
                ']' if !in_string => {
                    current.push(ch);
                    vector_depth -= 1;
                    if vector_depth == 0 {
                        in_vector = false;
                        tokens.push(current.clone());
                        current.clear();
                    }
                }
                ' ' | '\t' | '\n' | '\r' => {
                    if in_string || in_vector {
                        current.push(ch);
                    } else if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                }
                _ => {
                    current.push(ch);
                }
            }
        }
        
        if !current.is_empty() && !in_comment {
            tokens.push(current);
        }
        
        Ok(tokens)
    }

    fn execute_token(&mut self, token: String) -> Result<(), JsValue> {
        // 条件分岐マーカー
        if token == ":" {
            // TODO: 条件分岐の実装
            return Ok(());
        }

        // Vector リテラル
        if token.starts_with('[') && token.ends_with(']') {
            let vector = self.parse_vector(&token)?;
            self.stack.push(vector);
            return Ok(());
        }

        // 文字列リテラル
        if token.starts_with('"') && token.ends_with('"') {
            let content = &token[1..token.len()-1];
            self.stack.push(Value::String(content.to_string()));
            return Ok(());
        }

        // クォート付きワード名（DEF用）
        if token.starts_with('\'') {
            let word_name = &token[1..];
            self.stack.push(Value::String(word_name.to_string()));
            return Ok(());
        }

        // 数値リテラル
        if let Ok(number) = Number::from_string(&token) {
            self.stack.push(Value::Number(number));
            return Ok(());
        }

        // 真偽値
        if token == "true" {
            self.stack.push(Value::Boolean(true));
            return Ok(());
        }
        if token == "false" {
            self.stack.push(Value::Boolean(false));
            return Ok(());
        }

        // Nil
        if token == "nil" {
            self.stack.push(Value::Nil);
            return Ok(());
        }

        // 組み込みワードまたはカスタムワード
        self.execute_word(&token)
    }

    fn parse_vector(&self, token: &str) -> Result<Value, JsValue> {
        let inner = &token[1..token.len()-1];
        if inner.is_empty() {
            return Ok(Value::Vector(Vec::new()));
        }

        let tokens = self.tokenize(inner)?;
        let mut values = Vec::new();

        for t in tokens {
            if t.starts_with('[') && t.ends_with(']') {
                values.push(self.parse_vector(&t)?);
            } else if t.starts_with('"') && t.ends_with('"') {
                let content = &t[1..t.len()-1];
                values.push(Value::String(content.to_string()));
            } else if let Ok(number) = Number::from_string(&t) {
                values.push(Value::Number(number));
            } else if t == "true" {
                values.push(Value::Boolean(true));
            } else if t == "false" {
                values.push(Value::Boolean(false));
            } else if t == "nil" {
                values.push(Value::Nil);
            } else {
                return Err(JsValue::from_str(&format!("Invalid vector element: {}", t)));
            }
        }

        Ok(Value::Vector(values))
    }

    fn execute_word(&mut self, word: &str) -> Result<(), JsValue> {
        match word {
            // 算術演算
            "ADD" => {
                let b = self.pop()?;
                let a = self.pop()?;
                
                match (a, b) {
                    (Value::Number(n1), Value::Number(n2)) => {
                        self.stack.push(Value::Number(Number { 
                            value: n1.value + n2.value 
                        }));
                    }
                    (Value::Vector(v), Value::Number(n)) => {
                        let result: Vec<Value> = v.iter().map(|val| {
                            if let Value::Number(vn) = val {
                                Value::Number(Number { 
                                    value: vn.value.clone() + &n.value 
                                })
                            } else {
                                val.clone()
                            }
                        }).collect();
                        self.stack.push(Value::Vector(result));
                    }
                    _ => return Err(JsValue::from_str("Type error in ADD"))
                }
            }
            "SUBTRACT" => {
                let b = self.pop()?;
                let a = self.pop()?;
                
                match (a, b) {
                    (Value::Number(n1), Value::Number(n2)) => {
                        self.stack.push(Value::Number(Number { 
                            value: n1.value - n2.value 
                        }));
                    }
                    _ => return Err(JsValue::from_str("Type error in SUBTRACT"))
                }
            }
            "MULTIPLY" => {
                let b = self.pop()?;
                let a = self.pop()?;
                
                match (a, b) {
                    (Value::Number(n1), Value::Number(n2)) => {
                        self.stack.push(Value::Number(Number { 
                            value: n1.value * n2.value 
                        }));
                    }
                    _ => return Err(JsValue::from_str("Type error in MULTIPLY"))
                }
            }
            "DIVIDE" => {
                let b = self.pop()?;
                let a = self.pop()?;
                
                match (a, b) {
                    (Value::Number(n1), Value::Number(n2)) => {
                        if n2.value.is_zero() {
                            return Err(JsValue::from_str("Division by zero"));
                        }
                        self.stack.push(Value::Number(Number { 
                            value: n1.value / n2.value 
                        }));
                    }
                    _ => return Err(JsValue::from_str("Type error in DIVIDE"))
                }
            }
            "POWER" => {
                let b = self.pop()?;
                let a = self.pop()?;
                
                match (a, b) {
                    (Value::Number(base), Value::Number(exp)) => {
                        // 指数が整数の場合のみサポート
                        if exp.value.is_integer() {
                            let exp_int = exp.value.to_integer();
                            
                            // 小さな整数に変換可能か確認
                            if let Some(exp_u32) = exp_int.to_u32() {
                                if exp_u32 <= 10000 {  // 安全のため上限を設定
                                    let result = base.value.pow(exp_u32);
                                    self.stack.push(Value::Number(Number { value: result }));
                                } else {
                                    return Err(JsValue::from_str("Exponent too large (max 10000)"));
                                }
                            } else {
                                return Err(JsValue::from_str("Exponent must be a positive integer <= 10000"));
                            }
                        } else {
                            return Err(JsValue::from_str("POWER requires integer exponent"));
                        }
                    }
                    _ => return Err(JsValue::from_str("Type error in POWER"))
                }
            }
            // スタック操作
            "DUPLICATE" => {
                let top = self.stack.last()
                    .ok_or_else(|| JsValue::from_str("Stack underflow"))?
                    .clone();
                self.stack.push(top);
            }
            "DROP" => {
                self.pop()?;
            }
            "SWAP" => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.stack.push(b);
                self.stack.push(a);
            }
            // I/O
            "PRINT" => {
                let value = self.pop()?;
                self.output.push(value.to_string());
            }
            "CLEAR" => {
                self.output.clear();
            }
            // Vector操作
            "GET" => {
                let index = self.pop()?;
                let vector = self.pop()?;
                
                match (vector, index) {
                    (Value::Vector(v), Value::Number(n)) => {
                        let idx = n.value.to_integer().to_i64().unwrap_or(0);
                        let len = v.len() as i64;
                        
                        let actual_idx = if idx < 0 {
                            (len + idx) as usize
                        } else {
                            idx as usize
                        };
                        
                        if actual_idx < v.len() {
                            self.stack.push(v[actual_idx].clone());
                        } else {
                            return Err(JsValue::from_str("Index out of bounds"));
                        }
                    }
                    _ => return Err(JsValue::from_str("Type error in GET"))
                }
            }
            // 辞書操作
            "DEF" => {
                let name = self.pop()?;
                let _definition = self.pop()?;
                
                match name {
                    Value::String(n) => {
                        // TODO: 実際の定義を保存
                        self.dictionary.insert(n, vec![]);
                    }
                    _ => return Err(JsValue::from_str("DEF requires a string name"))
                }
            }
            _ => {
                // カスタムワードをチェック
                if let Some(definition) = self.dictionary.get(word) {
                    let def_clone = definition.clone();
                    for token in def_clone {
                        self.execute_token(token)?;
                    }
                } else {
                    return Err(JsValue::from_str(&format!("Unknown word: {}", word)));
                }
            }
        }
        Ok(())
    }

    fn pop(&mut self) -> Result<Value, JsValue> {
        self.stack.pop()
            .ok_or_else(|| JsValue::from_str("Stack underflow"))
    }

    pub fn get_stack_json(&self) -> String {
        let stack_str: Vec<String> = self.stack.iter().map(|v| v.to_string()).collect();
        serde_json::to_string(&stack_str).unwrap_or("[]".to_string())
    }

    pub fn get_output(&self) -> String {
        self.output.join("\n")
    }

    pub fn get_dictionary_json(&self) -> String {
        let mut words: Vec<Vec<String>> = Vec::new();
        
        // 組み込みワード
        let builtins = vec![
            ("ADD", "[built-in]", "red"),
            ("SUBTRACT", "[built-in]", "red"),
            ("MULTIPLY", "[built-in]", "red"),
            ("DIVIDE", "[built-in]", "red"),
            ("POWER", "[built-in]", "red"),
            ("DUPLICATE", "[built-in]", "red"),
            ("DROP", "[built-in]", "red"),
            ("SWAP", "[built-in]", "red"),
            ("PRINT", "[built-in]", "red"),
            ("CLEAR", "[built-in]", "red"),
            ("GET", "[built-in]", "red"),
            ("DEF", "[built-in]", "red"),
        ];
        
        for (name, content, color) in builtins {
            words.push(vec![name.to_string(), content.to_string(), color.to_string()]);
        }
        
        // カスタムワード
        for (name, _def) in &self.dictionary {
            words.push(vec![name.clone(), "[custom]".to_string(), "green".to_string()]);
        }
        
        serde_json::to_string(&words).unwrap_or("[]".to_string())
    }

    pub fn save_state(&self) -> String {
        serde_json::to_string(&self.dictionary).unwrap_or("{}".to_string())
    }

    pub fn load_state(&mut self, state: &str) -> Result<(), JsValue> {
        if let Ok(dict) = serde_json::from_str::<HashMap<String, Vec<String>>>(state) {
            self.dictionary = dict;
            Ok(())
        } else {
            Err(JsValue::from_str("Failed to parse state"))
        }
    }
}

// パニックフック設定
#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
