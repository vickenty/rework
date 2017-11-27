parse! {
    item basic {
        trait Foo<T> where T: Copy {
            type Foo: Debug;
            type Bar = T;
            const FOO: u32;
            fn foo();
            fn bar() {}
        }
    }
    item inherit {
        trait X: Debug {}
    }
    item self_arg {
        trait X {
            fn foo(&'a self, a: u32) {}
        }
    }
    item mut_param {
        trait X {
            fn foo(&self, mut a: u32) {}
        }
    }
    item unnamed_param {
        trait Y {
            fn foo(&'a u32, &'a u32) {}
        }
    }
    item item_attr {
        trait X {
            #[inline]
            fn foo();
        }
    }
}
