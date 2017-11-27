parse! {
    item struct_where {
        struct X<T> where T: Clone { x: T }
    }
    item enum_where {
        enum E<T> where T: Copy, { E(T) }
    }
    item type_where {
        type T<U> where U: Clone = Box<U>;
    }
    item mod_extern {
        mod foo;
    }
    item mod_ext_attr {
        #[path="../foo.rs"]
        mod foo;
    }
    item mod_intern {
        mod foo {
            struct X;
        }
    }
    item mod_int_attr {
        #[inline]
        mod foo {
            struct X;
        }
    }
    item enum_discr {
        enum E {
            A = 1,
            B = 2.5,
            C = "C",
        }
    }
    item enum_meta {
        enum A {
            #[ignore]
            A,
        }
    }
    item impl_for_ref {
        impl<'a, T> PartialEq<Interned<T>> for &'a Interned<T> {
        }
    }
    item extern_crate {
        #[macro_use]
        extern crate serde_derive;
    }
    item cfg_not_any {
        #[cfg(not(any(unix, windows)))]
        struct X;
    }
    item attr_kwargs {
        #[foo(foo=1, bar=2, baz)]
        struct X;
    }
    item extern_block {
        extern "C" {
            type Bar;
            fn foo();
            static FOO: u8;
        }
    }
    item let_no_val {
        fn foo() {
            let x;
        }
    }
    item static_var {
        static FOO: u8 = 1;
    }
    item struct_field_meta {
        struct X {
            #[serde(ignore)]
            pub x: u32,
        }
    }
    item tuple_struct_pub {
        struct X(pub u32, pub u32);
    }
}
