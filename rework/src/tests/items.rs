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
}
