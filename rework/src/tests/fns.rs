parse! {
    item fn_type_args {
        pub fn foo<'a, 'b: 'a, T: Iterator<Item=u8>>(t: &T) -> u8 {}
    }
    item fn_where {
        pub fn foo<'a, T>(t: T) -> T
        where
            T: Iterator + 'a,
            <T as Iterator>::Item: Clone + Debug,
            'a: 'static,
            Box<T>: Clone,
        {}
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
