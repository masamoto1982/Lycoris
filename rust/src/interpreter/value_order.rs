//! The one order the ordering Words share (LANG.VALUES.ORDER): `SORT` and
//! `ORDER` through [`order_indices`], `BSEARCH` through [`compare_for_sort`].
//! Numbers order by the exact field, Strings by Unicode scalar value, and a
//! NIL element is set aside and placed last rather than compared.

use crate::error::{AjisaiError, Result};
use crate::interpreter::comparison::three_way_compare;
use crate::types::{Value, ValueData};

/// The ascending, stable index permutation of `items` — `ORDER`'s answer, and
/// the permutation `SORT` applies to produce its own.
///
/// Shared so the two Words cannot disagree about an ordering: `xs ORDER` and
/// `xs SORT` are the same comparison sequence, read two ways. The order is
/// LANG.VALUES.ORDER's: numbers by the exact field, Strings by Unicode scalar
/// value, and every NIL after every present element, in its original place
/// among the NILs — SQL's `NULLS LAST`, so nothing is dropped and a payload
/// permuted by the answer stays aligned. One pair is undecided: `0/0` and
/// `1/0` have no order, so a Vector holding either has no sorted form, and
/// the answer is `None` — the projection `SORT` and `ORDER` make for it.
/// `Err(_)` is malformed use: an element with no order at all, or numbers
/// and Strings side by side (LANG.FAILURE.ERROR).
pub(crate) fn order_indices(items: &[Value]) -> Result<Option<Vec<usize>>> {
    // Whether a pair orders depends only on the domain of each element and on
    // whether a number is a point over zero, so one pass finds the first
    // malformed element, a mix of the two ordered domains, and the first point
    // before the sort runs. The sort's comparator must be a total order — the
    // standard sort panics on one that reports `Equal` for a pair it could not
    // order — and after this pass it cannot fail.
    let mut domain: Option<OrderedDomain> = None;
    let mut absent = 0usize;
    for item in items {
        if item.is_nil() {
            absent += 1;
            continue;
        }
        let this = OrderedDomain::of(item)?;
        match domain {
            None => domain = Some(this),
            Some(seen) if seen != this => return Err(mixed_domains(item)),
            Some(_) => {}
        }
        if this == OrderedDomain::Number && compare_for_sort(item, item)?.is_none() {
            return Ok(None);
        }
    }

    // The present elements, in source order: the NILs are set aside rather
    // than compared, so the comparator only ever sees one ordered domain.
    let mut perm: Vec<usize> = Vec::with_capacity(items.len());
    let mut nils: Vec<usize> = Vec::with_capacity(absent);
    for (index, item) in items.iter().enumerate() {
        if item.is_nil() {
            nils.push(index);
        } else {
            perm.push(index);
        }
    }
    match domain {
        // Strings compare by their bytes, which is their order; no `Value`
        // dispatch per probe.
        Some(OrderedDomain::Text) => perm.sort_by(|&i, &j| {
            let (Some(a), Some(b)) = (items[i].as_text(), items[j].as_text()) else {
                unreachable!("the pass above admitted only Strings");
            };
            crate::interpreter::comparison::text_order(a, b)
        }),
        _ => perm.sort_by(|&i, &j| {
            compare_for_sort(&items[i], &items[j])
                .ok()
                .flatten()
                .unwrap_or(std::cmp::Ordering::Equal)
        }),
    }
    perm.extend(nils);
    Ok(Some(perm))
}

/// The two domains an order is defined on (LANG.VALUES.ORDER).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OrderedDomain {
    Number,
    Text,
}

impl OrderedDomain {
    /// The ordered domain `value` lies in, or the `nonNumeric` an order Word
    /// raises for anything else. A NIL is not asked: callers set it aside.
    pub(crate) fn of(value: &Value) -> Result<Self> {
        match &value.data {
            ValueData::Scalar(_) | ValueData::ExactScalar(_) => Ok(OrderedDomain::Number),
            ValueData::Text(_) => Ok(OrderedDomain::Text),
            _ => Err(AjisaiError::declared(
                "nonNumeric",
                format!(
                    "expected Scalar or String elements, got {}",
                    value.domain_name()
                ),
            )),
        }
    }
}

/// Numbers and Strings side by side have no order between them: the domains
/// are disjoint (LANG.VALUES.DISJOINT).
pub(crate) fn mixed_domains(item: &Value) -> AjisaiError {
    let other = match OrderedDomain::of(item) {
        Ok(OrderedDomain::Text) => "Scalar",
        _ => "String",
    };
    AjisaiError::declared(
        "nonNumeric",
        format!(
            "expected elements of one ordered domain, got {} beside a {other}",
            item.domain_name()
        ),
    )
}

/// `three_way_compare`, with a pair the order is not defined on raised as
/// `nonNumeric`: the order is an order of numbers and of Strings
/// (LANG.VALUES.ORDER), and every Word that asks for one (LT/GT, MIN/MAX,
/// SORT/ORDER/BSEARCH) names the fault the same way. `None` for a pair
/// holding a point over zero, which the Word projects.
pub(crate) fn compare_for_sort(a: &Value, b: &Value) -> Result<Option<std::cmp::Ordering>> {
    three_way_compare(a, b).map_err(|e| {
        AjisaiError::declared(
            "nonNumeric",
            format!("expected Scalar or String elements, got {}", e.got),
        )
    })
}

