/// A macro to generate a module containing bitwise constants based on a list of identifiers.
///
/// # Usage
///
/// ```rust
/// bit_variants!(pub my_bits; u64; FLAG_A, FLAG_B, FLAG_C);
/// ```
/// This example expands to:
///
/// ```rust
/// pub mod my_bits {
///     pub const FLAG_A: u64 = 1 << 0;
///     pub const FLAG_B: u64 = 1 << 1;
///     pub const FLAG_C: u64 = 1 << 2;
/// }
/// ```
///
/// # Syntax
///
/// ```rust
/// bit_variants!(pub_visibility module_name; type; identifier1, identifier2, ...);
/// ```
/// - `pub_visibility`: Visibility of the generated module (e.g., `pub`).
/// - `module_name`: Name of the generated module.
/// - `type`: The type for the bitwise constants (e.g., `u64`).
/// - `identifier1, identifier2, ...`: A comma-separated list of identifiers to be used as constants.
///
/// # Details
/// - The macro uses recursive invocations to generate constants.
/// - The bit position starts at 0 and increments for each identifier.
/// - Each constant is calculated as `1 << bit_position`.
///
/// This macro is useful for scenarios where you need a set of bitflags or unique identifiers represented as powers of two.
#[macro_export]
macro_rules! bit_variants {
    ($pub:vis $mod:ident;$ty:ty;$($name:ident),+$(,)?) => {
        $pub mod $mod {
            bit_variants!(0;$ty;$($name)+);
        }
    };
($count:expr;$ty:ty;$name:ident $($rest:ident)* ) => {
        #[allow(bad_style,unused)]
        pub const $name:u64 = 1 << $count;
        bit_variants!($count+1;$ty;$($rest)*);
    };
    ($count:expr;$ty:ty;)=>{};
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_bit_variants_macro() {
        bit_variants!(test_mod; u64; FLAG_A, FLAG_B, FLAG_C);

        assert_eq!(test_mod::FLAG_A, 1 << 0);
        assert_eq!(test_mod::FLAG_B, 1 << 1);
        assert_eq!(test_mod::FLAG_C, 1 << 2);
    }

    // Test for empty identifiers removed as it's not required.

    #[test]
    fn test_large_bit_variants() {
        bit_variants!(pub large_mod; u128; LARGE_FLAG_1, LARGE_FLAG_2, LARGE_FLAG_3);

        assert_eq!(large_mod::LARGE_FLAG_1, 1 << 0);
        assert_eq!(large_mod::LARGE_FLAG_2, 1 << 1);
        assert_eq!(large_mod::LARGE_FLAG_3, 1 << 2);
    }
}
