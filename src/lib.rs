use wasm_bindgen::prelude::*;
use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{Zero, One, ToPrimitive, Signed};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::str::FromStr;

// ============================================================================
// Value型の定義
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Rational(BigRational),
    String(String),
    Bool(bool),
    Nil,
    Vector(Vec<Value>),
}

impl Value {
    pub fn to_display_string(&self) -> String {
        match self {
            Value::Rational(r) => {
                if r.is_integer() {
                    r.numer().to_string()
                } else {
                    format!("{}/{}", r.numer(), r.denom())
                }
            }
            Value::String(s) => format!("'{}'", s),
            Value::Bool(b) => b.to_string(),
            Value::Nil => "nil".to_string(),
            Value::Vector(v) => {
                let items: Vec<String> = v.iter().map(|val| val.to_display_string()).collect();
                format!("[{}]", items.join(" "))
            }
        }
    }

    pub fn is_function_name(&self) -> bool {
        matches!(self, Value::String(_))
    }
}

// ============================================================================
// スコープ指定
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Scope {
    Local,   // デフォルト: スタックトップのN個
    Map,     // @: Vector各要素に適用
    Reduce,  // *: Vector全体を単一値に
    Global,  // #: スタック全体を対象
}

// ============================================================================
// トークン
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Value(Value),
    Function(String, Scope),
}

// ============================================================================
// トライ木辞書
// ============================================================================

#[derive(Debug, Clone)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
            is_word: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrieDict {
    root: TrieNode,
}

impl TrieDict {
    pub fn new() -> Self {
        TrieDict {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::new);
        }
        node.is_word = true;
    }

    // 最長一致検索
    pub fn longest_match(&self, text: &str) -> Option<String> {
        let mut node = &self.root;
        let mut longest = None;
        let mut current = String::new();

        for ch in text.chars() {
            if let Some(next) = node.children.get(&ch) {
                current.push(ch);
                node = next;
                if node.is_word {
                    longest = Some(current.clone());
                }
            } else {
                break;
            }
        }

        longest
    }
}

// ============================================================================
// インタープリタ
// ============================================================================

#[wasm_bindgen]
pub struct Interpreter {
    stack: Vec<Value>,
    dictionary: HashMap<String, Vec<Token>>,
    builtin_dict: TrieDict,
    output: Vec<String>,
}

#[wasm_bindgen]
impl Interpreter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut builtin_dict = TrieDict::new();
        
        // 組み込みワードを登録
        let builtins = vec![
            "add", "sub", "mul", "div", "pow", "mod",
            "dup", "drop", "swap", "over", "rot",
            "vec", "unpack", "nth", "slice", "concat", "length",
            "run", "step", "quote",
            "def", "undef", "words",
            "print", "clear",
            "eq", "lt", "gt", "le", "ge",
        ];
        
        for word in builtins {
            builtin_dict.insert(word);
        }

        Interpreter {
            stack: Vec::new(),
            dictionary: HashMap::new(),
            builtin_dict,
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

    fn tokenize(&self, input: &str) -> Result<Vec<Token>, JsValue> {
        let mut tokens = Vec::new();
        let mut pos = 0;
        let chars: Vec<char> = input.chars().collect();

        while pos < chars.len() {
            let ch = chars[pos];

            // 空白をスキップ
            if ch.is_whitespace() {
                pos += 1;
                continue;
            }

            // コメント
            if ch == '#' {
                while pos < chars.len() && chars[pos] != '\n' {
                    pos += 1;
                }
                continue;
            }

            // 文字列リテラル
            if ch == '\'' {
                pos += 1;
                let mut string = String::new();
                while pos < chars.len() && chars[pos] != '\'' {
                    string.push(chars[pos]);
                    pos += 1;
                }
                if pos >= chars.len() {
                    return Err(JsValue::from_str("Unterminated string"));
                }
                pos += 1; // closing '
                tokens.push(Token::Value(Value::String(string)));
                continue;
            }

            // Vectorリテラル
            if ch == '[' {
                let start = pos;
                let mut depth = 0;
                while pos < chars.len() {
                    if chars[pos] == '[' {
                        depth += 1;
                    } else if chars[pos] == ']' {
                        depth -= 1;
                        if depth == 0 {
                            pos += 1;
                            break;
                        }
                    }
                    pos += 1;
                }
                
                let vector_str: String = chars[start..pos].iter().collect();
                let vector = self.parse_vector(&vector_str)?;
                tokens.push(Token::Value(vector));
                continue;
            }

            // スコーププレフィックスの検出
            let scope = match ch {
                '@' => {
                    pos += 1;
                    Scope::Map
                }
                '*' => {
                    pos += 1;
                    Scope::Reduce
                }
                '#' => {
                    pos += 1;
                    Scope::Global
                }
                _ => Scope::Local,
            };

            // 現在位置から残りのテキスト
            let remaining: String = chars[pos..].iter().collect();

            // 数値リテラル
            if let Some(num_token) = self.try_parse_number(&remaining) {
                let num_len = self.get_number_length(&remaining);
                pos += num_len;
                tokens.push(Token::Value(num_token));
                continue;
            }

            // 予約語
            if remaining.starts_with("true") {
                pos += 4;
                tokens.push(Token::Value(Value::Bool(true)));
                continue;
            }
            if remaining.starts_with("false") {
                pos += 5;
                tokens.push(Token::Value(Value::Bool(false)));
                continue;
            }
            if remaining.starts_with("nil") {
                pos += 3;
                tokens.push(Token::Value(Value::Nil));
                continue;
            }

            // 辞書の最長一致
            if let Some(func_name) = self.builtin_dict.longest_match(&remaining) {
                pos += func_name.len();
                tokens.push(Token::Function(func_name, scope));
                continue;
            }

            // カスタムワード
            if let Some(func_name) = self.find_custom_word(&remaining) {
                pos += func_name.len();
                tokens.push(Token::Function(func_name, scope));
                continue;
            }

            return Err(JsValue::from_str(&format!("Unknown token at position {}", pos)));
        }

        Ok(tokens)
    }

    fn try_parse_number(&self, text: &str) -> Option<Value> {
        // 科学記法
        if let Some(e_pos) = text.find(|c| c == 'e' || c == 'E') {
            let base_str = &text[..e_pos];
            let exp_str = &text[e_pos + 1..];
            
            if let (Some(base), Some(exp)) = (self.try_parse_simple_number(base_str), exp_str.chars().take_while(|c| c.is_numeric() || *c == '-').collect::<String>().parse::<i32>().ok()) {
                if let Value::Rational(base_rat) = base {
                    let multiplier = BigRational::from_integer(
                        BigInt::from(10).pow(exp.abs() as u32)
                    );
                    let result = if exp >= 0 {
                        base_rat * multiplier
                    } else {
                        base_rat / multiplier
                    };
                    return Some(Value::Rational(result));
                }
            }
        }

        // 通常の数値
        self.try_parse_simple_number(text)
    }

    fn try_parse_simple_number(&self, text: &str) -> Option<Value> {
        // 分数
        if let Some(slash_pos) = text.find('/') {
            let num_str = &text[..slash_pos];
            let rest = &text[slash_pos + 1..];
            let den_str: String = rest.chars().take_while(|c| c.is_numeric()).collect();
            
            if let (Ok(num), Ok(den)) = (BigInt::from_str(num_str), BigInt::from_str(&den_str)) {
                return Some(Value::Rational(BigRational::new(num, den)));
            }
        }

        // 小数
        if text.contains('.') {
            let num_str: String = text.chars().take_while(|c| c.is_numeric() || *c == '.' || *c == '-').collect();
            if let Ok(f) = num_str.parse::<f64>() {
                return Some(self.float_to_rational(f));
            }
        }

        // 整数
        let num_str: String = text.chars().take_while(|c| c.is_numeric() || *c == '-').collect();
        if let Ok(n) = BigInt::from_str(&num_str) {
            return Some(Value::Rational(BigRational::from_integer(n)));
        }

        None
    }

    fn get_number_length(&self, text: &str) -> usize {
        if text.contains('e') || text.contains('E') {
            // 科学記法の長さ
            text.chars().take_while(|c| c.is_numeric() || *c == '.' || *c == '-' || *c == 'e' || *c == 'E').count()
        } else if text.contains('/') {
            // 分数の長さ
            let slash_pos = text.find('/').unwrap();
            slash_pos + 1 + text[slash_pos + 1..].chars().take_while(|c| c.is_numeric()).count()
        } else {
            // 通常の数値の長さ
            text.chars().take_while(|c| c.is_numeric() || *c == '.' || *c == '-').count()
        }
    }

    fn float_to_rational(&self, f: f64) -> Value {
        let s = format!("{:.10}", f);
        let parts: Vec<&str> = s.split('.').collect();
        
        if parts.len() == 1 {
            return Value::Rational(BigRational::from_integer(BigInt::from(f as i64)));
        }
        
        let integer_part = parts[0].parse::<i64>().unwrap_or(0);
        let decimal_part = parts[1].trim_end_matches('0');
        
        if decimal_part.is_empty() {
            return Value::Rational(BigRational::from_integer(BigInt::from(integer_part)));
        }
        
        let decimal_value = decimal_part.parse::<i64>().unwrap_or(0);
        let denominator = 10_i64.pow(decimal_part.len() as u32);
        let numerator = integer_part * denominator + decimal_value;
        
        Value::Rational(BigRational::new(
            BigInt::from(numerator),
            BigInt::from(denominator)
        ))
    }

    fn find_custom_word(&self, text: &str) -> Option<String> {
        let mut longest = None;
        let mut max_len = 0;

        for word in self.dictionary.keys() {
            if text.starts_with(word) && word.len() > max_len {
                max_len = word.len();
                longest = Some(word.clone());
            }
        }

        longest
    }

    fn parse_vector(&self, text: &str) -> Result<Value, JsValue> {
        let inner = &text[1..text.len() - 1].trim();
        
        if inner.is_empty() {
            return Ok(Value::Vector(Vec::new()));
        }

        let tokens = self.tokenize(inner)?;
        let mut values = Vec::new();

        for token in tokens {
            match token {
                Token::Value(v) => values.push(v),
                Token::Function(name, _scope) => {
                    // 関数名を文字列として保存
                    values.push(Value::String(name));
                }
            }
        }

        Ok(Value::Vector(values))
    }

    fn execute_token(&mut self, token: Token) -> Result<(), JsValue> {
        match token {
            Token::Value(v) => {
                self.stack.push(v);
                Ok(())
            }
            Token::Function(name, scope) => {
                self.execute_function(&name, scope)
            }
        }
    }

    fn execute_function(&mut self, name: &str, scope: Scope) -> Result<(), JsValue> {
        match scope {
            Scope::Local => self.execute_local(name),
            Scope::Map => self.execute_map(name),
            Scope::Reduce => self.execute_reduce(name),
            Scope::Global => self.execute_global(name),
        }
    }

    fn execute_local(&mut self, name: &str) -> Result<(), JsValue> {
        match name {
            // 算術演算
            "add" => {
                let b = self.pop()?;
                let a = self.pop()?;
                match (a, b) {
                    (Value::Rational(x), Value::Rational(y)) => {
                        self.stack.push(Value::Rational(x + y));
                    }
                    _ => return Err(JsValue::from_str("add requires two numbers")),
                }
            }
            "sub" => {
                let b = self.pop()?;
                let a = self.pop()?;
                match (a, b) {
                    (Value::Rational(x), Value::Rational(y)) => {
                        self.stack.push(Value::Rational(x - y));
                    }
                    _ => return Err(JsValue::from_str("sub requires two numbers")),
                }
            }
            "mul" => {
                let b = self.pop()?;
                let a = self.pop()?;
                match (a, b) {
                    (Value::Rational(x), Value::Rational(y)) => {
                        self.stack.push(Value::Rational(x * y));
                    }
                    _ => return Err(JsValue::from_str("mul requires two numbers")),
                }
            }
            "div" => {
                let b = self.pop()?;
                let a = self.pop()?;
                match (a, b) {
                    (Value::Rational(x), Value::Rational(y)) => {
                        if y.is_zero() {
                            return Err(JsValue::from_str("Division by zero"));
                        }
                        self.stack.push(Value::Rational(x / y));
                    }
                    _ => return Err(JsValue::from_str("div requires two numbers")),
                }
            }
            "pow" => {
                let b = self.pop()?;
                let a = self.pop()?;
                match (a, b) {
                    (Value::Rational(base), Value::Rational(exp)) => {
                        if !exp.is_integer() {
                            return Err(JsValue::from_str("pow requires integer exponent"));
                        }
                        let exp_int = exp.to_integer();
                        if let Some(exp_i32) = exp_int.to_i32() {
                            if exp_i32.abs() > 10000 {
                                return Err(JsValue::from_str("Exponent too large (max 10000)"));
                            }
                            let result = base.pow(exp_i32);
                            self.stack.push(Value::Rational(result));
                        } else {
                            return Err(JsValue::from_str("Exponent out of range"));
                        }
                    }
                    _ => return Err(JsValue::from_str("pow requires two numbers")),
                }
            }

            // スタック操作
            "dup" => {
                let top = self.stack.last()
                    .ok_or_else(|| JsValue::from_str("Stack underflow"))?
                    .clone();
                self.stack.push(top);
            }
            "drop" => {
                self.pop()?;
            }
            "swap" => {
                let b = self.pop()?;
                let a = self.pop()?;
                self.stack.push(b);
                self.stack.push(a);
            }
            "over" => {
                if self.stack.len() < 2 {
                    return Err(JsValue::from_str("Stack underflow"));
                }
                let second = self.stack[self.stack.len() - 2].clone();
                self.stack.push(second);
            }
            "rot" => {
                if self.stack.len() < 3 {
                    return Err(JsValue::from_str("Stack underflow"));
                }
                let c = self.pop()?;
                let b = self.pop()?;
                let a = self.pop()?;
                self.stack.push(b);
                self.stack.push(c);
                self.stack.push(a);
            }

            // Vector操作
            "vec" => {
                let n = self.pop()?;
                match n {
                    Value::Rational(r) => {
                        if !r.is_integer() {
                            return Err(JsValue::from_str("vec requires integer count"));
                        }
                        let count = r.to_integer().to_usize()
                            .ok_or_else(|| JsValue::from_str("Invalid count"))?;
                        
                        if self.stack.len() < count {
                            return Err(JsValue::from_str("Stack underflow"));
                        }
                        
                        let start = self.stack.len() - count;
                        let elements: Vec<Value> = self.stack.drain(start..).collect();
                        self.stack.push(Value::Vector(elements));
                    }
                    _ => return Err(JsValue::from_str("vec requires number")),
                }
            }
            "unpack" => {
                let v = self.pop()?;
                match v {
                    Value::Vector(elements) => {
                        for elem in elements {
                            self.stack.push(elem);
                        }
                    }
                    _ => return Err(JsValue::from_str("unpack requires vector")),
                }
            }
            "nth" => {
                let idx = self.pop()?;
                let vec = self.pop()?;
                match (vec, idx) {
                    (Value::Vector(v), Value::Rational(n)) => {
                        let index = n.to_integer().to_i64()
                            .ok_or_else(|| JsValue::from_str("Invalid index"))?;
                        
                        let actual_idx = if index < 0 {
                            (v.len() as i64 + index) as usize
                        } else {
                            index as usize
                        };
                        
                        if actual_idx >= v.len() {
                            return Err(JsValue::from_str("Index out of bounds"));
                        }
                        
                        self.stack.push(v[actual_idx].clone());
                    }
                    _ => return Err(JsValue::from_str("nth requires vector and number")),
                }
            }
            "length" => {
                let v = self.pop()?;
                match v {
                    Value::Vector(vec) => {
                        self.stack.push(Value::Rational(BigRational::from_integer(
                            BigInt::from(vec.len())
                        )));
                    }
                    _ => return Err(JsValue::from_str("length requires vector")),
                }
            }
            "concat" => {
                let b = self.pop()?;
                let a = self.pop()?;
                match (a, b) {
                    (Value::Vector(mut v1), Value::Vector(v2)) => {
                        v1.extend(v2);
                        self.stack.push(Value::Vector(v1));
                    }
                    _ => return Err(JsValue::from_str("concat requires two vectors")),
                }
            }

            // 実行制御
            "run" => {
                let v = self.pop()?;
                match v {
                    Value::Vector(elements) => {
                        for elem in elements {
                            if let Value::String(func_name) = elem {
                                self.execute_function(&func_name, Scope::Local)?;
                            } else {
                                self.stack.push(elem);
                            }
                        }
                    }
                    _ => return Err(JsValue::from_str("run requires vector")),
                }
            }
            "quote" => {
                let v = self.pop()?;
                self.stack.push(Value::Vector(vec![v]));
            }

            // 辞書操作
            "def" => {
                let name = self.pop()?;
                let body = self.pop()?;
                
                match (name, body) {
                    (Value::String(n), Value::Vector(tokens)) => {
                        // トークンを保存（簡易版）
                        let token_list: Vec<Token> = tokens.into_iter().map(|v| {
                            Token::Value(v)
                        }).collect();
                        
                        self.dictionary.insert(n, token_list);
                    }
                    _ => return Err(JsValue::from_str("def requires string name and vector body")),
                }
            }

            // I/O
            "print" => {
                let v = self.pop()?;
                self.output.push(v.to_display_string());
            }
            "clear" => {
                self.output.clear();
            }

            _ => {
                // カスタムワード
                if let Some(tokens) = self.dictionary.get(name).cloned() {
                    for token in tokens {
                        self.execute_token(token)?;
                    }
                } else {
                    return Err(JsValue::from_str(&format!("Unknown word: {}", name)));
                }
            }
        }
        Ok(())
    }

    fn execute_map(&mut self, name: &str) -> Result<(), JsValue> {
        let vec = self.pop()?;
        
        match vec {
            Value::Vector(elements) => {
                let mut results = Vec::new();
                
                for elem in elements {
                    self.stack.push(elem);
                    self.execute_local(name)?;
                    results.push(self.pop()?);
                }
                
                self.stack.push(Value::Vector(results));
            }
            _ => return Err(JsValue::from_str("@ requires vector")),
        }
        
        Ok(())
    }

    fn execute_reduce(&mut self, name: &str) -> Result<(), JsValue> {
        let vec = self.pop()?;
        
        match vec {
            Value::Vector(elements) => {
                if elements.is_empty() {
                    return Err(JsValue::from_str("Cannot reduce empty vector"));
                }
                
                let mut result = elements[0].clone();
                
                for elem in elements.into_iter().skip(1) {
                    self.stack.push(result);
                    self.stack.push(elem);
                    self.execute_local(name)?;
                    result = self.pop()?;
                }
                
                self.stack.push(result);
            }
            _ => return Err(JsValue::from_str("* requires vector")),
        }
        
        Ok(())
    }

    fn execute_global(&mut self, name: &str) -> Result<(), JsValue> {
        // スタック全体を一つのVectorとして扱う
        let all_elements = self.stack.drain(..).collect::<Vec<_>>();
        
        if all_elements.is_empty() {
            return Err(JsValue::from_str("Stack is empty"));
        }
        
        self.stack.push(Value::Vector(all_elements));
        self.execute_reduce(name)?;
        
        Ok(())
    }

    fn pop(&mut self) -> Result<Value, JsValue> {
        self.stack.pop()
            .ok_or_else(|| JsValue::from_str("Stack underflow"))
    }

    pub fn get_stack_json(&self) -> String {
        let stack_str: Vec<String> = self.stack.iter()
            .map(|v| v.to_display_string())
            .collect();
        serde_json::to_string(&stack_str).unwrap_or("[]".to_string())
    }

    pub fn get_output(&self) -> String {
        self.output.join("\n")
    }

    pub fn clear_output(&mut self) {
        self.output.clear();
    }

    pub fn get_stack_size(&self) -> usize {
        self.stack.len()
    }
}

// パニックフック設定
#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
