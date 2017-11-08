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
}
