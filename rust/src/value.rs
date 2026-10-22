//! Values, their source-text display, and the JSON view the playground draws
//! (`Value` in `src/wasm-interpreter-types.ts`).

use serde::{Deserialize, Serialize};
use serde_json::json;

/// An exact fraction in lowest terms with a positive denominator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rational {
    pub num: i64,
    pub den: i64,
}

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.unsigned_abs(), b.unsigned_abs());
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a as i64
}

impl Rational {
    pub fn new(num: i64, den: i64) -> Result<Self, String> {
        if den == 0 {
            return Err("division by zero".to_string());
        }
        let g = gcd(num, den).max(1);
        let sign = if den < 0 { -1 } else { 1 };
        let num = (num / g).checked_mul(sign).ok_or_else(overflow)?;
        let den = (den / g).checked_mul(sign).ok_or_else(overflow)?;
        Ok(Rational { num, den })
    }

    pub fn integer(n: i64) -> Self {
        Rational { num: n, den: 1 }
    }

    /// `12`, `-3/4` or `1.25`.
    pub fn parse(text: &str) -> Option<Self> {
        let digits = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit());
        let (negative, body) = match text.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, text),
        };
        let value = if let Some((n, d)) = body.split_once('/') {
            if !digits(n) || !digits(d) {
                return None;
            }
            Rational::new(n.parse().ok()?, d.parse().ok()?).ok()?
        } else if let Some((int, frac)) = body.split_once('.') {
            if !digits(int) || !digits(frac) {
                return None;
            }
            let den = 10i64.checked_pow(frac.len() as u32)?;
            let num = format!("{int}{frac}").parse().ok()?;
            Rational::new(num, den).ok()?
        } else {
            if !digits(body) {
                return None;
            }
            Rational::integer(body.parse().ok()?)
        };
        Some(if negative {
            Rational {
                num: -value.num,
                den: value.den,
            }
        } else {
            value
        })
    }

    pub fn checked_add(self, o: Self) -> Result<Self, String> {
        let num = self
            .num
            .checked_mul(o.den)
            .and_then(|a| o.num.checked_mul(self.den).and_then(|b| a.checked_add(b)))
            .ok_or_else(overflow)?;
        Rational::new(num, self.den.checked_mul(o.den).ok_or_else(overflow)?)
    }

    pub fn checked_sub(self, o: Self) -> Result<Self, String> {
        self.checked_add(Rational {
            num: -o.num,
            den: o.den,
        })
    }

    pub fn checked_mul(self, o: Self) -> Result<Self, String> {
        let num = self.num.checked_mul(o.num).ok_or_else(overflow)?;
        Rational::new(num, self.den.checked_mul(o.den).ok_or_else(overflow)?)
    }

    pub fn checked_div(self, o: Self) -> Result<Self, String> {
        if o.num == 0 {
            return Err("division by zero".to_string());
        }
        self.checked_mul(Rational {
            num: o.den,
            den: o.num,
        })
    }

    pub fn lt(self, o: Self) -> bool {
        (self.num as i128) * (o.den as i128) < (o.num as i128) * (self.den as i128)
    }
}

fn overflow() -> String {
    "number out of range".to_string()
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Number(Rational),
    Str(String),
    Bool(bool),
    /// A word written inside `[ ... ]`: data until something runs it.
    Symbol(String),
    Vector(Vec<Value>),
    Nil,
}

impl Value {
    /// The value as source text that reads back as the same value. Numbers
    /// are always `numerator/denominator`, as the playground's stack area
    /// draws them (`formatNumber` in `src/gui/output-display-renderer.ts`).
    pub fn display(&self) -> String {
        match self {
            Value::Number(r) => format!("{}/{}", r.num, r.den),
            Value::Str(s) => format!("'{s}'"),
            Value::Bool(true) => "TRUE".to_string(),
            Value::Bool(false) => "FALSE".to_string(),
            Value::Symbol(name) => name.clone(),
            Value::Vector(items) if items.is_empty() => "[ ]".to_string(),
            Value::Vector(items) => {
                let inner: Vec<String> = items.iter().map(Value::display).collect();
                format!("[ {} ]", inner.join(" "))
            }
            Value::Nil => "NIL".to_string(),
        }
    }

    /// The node the playground's stack area draws.
    pub fn protocol(&self) -> serde_json::Value {
        match self {
            Value::Number(r) => json!({
                "type": "number",
                "value": { "numerator": r.num.to_string(), "denominator": r.den.to_string() },
            }),
            Value::Str(s) => json!({ "type": "string", "value": s }),
            Value::Bool(b) => json!({ "type": "boolean", "value": b }),
            Value::Symbol(name) => json!({ "type": "symbol", "value": name }),
            Value::Vector(items) => json!({
                "type": "vector",
                "value": items.iter().map(Value::protocol).collect::<Vec<_>>(),
            }),
            Value::Nil => json!({ "type": "nil", "value": null }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_integers_fractions_and_decimals() {
        assert_eq!(Rational::parse("42"), Some(Rational::integer(42)));
        assert_eq!(Rational::parse("-2/4"), Some(Rational { num: -1, den: 2 }));
        assert_eq!(Rational::parse("0.25"), Some(Rational { num: 1, den: 4 }));
        assert_eq!(Rational::parse("1/0"), None);
        assert_eq!(Rational::parse("abc"), None);
    }

    #[test]
    fn arithmetic_is_exact() {
        let tenth = Rational::parse("0.1").unwrap();
        let sum = tenth.checked_add(Rational::parse("0.2").unwrap()).unwrap();
        assert_eq!(sum, Rational::parse("0.3").unwrap());
    }
}
