parse! {
    program simple1 {
        use std::io;
        use std::collections::{HashMap, HashSet};
        struct Set<K>(HashSet<K, Box<Inner>>);
        struct Inner {
            set: Set<::std::vec::Vec<u8>>,
            count: u8,
        }
        enum Value {
            Empty,
            Single(Inner),
            Double {
                first: Inner,
                second: Option<Inner>,
            }
        }
        pub fn test() -> u8 {
            let foo: u8 = MAX;
            foo
        }
    }
}
