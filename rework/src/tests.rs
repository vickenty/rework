use parser;
use syntax;

pub fn check_parses(input: &str, res: parser::ParseResult<syntax::Elem>) {
    match res {
        Ok(e) => assert_eq!(&format!("{}", e), input),
        Err(e) => {
            let frag: String = (&input[e.offset..]).chars().take(40).collect();
            println!("");
            println!(
                "Parse error at {}: expected {:?} before `{}`",
                e.offset,
                e.expected,
                frag
            );
            println!("Full input:\n{}", input);
            panic!("parse error");
        }
    }
}

macro_rules! parse {
    ( $( $type:ident $name:ident { $( $tt:tt )* })* ) => {
        $(
            #[test]
            fn $name() {
                let input = stringify!($($tt)*).trim_right();
                check_parses(input, parser::$type(input));
            }
        )*
    };
}

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
    }

    type_name ref_slice_u8 { &'a [u8] }
    type_name mut_slice_u8 { &mut [u8] }
    type_name mut_a_slice_u8 { &'a mut [u8] }
    type_name vec_u8 { Vec<u8> }
    type_name tuple_u8 { (u8, u8) }
    type_name raw_ptr { *const u8 }
    type_name raw_mut { * mut u8 }
    type_name path_args { ::std::collections::HashMap<String, std::vec::Vec<Box<String>>> }
    type_name cow { Cow<'a, str> }
    type_name qualified { <T as Deref>::Output }
    type_name compound {
        ::std::collections::HashMap <
            vec::Vec<( u32, &'static [ String ])>,
            &'a mut ::std::collections::HashSet < String >>
    }
    type_name type_fn1 { fn(u8) -> u32 }
    type_name type_fn2 { fn(x: u8) }
    type_name type_fn3 { for<'a> fn(x: &'a u8) }
    type_name type_fn4 { unsafe extern "C" fn() }
    type_name trait_obj { &(Copy + Send + 'foo) }
    type_name impl_trait { impl Iterator<Item=Box<'x>> }
    type_name never_type { ! }
}
