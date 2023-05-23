#![no_std]
#![no_main]

use nanos_sdk::bindings::os_lib_end;

nanos_sdk::set_panic!(nanos_sdk::exiting_panic);

use nanos_sdk::plugin::{
    PluginCheckParams, PluginCoreParams, PluginFeedParams, PluginFinalizeParams, PluginGetUiParams,
    PluginInitParams, PluginInteractionType, PluginQueryUiParams, PluginResult,
};

use nanos_sdk::{string, testing};

struct Selector {
    name: &'static str,
    value: [u8; 32],
}

struct Erc20Ctx {
    address: [u8; 32],
    method: &'static str,
    destination: [u8; 32],
    amount: [u8; 32],
    token_info_idx: Option<usize>,
}

const N_SELECTORS: usize = 2;

const METHODS: [&str; N_SELECTORS] = ["TRANSFER", "APPROVE"];
const SN_KECCAK: [[u8; 32]; N_SELECTORS] = [
    [
        0x00, 0x83, 0xaf, 0xd3, 0xf4, 0xca, 0xed, 0xc6, 0xee, 0xbf, 0x44, 0x24, 0x6f, 0xe5, 0x4e,
        0x38, 0xc9, 0x5e, 0x31, 0x79, 0xa5, 0xec, 0x9e, 0xa8, 0x17, 0x40, 0xec, 0xa5, 0xb4, 0x82,
        0xd1, 0x2e,
    ],
    [
        0x02, 0x19, 0x20, 0x9e, 0x08, 0x32, 0x75, 0x17, 0x17, 0x74, 0xda, 0xb1, 0xdf, 0x80, 0x98,
        0x2e, 0x9d, 0xf2, 0x09, 0x65, 0x16, 0xf0, 0x63, 0x19, 0xc5, 0xc6, 0xd7, 0x1a, 0xe0, 0xa8,
        0x48, 0x0c,
    ],
];

const SELECTORS: [Selector; N_SELECTORS] = [
    Selector {
        name: "transfer",
        value: [
            0x00, 0x83, 0xaf, 0xd3, 0xf4, 0xca, 0xed, 0xc6, 0xee, 0xbf, 0x44, 0x24, 0x6f, 0xe5,
            0x4e, 0x38, 0xc9, 0x5e, 0x31, 0x79, 0xa5, 0xec, 0x9e, 0xa8, 0x17, 0x40, 0xec, 0xa5,
            0xb4, 0x82, 0xd1, 0x2e,
        ],
    },
    Selector {
        name: "approve",
        value: [
            0x02, 0x19, 0x20, 0x9e, 0x08, 0x32, 0x75, 0x17, 0x17, 0x74, 0xda, 0xb1, 0xdf, 0x80,
            0x98, 0x2e, 0x9d, 0xf2, 0x09, 0x65, 0x16, 0xf0, 0x63, 0x19, 0xc5, 0xc6, 0xd7, 0x1a,
            0xe0, 0xa8, 0x48, 0x0c,
        ],
    },
];

mod context;
use context::Transaction;

#[no_mangle]
extern "C" fn sample_main(arg0: u32) {
    // to remove when PR https://github.com/LedgerHQ/ledger-nanos-sdk/pull/69 will be merged into SDK
    let selectors: [Selector; 2] = [
        Selector {
            name: METHODS[0],
            value: SN_KECCAK[0],
        },
        Selector {
            name: METHODS[1],
            value: SN_KECCAK[1],
        },
    ];

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
            let erc20_ctx: &mut Erc20Ctx =
                unsafe { &mut *(params.core_params.plugin_internal_ctx as *mut Erc20Ctx) };
            let tx_info: &Transaction =
                unsafe { &*(params.core_params.app_data as *const Transaction) };

            erc20_ctx.token_info_idx = None;
            for i in 0..2 {}
            params.num_ui_screens = 4;
            params.core_params.plugin_result = match erc20_ctx.token_info_idx {
                Some(idx) => {
                    testing::debug_print("token info found in plugin\n");
                    PluginResult::Ok
                }
                None => {
                    testing::debug_print("token info not found in plugin\n");
                    PluginResult::NeedInfo
                }
            };
        }
        PluginInteractionType::QueryUi => {
            testing::debug_print("QueryUI plugin\n");

            let value2 = unsafe { *args.add(1) as *mut PluginQueryUiParams };

            let params: &mut PluginQueryUiParams = unsafe { &mut *value2 };

            let title = "ERC-20 OPERATION".as_bytes();
            params.title[..title.len()].copy_from_slice(title);
            params.title_len = title.len();
            params.core_params.plugin_result = PluginResult::Ok;
        }
        PluginInteractionType::GetUi => {
            testing::debug_print("GetUI plugin\n");

            let value2 = unsafe { *args.add(1) as *mut PluginGetUiParams };

            let params: &mut PluginGetUiParams = unsafe { &mut *value2 };
            let erc20_ctx: &mut Erc20Ctx =
                unsafe { &mut *(params.core_params.plugin_internal_ctx as *mut Erc20Ctx) };

            testing::debug_print("requested screen index: ");
            let mut s = string::to_utf8::<2>(string::Value::U8(params.ui_screen_idx as u8));
            testing::debug_print(core::str::from_utf8(&s).unwrap());
            testing::debug_print("\n");

            let idx = erc20_ctx.token_info_idx.expect("unknown token");
            if (true) {
                params.core_params.plugin_result = PluginResult::Ok;
            } else {
                params.core_params.plugin_result = PluginResult::Err;
            }
        }
        _ => {
            testing::debug_print("Not implemented\n");
        }
    }

    unsafe {
        os_lib_end();
    }
}
