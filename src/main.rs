#![no_std]
#![no_main]

use context::Transaction;
use nanos_sdk::bindings::os_lib_end;

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);

use nanos_sdk::plugin::{
    PluginFeedParams, PluginFinalizeParams, PluginGetUiParams, PluginInitParams,
    PluginInteractionType, PluginQueryUiParams, PluginResult,
};

use nanos_sdk::testing;

mod context;
use testing::debug_print;
#[macro_use]
mod debug;
mod parser;

use crate::parser::parse_transaction_v1;

#[no_mangle]
extern "C" fn sample_main(arg0: u32) {
    let args: *mut u32 = arg0 as *mut u32;
    let value1 = unsafe { *args as u16 };
    let operation: PluginInteractionType = value1.into();

    match operation {
        PluginInteractionType::Check => {
            testing::debug_print("Check plugin presence\n");
        }
        PluginInteractionType::Init => {
            testing::debug_print("Init plugin context\n");
            let value2 = unsafe { *args.add(1) as *mut PluginInitParams };
            let params: &mut PluginInitParams = unsafe { &mut *value2 };
            let tx_info: &Transaction =
                unsafe { &*(params.core_params.app_data as *const Transaction) };

            let calls = parse_transaction_v1(tx_info);
            for call in calls {
                debug_print("- CALL | ");
                debug_print(&call.function_name);
                debug_print(": [ ");
                for param in call.parameters {
                    debug_print(&param);
                    debug_print(", ");
                }
                debug_print("]\n");
            }

            params.core_params.plugin_result = PluginResult::Ok;
        }
        PluginInteractionType::Feed => {
            testing::debug_print("Feed plugin\n");
            let value2 = unsafe { *args.add(1) as *mut PluginFeedParams };
            let params: &mut PluginFeedParams = unsafe { &mut *value2 };

            params.core_params.plugin_result = PluginResult::Ok;
        }
        PluginInteractionType::Finalize => {
            testing::debug_print("Finalize plugin\n");
            let value2 = unsafe { *args.add(1) as *mut PluginFinalizeParams };
            let params: &mut PluginFinalizeParams = unsafe { &mut *value2 };

            params.core_params.plugin_result = PluginResult::Ok;
        }
        PluginInteractionType::QueryUi => {
            testing::debug_print("QueryUI plugin\n");
            let value2 = unsafe { *args.add(1) as *mut PluginQueryUiParams };
            let params: &mut PluginQueryUiParams = unsafe { &mut *value2 };

            params.core_params.plugin_result = PluginResult::Ok;
        }
        PluginInteractionType::GetUi => {
            testing::debug_print("GetUI plugin\n");
            let value2 = unsafe { *args.add(1) as *mut PluginGetUiParams };
            let params: &mut PluginGetUiParams = unsafe { &mut *value2 };

            params.core_params.plugin_result = PluginResult::Ok;
        }
        _ => {
            testing::debug_print("Not implemented\n");
        }
    }

    unsafe {
        os_lib_end();
    }
}
