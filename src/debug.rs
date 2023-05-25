macro_rules! print_address {
    ($addr:expr) => {{
        use testing::debug_print;
        use nanos_sdk::{string, testing};
        debug_print("0x");
        let s = string::to_utf8::<64>(string::Value::ARR32(($addr).value));
        testing::debug_print(core::str::from_utf8(&s).unwrap());
    }};
}
