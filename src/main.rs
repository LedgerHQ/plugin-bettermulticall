#![no_std]
#![no_main]

//use context::Transaction;
use nanos_sdk::bindings::os_lib_end;

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);

use nanos_sdk::plugin::{
    PluginInteractionType, 
    PluginFeedParams
};

use nanos_sdk::starknet::{
    Call, 
    AbstractCall,
    AbstractCallData, FieldElement
};

use nanos_sdk::testing;

//mod context;
//#[macro_use]
//mod debug;
//mod parser;
use heapless::{ String };

//use crate::parser::parse_transaction_v1;

#[no_mangle]
extern "C" fn sample_main(arg0: u32) {
    let args: *mut u32 = arg0 as *mut u32;
    let value1 = unsafe { *args as u16 };
    let operation: PluginInteractionType = value1.into();

    match operation {
        PluginInteractionType::Feed => {
            testing::debug_print("Check plugin Better MultiCall IN\n");

            let value2 = unsafe { *args.add(1) as *mut PluginFeedParams };
            let params: &mut PluginFeedParams = unsafe { &mut *value2 };

            let call: &Call = unsafe {&*(params.data_in[0] as *const Call)};

            let abstract_call: &mut AbstractCall = unsafe {&mut *(params.data_out[0] as *mut AbstractCall)};
            let call_to_nref: &mut [u8; 256] = unsafe {&mut *(params.data_out[1] as *mut [u8; 256])};

            abstract_call.to.value = call.to.value;
            abstract_call.method = String::from("transfer");
            abstract_call.selector.value = call.selector.value;
            let mut i = 0;
            while i < call.calldata.len() {
                match call.calldata[i] {
                    FieldElement::ZERO => {
                        if i + 1 < call.calldata.len() {
                            abstract_call.calldata.push(AbstractCallData::Felt(call.calldata[i + 1])).unwrap();
                            i += 2;  // we just processed two elements
                        } else {
                            panic!("Invalid data: A 0-prefix felt should be followed by a value.");
                        }
                    },
                    FieldElement::ONE => {
                        if i + 1 < call.calldata.len() {
                            abstract_call.calldata.push(AbstractCallData::Ref(call.calldata[i + 1].into())).unwrap();
                            i += 2; 
                        } else {
                            panic!("Invalid data: A 1-prefix felt should be followed by a value.");
                        }
                    },
                    FieldElement::TWO => {
                        if i + 2 < call.calldata.len() {
                            abstract_call.calldata.push(AbstractCallData::CallRef(call.calldata[i + 1].into(), call.calldata[i + 2].into())).unwrap();
                            i += 3;  // we just processed three elements
                        } else {
                            panic!("Invalid data: A 2-prefix felt should be followed by two values.");
                        }
                    },
                    _ => panic!("Unknown prefix for better-multicall."),
                }
            }

            call_to_nref[0] = 0xDE;call_to_nref[1] = 0xAD;
            call_to_nref[2] = 0xBE;call_to_nref[3] = 0xEF;

            testing::debug_print("Check plugin Better MultiCall OUT \n");

        }
        _ => {
            testing::debug_print("Not implemented\n");
        }
    }
    unsafe {
        os_lib_end();
    }
}
