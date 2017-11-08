parse! {
    type_name simple { T }
    type_name array1 { [u8; 10] }
    type_name array2 { [u8; std::u8::MAX as usize] }
    type_name ref_named { &T }
    type_name ref_slice_u8 { &'a [u8] }
    type_name mut_slice_u8 { &mut [u8] }
    type_name mut_a_slice_u8 { &'a mut [u8] }
    type_name vec_u8 { Vec<u8> }
    type_name tuple_u8 { (u8, u8) }
    type_name raw_ptr { *const u8 }
    type_name raw_mut { * mut u8 }
    type_name path_args { ::std::collections::HashMap<String, std::vec::Vec<Box<String>>> }
    type_name cow { Cow<'a, str> }
    type_name qualified { <T as Deref>::Output }
    type_name compound {
        ::std::collections::HashMap <
            vec::Vec<( u32, &'static [ String ])>,
            &'a mut ::std::collections::HashSet < String >>
    }
    type_name type_fn1 { fn(u8) -> u32 }
    type_name type_fn2 { fn(x: u8) }
    type_name type_fn3 { for<'a> fn(x: &'a u8) }
    type_name type_fn4 { unsafe extern "C" fn() }
    type_name trait_obj1 { &(Copy + Send + 'foo) }
    type_name trait_obj2 { Copy + 'foo + Send }
    type_name trait_obj3 { Copy + }
    type_name trait_obj4 { Copy + 'foo + }
    type_name trait_obj_fn { &FnMut(&u8) }
    type_name impl_trait { impl Iterator<Item=Box<'x>> }
    type_name never_type { ! }
    type_name fn_trait1 { FnMut() }
    type_name fn_trait2 { FnOnce() -> u8 }
    type_name fn_trait3 { Fn((u8, u8)) }
    type_name fn_impl_bad { impl (Fn() -> impl Fn() -> u8 + 'static) + 'static }
    type_name for_lifetime { for<'a> Foo<'a> }
}

parse! {
    type_name tuple_unit { () } => { .. type_tuple empty }
    type_name tuple_one { (u8,) } => { .. type_tuple list [= 1] }
    type_name tuple_two { (u8, u8) } => { .. type_tuple list [= 2] }
    type_name grouped { (u8) } => { .. type_group }
}
