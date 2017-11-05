parse! {
    expr int1 { 1 }
    expr int2 { 100_000u8 }
    expr int3 { 0x80_00u32 }
    expr int4 { 0o010 }
    expr int5 { 0b0100_0001 }

    expr flt1 { 10.0_3e+10f32 }
    expr flt2 { 0.5 }
    expr flt3 { 1. }
    expr flt4 { 1f32 }

    expr str_ascii { "\"Hello, \u{1f310}\"" }
    expr str_unicode { "☺" }
    expr string_raw1 { r"foo\n" }
    expr string_raw2 { r#"#"# }
    expr string_raw3 { r##"##"## }

    expr char_ascii { 'a' }
    expr char_escape1 { '\n' }
    expr char_escape2 { '\u{1f310}' }
    expr char_escape3 { '\'' }
    expr char_unicode { '☺' }

    expr array1 { [ 1 ] }
    expr array2 { [(a, b), c] }
    expr array3 { [None; 4] }
    expr array4 { [None; std::u8::MAX] }

    expr struct_named1 { Foo { x: x, y: y } }
    expr struct_named2 { Foo { x, y: y } }
    expr struct_named3 { Foo {} }
    expr struct_tuple1 { Foo(x, y) }
    expr struct_tuple2 { Foo() }
    expr struct_empty1 { Foo }

    // FIXME: add disambiguation tests
    expr tuple1 { () }
    expr tuple2 { (1,) }
    expr tuple3 { (1, 2) }
    expr tuple4 { (1, 2,) }
    expr grouped { (1) }

    expr id1 { x }
    expr id2 { std::usize::MAX }
    expr id3 { Vec::<u8>::len }

    expr unop_neg { -1 }
    expr unop_not { !0 }
    expr unop_ref { &foo }
    expr unop_mut { &mut &mut bar }
    expr unop_deref { *foo }
    expr unop_reborrow { &mut *foo }

    expr cast1 { 1 as f32 }
    expr cast2 { foo as (Foo + 'b) }

    expr expr_if { if true { 1 } else { 2 } }
    expr expr_match {
        match *foo {
            Some(x) => x,
            None => return,
        }
    }

    expr expr_loop { loop { break 1 } }
    expr expr_for { for x in xs { } }
    expr expr_while { while true { } }
    expr expr_while_let { while let None = x { } }
    expr expr_break1 { break }
    expr expr_break2 { break 1 }
    expr expr_break3 { break 'foo }
    expr expr_break4 { break 'foo 2 }
    expr expr_continue { continue }
    expr expr_return1 { return }
    expr expr_return2 { return foo }

    expr assign { x = 1 }
    expr assign_add { x += 1 }

    expr block { { 1; 2; 3 } }

    expr closure1 { || 1 }
    expr closure2 { |_| (2, 3) }
    expr closure3 { |a| { a } }
    expr closure4 { |&a| *a }

    expr tuple_field { foo.0 }
    expr named_field1 { foo.bar }
    expr named_field2 { (**foo).bar }

    expr index1 { foo[bar] }
    expr index2 { &foo[1..4] }
    expr index3 { &(*foo)[..] }
    expr index4 { foo[(a, b)][c] }

    expr call1 { foo(1) }
    expr call2 { &foo(2) }
    expr call3 { Foo::bar(baz) }
    expr call4 { <Foo as Box<u8>>::bar(ook) }
    expr call5 { mem::size_of::<u8>() }
    expr call6 { ()() }
}

parse! {
    expr binop_prec1 { a + b * c } => { .. (* {:text = "+"}) .. (* {:text = "*"})}
    expr binop_prec2 { a * b + c } => { .. (* {:text = "+"}) .. (* {:text = "*"})}
    expr cast_plus { 1 as f32 + 2. } => { .. (* {:text = "+"}) .. ({:text = "as"}) }
    expr unop_cast { !0 as f32 } => { .. (* {:text = "as"}) .. ({:text = "!"}) }
}
