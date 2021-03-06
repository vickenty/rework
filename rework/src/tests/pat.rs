parse! {
    pattern const_int { 1 }
    pattern const_str { "foo" }

    pattern bind_var { x }
    pattern bind_mut { mut y }
    pattern bind_ref { ref z }
    pattern bind_ref_mut { ref mut foo }
    pattern bind_at { x @ Some(_) }

    pattern range_int { 1...10 }
    pattern range_char { 'a'...'z' }

    pattern tuple_unit { () }
    pattern tuple_single { (1,) }
    pattern tuple_simple { (1, 2) }
    pattern tuple_nested { ((1, 2), ref a, (), ) }
    pattern tuple_splat_mid { (a, b, .., c) }
    pattern tuple_splat_empty { (..) }

    pattern path { foo::Bar }
    pattern path_qual { <Type<Arg> as Trait<Arg>>::Foo::<Arg>::Bar }

    pattern struct_tuple1 { Some(1) }
    pattern struct_tuple2 { foo::Foo("bar", ref x) }
    pattern struct_tuple_splat { Named(x, ..) }
    pattern struct_named1 { Point::<f32> { x, ref y, mut z, ref mut w } }
    pattern struct_named2 { ::th::BBox { tl, br: Point { x, y } } }
    pattern struct_named_splat { S { foo, bar, .. } }
    pattern ref_pat { &(a, b) }
}
