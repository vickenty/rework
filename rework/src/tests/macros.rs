parse! {
    item query {
        #[macro_export]
        macro_rules! query_op {
            (*) => ($crate::query::QueryOp::Any);
            (..) => ($crate::query::QueryOp::Many);
            ($kind:ident) => ($crate::query::QueryOp::Kind(stringify!($kind)));
            ([ = $len:expr ]) => ($crate::query::QueryOp::Size($len, $len + 1));
            ([ < $len:expr ]) => ($crate::query::QueryOp::Size(0, $len));
            ([ > $len:expr ]) => ($crate::query::QueryOp::More($len));
            (( $( $tt:tt )+ )) => ($crate::query::QueryOp::Query(query!($( $tt )+)));
            ({:is $kind:ident }) => ($crate::query::QueryOp::KindSelf(stringify!($kind)));
            ({:first}) => ($crate::query::QueryOp::Nth(0));
            ({:nth $e:expr}) => ($crate::query::QueryOp::Nth($e));
            ({:last}) => ($crate::query::QueryOp::Last);
            ({:prefix ~ $pat:expr}) => ($crate::query::QueryOp::PrefixPat($pat));
            ({:prefix = $pat:expr}) => ($crate::query::QueryOp::PrefixEq($pat));
            ({:text ~ $pat:expr}) => ($crate::query::QueryOp::TextPat($pat));
            ({:text = $pat:expr}) => ($crate::query::QueryOp::TextEq($pat));
        }
    }

    item space {
        macro_rules ! foo { () => {} }
    }

    item call_as_block {
        fn foo() {
            block! { foo, bar }
            baz
        }
    }

    item call_as_expr {
        fn foo() {
            println!("foo");
            bar
        }
    }
}

parse! {
    expr macro_lit { foo!("abc" 14) } => { (.. {:text="abc"}) (.. {:text="14"}) }
}
