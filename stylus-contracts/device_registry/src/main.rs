#![cfg_attr(not(feature = "export-abi"), no_main)]

#[cfg(feature = "export-abi")]
fn main() {
    // ABI export handled by cargo stylus; no-op.
}

#[cfg(not(feature = "export-abi"))]
fn main() {} 