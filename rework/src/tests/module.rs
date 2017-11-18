parse! {
    module simple1 {
        #![feature(unstable)]
        #[macro_use]
        use std::io;
        use std::collections::{HashMap, HashSet};

        #[repr(C)]
        struct Set<K>(HashSet<K, Box<Inner>>);
        struct Inner {
            set: Set<::std::vec::Vec<u8>>,
            count: u8,
        }
        #[derive(Debug)]
        enum Value {
            Empty,
            Single(Inner),
            Double {
                first: Inner,
                second: Option<Inner>,
            }
        }
        #[inline]
        pub fn test() -> u8 {
            let foo: u8 = MAX;
            foo
        }
    }
}
