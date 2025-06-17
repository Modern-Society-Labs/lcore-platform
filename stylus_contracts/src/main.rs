#![cfg_attr(not(feature = "export-abi"), no_main)]
#![cfg(feature = "export-abi")]

fn main() {
    mvp_iot_processor::print_from_args();
} 