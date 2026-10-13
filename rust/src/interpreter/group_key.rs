//! The key `UNIQUE`, `TALLY` and `GROUP` bundle by (LANG.VALUES.ORDER).
//!
//! Two elements fall in one group when they are one value, as `EQ` decides —
//! with the one exception SQL's `DISTINCT` and `GROUP BY` make too: every NIL
//! is the same key whatever its reason, at any depth. A null a source stored
//! and a key a record lacked are one group, where `EQ` tells them apart by
//! their reasons (LANG.VALUES.NIL). The group keeps the first NIL it met, so
//! the reason survives in the answer.
//!
//! The wrapper decides once, when it is built, whether a key holds a NIL at
//! all. One that does not — every scalar, text and NIL-free container, which
//! is nearly every key — hashes and compares exactly as `Value` does, so the
//! common case pays one `match` for the exception.

use crate::types::{Value, ValueData};
use std::hash::{Hash, Hasher};

/// A grouping key: a borrowed element, and whether a NIL lies anywhere in it.
pub(crate) struct GroupKey<'a> {
    value: &'a Value,
    holds_nil: bool,
}

impl<'a> GroupKey<'a> {
    pub(crate) fn of(value: &'a Value) -> Self {
        GroupKey {
            value,
            holds_nil: holds_nil(value),
        }
    }
}

impl PartialEq for GroupKey<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.holds_nil != other.holds_nil {
            return false;
        }
        if !self.holds_nil {
            return self.value == other.value;
        }
        same_group(self.value, other.value)
    }
}

impl Eq for GroupKey<'_> {}

impl Hash for GroupKey<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.holds_nil {
            hash_group(self.value, state);
        } else {
            self.value.hash(state);
        }
    }
}

/// Whether a NIL lies anywhere in `value`. A dense Tensor holds numbers and
/// nothing else, so it never does.
fn holds_nil(value: &Value) -> bool {
    match &value.data {
        ValueData::Nil(_) => true,
        ValueData::Vector(children) => children.iter().any(holds_nil),
        ValueData::Truths(column) => column.unknown().is_some(),
        ValueData::Record(record) => {
            record.keys().iter().any(holds_nil) || record.values().iter().any(holds_nil)
        }
        ValueData::Boolean(_)
        | ValueData::Scalar(_)
        | ValueData::ExactScalar(_)
        | ValueData::Tensor { .. }
        | ValueData::Symbol(_)
        | ValueData::Text(_) => false,
    }
}

/// The elements of a Vector held either way: boxed, or as a truth column.
fn sequence(value: &Value) -> Option<std::borrow::Cow<'_, [Value]>> {
    match &value.data {
        ValueData::Vector(children) => Some(std::borrow::Cow::Borrowed(children.as_slice())),
        ValueData::Truths(column) => Some(std::borrow::Cow::Owned(column.to_values())),
        _ => None,
    }
}

/// `EQ`, with every NIL one value. Reached only for a key that holds a NIL;
/// a NIL-free part of it compares as `Value` does.
fn same_group(a: &Value, b: &Value) -> bool {
    if a.is_nil() || b.is_nil() {
        return a.is_nil() && b.is_nil();
    }
    if let (Some(left), Some(right)) = (sequence(a), sequence(b)) {
        return left.len() == right.len()
            && left.iter().zip(right.iter()).all(|(x, y)| same_group(x, y));
    }
    if let (ValueData::Record(left), ValueData::Record(right)) = (&a.data, &b.data) {
        let pairwise = |x: &[Value], y: &[Value]| {
            x.len() == y.len() && x.iter().zip(y).all(|(p, q)| same_group(p, q))
        };
        return pairwise(left.keys(), right.keys()) && pairwise(left.values(), right.values());
    }
    a == b
}

const GROUP_TAG_NIL: u8 = 0xA0;
const GROUP_TAG_SEQUENCE: u8 = 0xA1;
const GROUP_TAG_RECORD: u8 = 0xA2;

/// The hash `same_group` agrees with: a NIL hashes as one tag, whatever its
/// reason, and a container holding one hashes its parts the same way. A
/// NIL-free part hashes as `Value` does, so it meets the equal part of
/// another key — a Tensor and its boxed Vector alike — in one bucket.
fn hash_group<H: Hasher>(value: &Value, state: &mut H) {
    if value.is_nil() {
        state.write_u8(GROUP_TAG_NIL);
        return;
    }
    if !holds_nil(value) {
        value.hash(state);
        return;
    }
    if let Some(children) = sequence(value) {
        state.write_u8(GROUP_TAG_SEQUENCE);
        state.write_usize(children.len());
        for child in children.iter() {
            hash_group(child, state);
        }
        return;
    }
    if let ValueData::Record(record) = &value.data {
        state.write_u8(GROUP_TAG_RECORD);
        state.write_usize(record.keys().len());
        for part in record.keys().iter().chain(record.values()) {
            hash_group(part, state);
        }
    }
}
