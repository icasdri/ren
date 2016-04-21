#[cfg(test)]
macro_rules! parameterized_tests {
    ( $func:path; $( $i:ident: $( $param:expr ),*  => $expected:expr ),* ) => {
        $(
            #[test]
            fn $i() {
                assert_eq!($func($( $param ),*), $expected);
            }
        )*
    }
}

