use alloy_sol_types::sol;

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        uint32 n;
        uint256 a;
        uint256 b;
    }
}

// bring alloys gen u256 type into scope so that we can make n=100
// also, under the hood sol! macro uses this type for uint256 fields
use alloy_sol_types::private::primitives::aliases::U256;

/// Compute the n'th fibonacci number (wrapping around on overflows), using normal Rust code.
pub fn fibonacci(n: u32) -> (U256, U256) {
    let mut a = U256::from(0);
    let mut b = U256::from(1);

    for _ in 0..n {
        // originally we had to use a wrapping add, however alloysU256 actually implementes
        // overflowing nativley via the standard op overlaoding, we can use +!
        let c = a + b;
        a = b;
        b = c;
    }
    (a, b)
}
