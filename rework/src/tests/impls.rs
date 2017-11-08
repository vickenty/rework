parse! {
    item empty { impl X { } }
    item impl_trait { impl X for Y {} }
    item trait_args { impl<'a, T> X<'a> for Y<T> {} }
    item content {
        impl X {
            fn foo() {}
            pub fn bar() {}
            const PI: Self::Pi = 3.314;
            pub type Pi = f32;
            default fn baz() {}
            unsafe extern "C" fn ook() {}
        }
    }
}

