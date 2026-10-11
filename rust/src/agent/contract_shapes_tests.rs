//! `#:contract … shapes=…` (LANG.CONTRACT.FIELD): the zero shapes a body's
//! numbers may take from operands in the field, declared as a bound and held
//! to the stack walk's inference.

use crate::agent::api::check;
use serde_json::Value as Json;

fn contract_decls(source: &str) -> Json {
    check(source, true).to_json()["contractDecls"].clone()
}

#[test]
fn a_shapes_declaration_the_walk_proves_is_verified() {
    for (body, shapes) in [
        ("[ 0 MUL 0 DIV ]", "zero,nullity"),
        ("[ 'x' BIND 1 x DIV ]", "nonzero,infinite"),
        ("[ LENGTH 1 ADD DIV ]", "zero,nonzero"),
        ("[ STR ]", "none"),
        ("[ 2 MUL ]", "zero,nonzero,infinite"),
    ] {
        let source = format!("{body} 'W' DEF\n#:contract W shapes={shapes}");
        let decls = contract_decls(&source);
        assert_eq!(
            decls["outcome"], "value",
            "{body} declared shapes={shapes}: {decls}"
        );
    }
}

#[test]
fn a_body_reaching_a_shape_it_did_not_declare_is_violated() {
    let source = "[ 'x' BIND 1 x DIV ] 'W' DEF\n#:contract W shapes=zero,nonzero";
    let decls = contract_decls(source);
    assert_eq!(decls["outcome"], "error", "{decls}");
    let message = decls["findings"][0]["message"].as_str().unwrap();
    assert!(message.contains("shapes=nonzero,infinite"), "{message}");
}

#[test]
fn shapes_the_walk_could_not_infer_cannot_refute_a_declaration() {
    // `COLLECT` has variable arity, so the walk gives up and the widest set
    // the `field` join admits stands — which proves nothing.
    let source = "[ 1 2 2 COLLECT LENGTH ] 'W' DEF\n#:contract W shapes=nonzero";
    let decls = contract_decls(source);
    assert_ne!(decls["outcome"], "error", "{decls}");
}

#[test]
fn an_unknown_shape_is_a_parse_error() {
    let source = "[ 1 ] 'W' DEF\n#:contract W shapes=zero,huge";
    let decls = contract_decls(source);
    assert_eq!(decls["outcome"], "error");
    let findings = decls["findings"].as_array().expect("findings array");
    assert!(findings
        .iter()
        .any(|f| f["message"].as_str().unwrap().contains("`shapes`")));
}
