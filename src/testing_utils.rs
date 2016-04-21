#[cfg(test)]
macro_rules! parameterized {
    ( $func:path; $( $i:ident: $( $param:expr ),*  => $expected:expr )* ) => {
        $(
            #[test]
            fn $i() {
                $func($( $param ),*, $expected);
            }
        )*
    }
}

