//! The order of the exact scalars (LANG.VALUES.EXACT): the field is totally
//! ordered, and the two points over zero are ordered against nothing. `1/0`
//! is the one unsigned point at infinity — it lies neither above nor below
//! any number — and `0/0` is the wheel's bottom. `order` answers `None`
//! exactly when either operand is a point, and every Word that asks for an
//! order projects `domainMiss` on that answer.

use super::fraction::Fraction;
use num_bigint::BigInt;

impl Fraction {
    /// The order of two numbers, when both lie in the field: a point over
    /// zero is ordered against nothing, not even itself — the one comparison
    /// the exact domain does not decide (LANG.VALUES.EXACT).
    pub fn order(&self, other: &Fraction) -> Option<std::cmp::Ordering> {
        if !self.is_finite() || !other.is_finite() {
            return None;
        }
        Some(self.cmp_finite(other))
    }

    /// The order of two rationals. Both operands must be finite.
    pub(crate) fn cmp_finite(&self, other: &Fraction) -> std::cmp::Ordering {
        debug_assert!(self.is_finite() && other.is_finite());
        if let (Some((a, b)), Some((c, d))) = (self.extract_i64_pair(), other.extract_i64_pair()) {
            if b == d {
                return a.cmp(&c);
            }
            let lhs = (a as i128) * (d as i128);
            let rhs = (c as i128) * (b as i128);
            return lhs.cmp(&rhs);
        }
        let (an, ad): (BigInt, BigInt) = self.to_bigint_pair();
        let (bn, bd): (BigInt, BigInt) = other.to_bigint_pair();
        if ad == bd {
            return an.cmp(&bn);
        }
        let lhs: BigInt = an * &bd;
        let rhs: BigInt = bn * &ad;
        lhs.cmp(&rhs)
    }
}

impl PartialOrd for Fraction {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.order(other)
    }
}
