use parser;
use syntax;
use query::{find, QueryOp};

pub fn check_parses(
    input: &str,
    res: parser::ParseResult<syntax::Elem>,
    pat: &[QueryOp],
    pat_str: &str,
) {
    match res {
        Ok(mut e) => {
            assert_eq!(&format!("{}", e), input);
            if !pat.is_empty() {
                let mut matches = false;
                find(&mut e, pat, &mut |_| matches = true);
                if !matches {
                    println!("Does not match: {}", pat_str);
                    println!("Parsed as:\n{:#?}", e);
                    panic!("structure error");
                }
            }
        }
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
    ( $( $(#[$me:meta])* $type:ident $name:ident { $( $tt:tt )* })* ) => {
        $(
            #[test]
            $(#[$me])*
            fn $name() {
                let input = stringify!($($tt)*).trim_right();
                $crate::tests::check_parses(input, $crate::parser::$type(input, true), &[], "");
            }
        )*
    };
    ( $( $(#[$me:meta])* $type:ident $name:ident { $( $tt:tt )* } => { $( $pat:tt )* })* ) => {
        $(
            #[test]
            $(#[$me])*
            fn $name() {
                let input = stringify!($($tt)*).trim_right();
                assert_parses!($type input, { $( $pat )+ });
            }
        )*
    };
}

macro_rules! assert_parses {
    ( $type:ident $input:expr, { $( $pat:tt )+ } ) => (
        let query = query!($( $pat )*);
        let query_str = stringify!($( $pat )*);
        $crate::tests::check_parses($input, $crate::parser::$type($input, true), query, query_str);
    )
}

mod program;
mod type_name;
mod expr;
mod pat;
mod macros;
