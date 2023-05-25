use crate::context::{FieldElement, Transaction};
use core::slice;
use core::str;
use heapless::Vec as HVec;
use heapless::String;
use nanos_sdk::string;

pub struct AbstractCall {
    pub contract_address: FieldElement,
    pub function_name: String<64>,
    pub parameters: HVec<String<64>, 10>,
}

pub fn parse_transaction_v1(tx: &Transaction) -> HVec<AbstractCall, 10> {
    let mut abstract_calls = HVec::new();
    let calls_len: usize = tx.calldata_v1.call_array_len.into();
    for call_id in 0..calls_len {
        let call = tx.calldata_v1.calls[call_id];

        let parameters = unsafe {
            let len: usize = call.call_data_len.into();
            let slice = slice::from_raw_parts(call.call_data.as_ptr(), len);
            let mut parameters = HVec::<String<64>, 10>::new();
            for item in slice {
                let string_item =
                    String::from(core::str::from_utf8(&string::to_utf8::<64>(string::Value::ARR32(item.value)))
                        .unwrap());
                let _ = parameters.push(string_item);
            }
            parameters
        };

        let function_name = String::from("todo");

        let abstract_call = AbstractCall {
            contract_address: call.to,
            function_name,
            parameters,
        };
        let _ = abstract_calls.push(abstract_call);
    }

    abstract_calls
}
