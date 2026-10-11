//! LANG.CONTRACT.FIELD read per zero shape, held against the implementation.
//!
//! The contract inference carries the zero shapes each value may take through
//! a body (`interpreter::word_contract_shape`), asking every Core Word what it
//! answers by the `zeroShapes` law `spec/words.json` declares for it. Three
//! laws keep that honest, each by running programs:
//!
//! 1. every Core Word's law contains what the Word answers, over operands of
//!    every zero shape and sign;
//! 2. `field` is the law's projection;
//! 3. the shapes inferred for a whole body — blocks, bindings, folds and User
//!    Words — contain what running it over finite operands answers.

use crate::interpreter::Interpreter;
use crate::types::{Value, ValueData};

async fn run(code: &str) -> Vec<Value> {
    let mut interp = Interpreter::new();
    interp
        .execute(code)
        .await
        .unwrap_or_else(|e| panic!("`{code}` must run: {e:?}"));
    interp.get_stack().to_vec()
}

/// Operands of every zero shape and sign, inside and outside Vectors, beside
/// values that hold no number.
const LAW_POOL: &[&str] = &[
    "0",
    "3",
    "-2",
    "1/2",
    "2 SQRT",
    "0 2 SQRT SUB",
    "1/0",
    "-1/0",
    "0/0",
    "[ 0 3 -2 ]",
    "[ 2 3 ]",
    "[ 1/0 0/0 ]",
    "'a'",
    "'12'",
    "TRUE",
];

/// Every Core Word's `zeroShapes` law (`spec/words.json`) is sound: run over
/// every combination of operands from the pool, each number it answers lies
/// in an atom the law predicts from its operands' atoms. The laws that run a
/// block are the stack walk's to apply and are held by the contract tests.
#[tokio::test]
async fn every_core_words_zero_shape_law_contains_what_it_answers() {
    use crate::coreword_registry::ZeroShapeLaw;
    use crate::interpreter::zero_shape::{apply_law, ZeroAtoms};
    use crate::kernel::generated::{Arity, GENERATED_WORDS};

    let mut pool = Vec::new();
    for operand in LAW_POOL {
        pool.push((
            *operand,
            ZeroAtoms::of_value(&number_or_value(operand).await),
        ));
    }
    let mut checked = 0;
    for word in GENERATED_WORDS {
        let (Arity::Fixed(inputs), Arity::Fixed(outputs)) = (word.stack_inputs, word.stack_outputs)
        else {
            continue;
        };
        if outputs == 0
            || matches!(
                word.zero_shapes,
                ZeroShapeLaw::Map | ZeroShapeLaw::Filter | ZeroShapeLaw::Fold | ZeroShapeLaw::Exec
            )
        {
            continue;
        }
        let inputs = usize::from(inputs);
        let combos = pool.len().pow(inputs as u32);
        for mut n in 0..combos {
            let mut code = String::new();
            let mut atoms = Vec::new();
            for _ in 0..inputs {
                let (operand, a) = pool[n % pool.len()];
                n /= pool.len();
                code.push_str(operand);
                code.push(' ');
                atoms.push(a);
            }
            code.push_str(word.name);
            let mut interp = Interpreter::new();
            if interp.execute(&code).await.is_err() {
                continue;
            }
            let predicted = apply_law(word.zero_shapes, &atoms);
            for value in interp.get_stack() {
                let answered = ZeroAtoms::of_value(value);
                assert!(
                    answered.is_subset(predicted),
                    "`{code}` answered {answered:?}, outside what {} predicts: {predicted:?}",
                    word.zero_shapes.as_spec_str()
                );
            }
            checked += 1;
        }
    }
    assert!(checked > 1000, "the law check ran {checked} programs");
}

/// `field` is the projection of `zeroShapes`: a Core Word is `leaving` exactly
/// when its law can answer a point over zero from operands in the field.
#[test]
fn field_is_the_projection_of_the_zero_shape_law() {
    use crate::coreword_registry::FieldClosure;
    use crate::interpreter::word_contract::core_word_shapes;
    use crate::kernel::generated::GENERATED_WORDS;

    for word in GENERATED_WORDS {
        let leaves = !core_word_shapes(word.name).is_finite();
        assert_eq!(
            word.field == FieldClosure::Leaving,
            leaves,
            "{}: `field` and `zeroShapes` must agree",
            word.name
        );
    }
}

async fn number_or_value(code: &str) -> Value {
    let stack = run(code).await;
    stack
        .into_iter()
        .next()
        .expect("each pool operand pushes one value")
}

/// The stack walk is sound: for bodies built from blocks, bindings, folds and
/// User Words, every number running the body over finite operands answers
/// lies in a shape `CONTRACT` inferred for it.
#[tokio::test]
async fn an_inferred_shape_set_contains_what_the_body_answers() {
    use crate::interpreter::zero_shape::ZeroAtoms;

    let finite = ["0", "3", "-2", "1/2", "2 SQRT", "[ 0 1 -1 ]", "[ ]"];
    let bodies = [
        "[ 0 DIV ]",
        "[ 0 MUL 0 DIV ]",
        "[ 'x' BIND 1 x DIV ]",
        "[ LENGTH 1 ADD DIV ]",
        "[ [ 0 DIV ] MAP ]",
        "[ [ 2 MUL 1 ADD ] MAP ]",
        "[ 0 [ ADD ] FOLD ]",
        "[ 1 [ DIV ] SCAN ]",
        "[ [ 'x' BIND 1 x DIV ] MAP ]",
        "[ [ 0 GT ] FILTER ]",
        "[ -1 POW FLOOR ]",
        "[ 1/0 MUL ]",
        "[ 2 SQRT 1 SUB 0 MAX DIV ]",
    ];
    let definitions = "[ 0 DIV ] 'AWAY' DEF [ AWAY 1 ADD ] 'BACK' DEF ";
    let mut checked = 0;
    for body in bodies
        .iter()
        .copied()
        .chain(["[ BACK ]", "[ AWAY AWAY MUL ]"])
    {
        let Ok(stack) = run_quiet(&format!("{definitions}{body} CONTRACT 'shapes' GET")).await
        else {
            continue;
        };
        let inferred = shape_set(&stack[0]);
        for operand in finite {
            let code = format!("{definitions}{operand} {body} EXEC");
            let Ok(stack) = run_quiet(&code).await else {
                continue;
            };
            for value in &stack {
                let answered = ZeroAtoms::of_value(value).whole_shapes();
                assert!(
                    answered.is_subset(inferred),
                    "`{code}` answered {answered:?}, outside the inferred {inferred:?}"
                );
            }
            checked += 1;
        }
    }
    assert!(checked > 40, "the walk check ran {checked} programs");
}

fn shape_set(names: &Value) -> crate::interpreter::zero_shape::ZeroAtoms {
    use crate::interpreter::zero_shape::ZeroAtoms;
    let ValueData::Vector(items) = &names.data else {
        panic!("shapes is a Vector, got {names:?}");
    };
    items.iter().fold(ZeroAtoms::NONE, |acc, item| {
        let ValueData::Text(name) = &item.data else {
            panic!("a shape is a String");
        };
        acc.union(ZeroAtoms::from_shape_name(name).expect("a registered shape name"))
    })
}

async fn run_quiet(code: &str) -> Result<Vec<Value>, ()> {
    let mut interp = Interpreter::new();
    interp.execute(code).await.map_err(|_| ())?;
    Ok(interp.get_stack().to_vec())
}

/// Every Core Word carries the `zeroShapes` law `spec/words.json` declares,
/// spelled as the registry spells it, and every law the schema admits is
/// some Word's — so no variant of the generated vocabulary is dead.
#[test]
fn every_word_carries_the_zero_shape_law_the_registry_declares() {
    use crate::kernel::generated::GENERATED_WORDS;
    use std::collections::BTreeSet;

    let registry: serde_json::Value =
        serde_json::from_str(include_str!("../../spec/words.json")).expect("words.json parses");
    let declared = registry["entries"].as_array().expect("entries");
    let mut laws = BTreeSet::new();
    for word in GENERATED_WORDS {
        let entry = declared
            .iter()
            .find(|e| e["name"] == word.name)
            .unwrap_or_else(|| panic!("{} is declared", word.name));
        assert_eq!(
            word.zero_shapes.as_spec_str(),
            entry["zeroShapes"],
            "{}: generated law and registry law",
            word.name
        );
        laws.insert(word.zero_shapes.as_spec_str());
    }
    let schema: serde_json::Value =
        serde_json::from_str(include_str!("../../spec/words.schema.json"))
            .expect("words.schema.json parses");
    let admitted: BTreeSet<&str> = schema["$defs"]["word"]["properties"]["zeroShapes"]["enum"]
        .as_array()
        .expect("zeroShapes enum")
        .iter()
        .filter_map(|v| v.as_str())
        .collect();
    assert_eq!(laws, admitted, "every admitted law is some Word's");
}
