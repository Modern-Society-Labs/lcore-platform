#![cfg_attr(not(feature = "export-abi"), no_main)]

#[cfg(feature = "export-abi")]
fn main() {
    // Call no-op to trigger cargo stylus ABI generation for the crate.
    // The actual logic lives in src/lib.rs.
}

#[cfg(not(feature = "export-abi"))]
fn main() {} 