#![no_std]
#![no_main]

use nanos_sdk::bindings::os_lib_end;

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);

use nanos_sdk::plugin::{
    PluginInteractionType, 
    PluginParam
};

use starknet_sdk::types::{
    Call, 
    AbstractCall,
    AbstractCallData, 
    FieldElement
};

use nanos_sdk::testing;
use nanos_sdk::string::String;

#[no_mangle]
extern "C" fn sample_main(arg0: u32) {
    let args: *mut u32 = arg0 as *mut u32;
    let value1 = unsafe { *args as u16 };
    let operation: PluginInteractionType = value1.into();

    match operation {
        PluginInteractionType::Feed => {
            testing::debug_print("Feed plugin Better MultiCall IN\n");

            let value2 = unsafe { *args.add(1) as *mut PluginParam };
            let params: &mut PluginParam = unsafe { &mut *value2 };

            let call: &Call = unsafe {&*(params.data_in as *const Call)};
            let abstract_call: &mut AbstractCall = unsafe {&mut *(params.data_out as *mut AbstractCall)};

            abstract_call.to.value = call.to.value;
            abstract_call.method.clone_from(&call.method);
            abstract_call.selector.value = call.selector.value;
            let mut i = 0;
            let mut j: usize = 0;

            while i < call.calldata_len {
                match call.calldata[i] {
                    FieldElement::ZERO => {
                        if i + 1 < call.calldata.len() {
                            abstract_call.calldata[j] = AbstractCallData::Felt(call.calldata[i + 1]);
                            i += 2;  // we just processed two elements
                            j += 1;
                        } else {
                            panic!("Invalid data: A 0-prefix felt should be followed by a value.");
                        }
                    },
                    FieldElement::ONE => {
                        if i + 2 < call.calldata.len() {
                            abstract_call.calldata[j] = AbstractCallData::CallRef(call.calldata[i + 1].into(), call.calldata[i + 2].into());
                            i += 3;  // we just processed three elements
                            j += 1;
                        } else {
                            panic!("Invalid data: A 1-prefix felt should be followed by two values.");
                        }
                    },
                    _ => {
                        testing::debug_print("Unknown prefix\n");
                        panic!("Unknown prefix for better-multicall.")
                    },
                }
            }
            abstract_call.calldata_len = j;
            testing::debug_print("Feed plugin Better MultiCall OUT \n");
        }
        _ => {
            testing::debug_print("Not implemented\n");
        }
    }
    unsafe {
        os_lib_end();
    }
}
