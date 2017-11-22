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
    item self_arg { impl X { fn foo(self) {} } }
    item self_ref { impl X { fn foo(&self) {} } }
    item self_ref_lt { impl X { fn foo(&'a self) {} } }
    item self_mut { impl X { fn foo(mut self) {} } }
    item self_ref_mut { impl X { fn foo(&mut self) {} } }
    item self_ref_mut_lt { impl X { fn foo(&'a mut self) {} } }
    item path {
        impl<T> foo::Foo for Bar<T> where T: Baz<[T]> {}
    }
}

