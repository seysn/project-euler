#[macro_export]
macro_rules! test_pb {
    ($func_name:ident, $expected:expr, $input:expr) => (
        #[cfg(test)]
        mod test {
            #[test]
            fn $func_name() {
                assert_eq!($expected, super::solve($input));
            }
        }
    )
}
