//! Zero shapes as the contract inference reads them (LANG.VALUES.SHAPE,
//! LANG.CONTRACT.FIELD).
//!
//! A number's zero shape is whether its numerator and its denominator are
//! nonzero: `0`, a finite nonzero number, `±1/0` and `0/0`. The inference
//! carries a little more than the shape — the sign — because two Words decide
//! on it: `POW`, whose negative exponent turns a zero base into `1/0`, and the
//! sum, where two finite numbers of one sign never cancel. So an abstract value
//! is a set of six *atoms*, the four shapes with the finite nonzero and the
//! infinite ones split by sign, and every table here is the pair formula of
//! LANG.VALUES.EXACT read on those atoms.
//!
//! The set is over every number a value holds at any depth, so a Vector, a
//! Record and a scalar are read alike: an element-wise Word applies its table
//! to every pair of atoms its operands may hold, which covers every pair of
//! lanes it can meet.

use crate::coreword_registry::ZeroShapeLaw;
use crate::types::fraction::Fraction;
use crate::types::{Value, ValueData};
use std::cmp::Ordering;

/// A set of zero-shape atoms.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ZeroAtoms(u8);

/// The atoms in the order their bits are numbered.
const ATOMS: [ZeroAtoms; 6] = [
    ZeroAtoms::ZERO,
    ZeroAtoms::POS,
    ZeroAtoms::NEG,
    ZeroAtoms::POS_INF,
    ZeroAtoms::NEG_INF,
    ZeroAtoms::NULLITY,
];

/// The four shape names a contract reports, each with the atoms it covers.
const SHAPES: [(&str, ZeroAtoms); 4] = [
    ("zero", ZeroAtoms::ZERO),
    ("nonzero", ZeroAtoms::NONZERO),
    ("infinite", ZeroAtoms::INFINITE),
    ("nullity", ZeroAtoms::NULLITY),
];

impl ZeroAtoms {
    pub const NONE: Self = Self(0);
    pub const ZERO: Self = Self(1);
    pub const POS: Self = Self(1 << 1);
    pub const NEG: Self = Self(1 << 2);
    pub const POS_INF: Self = Self(1 << 3);
    pub const NEG_INF: Self = Self(1 << 4);
    pub const NULLITY: Self = Self(1 << 5);
    pub const NONZERO: Self = Self(Self::POS.0 | Self::NEG.0);
    pub const INFINITE: Self = Self(Self::POS_INF.0 | Self::NEG_INF.0);
    /// The numbers of the field: what a contract assumes its operands hold.
    pub const FINITE: Self = Self(Self::ZERO.0 | Self::NONZERO.0);
    /// The non-negative integers a count or an index answers.
    pub const NATURAL: Self = Self(Self::ZERO.0 | Self::POS.0);
    pub const ALL: Self = Self(0b11_1111);

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    pub const fn is_subset(self, other: Self) -> bool {
        self.0 & !other.0 == 0
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Whether every number the set admits lies in the field.
    pub const fn is_finite(self) -> bool {
        self.is_subset(Self::FINITE)
    }

    fn atoms(self) -> impl Iterator<Item = ZeroAtoms> {
        ATOMS.into_iter().filter(move |a| self.intersects(*a))
    }

    fn map(self, f: impl Fn(ZeroAtoms) -> ZeroAtoms) -> Self {
        self.atoms().fold(Self::NONE, |acc, a| acc.union(f(a)))
    }

    fn map2(self, other: Self, f: impl Fn(ZeroAtoms, ZeroAtoms) -> ZeroAtoms) -> Self {
        self.atoms().fold(Self::NONE, |acc, a| {
            other.atoms().fold(acc, |acc, b| acc.union(f(a, b)))
        })
    }

    /// The atom one number lies in.
    pub fn of_fraction(f: &Fraction) -> Self {
        let sign = if f.is_zero() {
            Ordering::Equal
        } else if f.is_positive() {
            Ordering::Greater
        } else {
            Ordering::Less
        };
        match (f.is_finite(), f.is_nullity(), sign) {
            (_, true, _) => Self::NULLITY,
            (true, _, Ordering::Equal) => Self::ZERO,
            (true, _, Ordering::Greater) => Self::POS,
            (true, _, Ordering::Less) => Self::NEG,
            (false, _, Ordering::Less) => Self::NEG_INF,
            (false, _, _) => Self::POS_INF,
        }
    }

    /// Every atom a value holds, at any depth.
    pub fn of_value(value: &Value) -> Self {
        match &value.data {
            ValueData::Scalar(f) => Self::of_fraction(f),
            ValueData::ExactScalar(real) => match real.signum() {
                Ordering::Less => Self::NEG,
                Ordering::Equal => Self::ZERO,
                Ordering::Greater => Self::POS,
            },
            ValueData::Vector(items) => items
                .iter()
                .fold(Self::NONE, |acc, v| acc.union(Self::of_value(v))),
            ValueData::Tensor { data, .. } => {
                let mut atoms = Self::NONE;
                for lane in data.iter() {
                    atoms = atoms.union(Self::of_fraction(&lane));
                    if atoms == Self::ALL {
                        break;
                    }
                }
                atoms
            }
            ValueData::Record(record) => record
                .keys()
                .iter()
                .chain(record.values())
                .fold(Self::NONE, |acc, v| acc.union(Self::of_value(v))),
            _ => Self::NONE,
        }
    }

    /// The shape names a contract reports, in the order of LANG.VALUES.SHAPE.
    pub fn shape_names(self) -> Vec<&'static str> {
        SHAPES
            .iter()
            .filter(|(_, atoms)| self.intersects(*atoms))
            .map(|(name, _)| *name)
            .collect()
    }

    /// The shapes as a `#:contract` declaration writes them: comma-separated,
    /// and `none` for a body that pushes no number.
    pub fn declared_spelling(self) -> String {
        match self.shape_names().as_slice() {
            [] => "none".to_string(),
            names => names.join(","),
        }
    }

    /// The atoms a declaration's term names: one shape, or `none`.
    pub fn from_shape_name(name: &str) -> Option<Self> {
        if name == "none" {
            return Some(Self::NONE);
        }
        SHAPES.iter().find(|(n, _)| *n == name).map(|(_, a)| *a)
    }

    /// Widened to whole shapes, so two sets compare as a contract states them.
    pub fn whole_shapes(self) -> Self {
        SHAPES
            .iter()
            .filter(|(_, atoms)| self.intersects(*atoms))
            .fold(Self::NONE, |acc, (_, atoms)| acc.union(*atoms))
    }
}

fn negated(a: ZeroAtoms) -> ZeroAtoms {
    match a {
        ZeroAtoms::POS => ZeroAtoms::NEG,
        ZeroAtoms::NEG => ZeroAtoms::POS,
        ZeroAtoms::POS_INF => ZeroAtoms::NEG_INF,
        ZeroAtoms::NEG_INF => ZeroAtoms::POS_INF,
        other => other,
    }
}

/// `1/x`: the pair swapped, the sign carried into the numerator.
fn reciprocal(a: ZeroAtoms) -> ZeroAtoms {
    match a {
        ZeroAtoms::ZERO => ZeroAtoms::POS_INF,
        ZeroAtoms::POS_INF | ZeroAtoms::NEG_INF => ZeroAtoms::ZERO,
        other => other,
    }
}

fn sum(a: ZeroAtoms, b: ZeroAtoms) -> ZeroAtoms {
    use ZeroAtoms as Z;
    match (a, b) {
        (Z::NULLITY, _) | (_, Z::NULLITY) => Z::NULLITY,
        (Z::ZERO, x) | (x, Z::ZERO) => x,
        // Two points over zero sum to `0/0`, whatever their signs.
        (x, y) if Z::INFINITE.intersects(x) && Z::INFINITE.intersects(y) => Z::NULLITY,
        (x, _) | (_, x) if Z::INFINITE.intersects(x) => x,
        (x, y) if x == y => x,
        // A positive and a negative number may cancel.
        _ => Z::FINITE,
    }
}

fn product(a: ZeroAtoms, b: ZeroAtoms) -> ZeroAtoms {
    use ZeroAtoms as Z;
    let negative = |x: ZeroAtoms| Z::NEG.union(Z::NEG_INF).intersects(x);
    match (a, b) {
        (Z::NULLITY, _) | (_, Z::NULLITY) => Z::NULLITY,
        (Z::ZERO, x) | (x, Z::ZERO) if Z::INFINITE.intersects(x) => Z::NULLITY,
        (Z::ZERO, _) | (_, Z::ZERO) => Z::ZERO,
        (x, y) => {
            let infinite = Z::INFINITE.intersects(x) || Z::INFINITE.intersects(y);
            match (infinite, negative(x) != negative(y)) {
                (false, false) => Z::POS,
                (false, true) => Z::NEG,
                (true, false) => Z::POS_INF,
                (true, true) => Z::NEG_INF,
            }
        }
    }
}

/// `base ^ exponent`. A finite nonzero exponent may be an integer (repeated
/// multiplication or division), a half-integer (a root, inside the field or
/// absent) or neither (absent); an infinite one is absent; `0/0` absorbs.
fn power(base: ZeroAtoms, exponent: ZeroAtoms) -> ZeroAtoms {
    use ZeroAtoms as Z;
    match (base, exponent) {
        (Z::NULLITY, _) | (_, Z::NULLITY) => Z::NULLITY,
        (_, Z::POS_INF | Z::NEG_INF) => Z::NONE,
        (_, Z::ZERO) => Z::POS,
        // A positive exponent keeps zero and infinity where they are; a
        // negative base may come out of either sign.
        (Z::ZERO, Z::POS) => Z::ZERO,
        (Z::POS | Z::POS_INF, Z::POS) => base,
        (Z::NEG, Z::POS) => Z::NONZERO,
        (Z::NEG_INF, Z::POS) => Z::INFINITE,
        // A negative exponent is the reciprocal of the positive power.
        (_, _) => power(base, Z::POS).map(reciprocal),
    }
}

fn root(a: ZeroAtoms) -> ZeroAtoms {
    match a {
        // A negative radicand is absent.
        ZeroAtoms::NEG | ZeroAtoms::NEG_INF => ZeroAtoms::NONE,
        other => other,
    }
}

/// `FLOOR` and `ROUND`: a finite nonzero number may round to zero; the three
/// points over zero stay where they are.
fn rounded(a: ZeroAtoms) -> ZeroAtoms {
    match a {
        ZeroAtoms::POS | ZeroAtoms::NEG => a.union(ZeroAtoms::ZERO),
        other => other,
    }
}

/// The atoms a Word's result may hold, by its law, from the atoms its operands
/// may hold (deepest first). A law that runs a block answers what the block
/// does, which the caller works out; here it carries its operands.
pub fn apply_law(law: ZeroShapeLaw, operands: &[ZeroAtoms]) -> ZeroAtoms {
    let all = operands
        .iter()
        .fold(ZeroAtoms::NONE, |acc, a| acc.union(*a));
    let unary =
        |f: fn(ZeroAtoms) -> ZeroAtoms| operands.first().map_or(ZeroAtoms::NONE, |a| a.map(f));
    let binary = |f: fn(ZeroAtoms, ZeroAtoms) -> ZeroAtoms, negate: bool| match operands {
        [a, b] => a.map2(if negate { b.map(negated) } else { *b }, f),
        _ => ZeroAtoms::NONE,
    };
    match law {
        ZeroShapeLaw::None => ZeroAtoms::NONE,
        ZeroShapeLaw::Carry
        | ZeroShapeLaw::Map
        | ZeroShapeLaw::Filter
        | ZeroShapeLaw::Fold
        | ZeroShapeLaw::Exec => all,
        ZeroShapeLaw::Count => ZeroAtoms::NATURAL,
        ZeroShapeLaw::CarryCount => all.union(ZeroAtoms::NATURAL),
        ZeroShapeLaw::Finite => ZeroAtoms::FINITE,
        ZeroShapeLaw::Parse => ZeroAtoms::ALL,
        ZeroShapeLaw::Sum => binary(sum, false),
        ZeroShapeLaw::Difference => binary(sum, true),
        ZeroShapeLaw::Product => binary(product, false),
        ZeroShapeLaw::Quotient => match operands {
            [a, b] => a.map2(b.map(reciprocal), product),
            _ => ZeroAtoms::NONE,
        },
        ZeroShapeLaw::Power => binary(power, false),
        ZeroShapeLaw::Root => unary(root),
        ZeroShapeLaw::Round => unary(rounded),
    }
}

/// The atoms a Core Word answers from operands that all lie in the field: the
/// `shapes` of its registered contract. `field` is this set's projection.
pub fn law_from_finite(law: ZeroShapeLaw, inputs: usize) -> ZeroAtoms {
    apply_law(law, &vec![ZeroAtoms::FINITE; inputs])
}

/// A Core Word's `shapes`: its `zeroShapes` law read over operands that lie
/// in the field.
pub(crate) fn core_word_shapes(name: &str) -> ZeroAtoms {
    crate::coreword_registry::get_declared_word(name).map_or(ZeroAtoms::ALL, |word| {
        let inputs = match word.stack_inputs {
            crate::kernel::generated::Arity::Fixed(n) => usize::from(n),
            crate::kernel::generated::Arity::Variable => 1,
        };
        law_from_finite(word.zero_shapes, inputs)
    })
}
