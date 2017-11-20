parse! {
    item basic {
        trait Foo<T> where T: Copy {
            type Foo;
            type Bar = T;
            const FOO: u32;
            fn foo();
            fn bar() {}
        }
    }
    item inherit {
        trait X: Debug {}
    }
}