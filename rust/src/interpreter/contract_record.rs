//! A Word's contract as a Record — the one shape `CONTRACT` answers for a Core
//! Word, a User Word and a block (LANG.CONTRACT.REGISTRY): `inputs` and
//! `outputs`, each a count or `'variable'`, and `field`, `'closed'` or
//! `'leaving'`. A Core Word's is registered and a User Word's or a block's is
//! inferred, in the same keys and the same vocabulary, so the two compare.

use crate::interpreter::word_contract::{ContractFlow, WordContract};
use crate::types::{RecordData, Value};

pub(crate) fn contract_record(contract: &WordContract) -> Value {
    let (inputs, outputs) = match contract.flow {
        ContractFlow::Fixed { consumes, produces } => (
            Value::from_int(i64::from(consumes)),
            Value::from_int(i64::from(produces)),
        ),
        ContractFlow::Dynamic => (
            Value::from_string("variable"),
            Value::from_string("variable"),
        ),
    };
    let keys = ["inputs", "outputs", "field"]
        .map(Value::from_string)
        .to_vec();
    let values = vec![
        inputs,
        outputs,
        Value::from_string(contract.field.as_spec_str()),
    ];
    Value::from_record(RecordData::new(keys, values).expect("contract keys are distinct"))
}
