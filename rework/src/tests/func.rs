parse! {
    item fn_type_args {
        pub fn foo<'a, 'b: 'a, T: Iterator<Item=u8>>(t: &T) -> u8 {}
    }

    item stmt_like_expr {
        pub fn foo() {
            for foo in bar { }
            for bar in foo { }
        }
    }
}

parse! {
    item block_in_expr {
        fn foo() {
            // parses as (foo & if ... & bar)
            foo & if bar { } else { } & baz
        }
    } => {.. expr (.. {:text="foo"}) (.. expr_if) (.. {:text="baz"})}
    item block_before_expr {
        fn foo() {
            // parses as if ... ; &bar
            if foo { } else { } & baz
        }
    } => {.. block (.. stmt_expr expr_if) (expr .. refop) }
}
