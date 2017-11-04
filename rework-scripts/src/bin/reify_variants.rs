#![feature(slice_patterns, advanced_slice_patterns)]
#[macro_use]
extern crate rework;
use rework::prelude::*;

fn main() {
    rework::cmd::simple(
        "Convert enum struct variants into stand-alone structs",
        process_file,
    );
}

fn process_file(e: &mut Elem) {
    let mut reified: Vec<Elem> = Vec::new();

    find(e, query!(.. variant(* list[>1])), &mut |e| {
        reify_struct_variants(e, &mut reified)
    });

    e.extend(reified);
}

fn reify_struct_variants<'a, 'i: 'a>(e: &'a mut Elem<'i>, reified: &'a mut Vec<Elem<'i>>) {
    if let Node {
        kind: "variant",
        ref mut children,
    } = *e
    {
        if let &mut [Text { text: ident, .. }, ref mut body] = &mut children[..] {
            if !body.is("empty") {
                reified.push(gen_reified_struct(ident, body));
                *body = gen_tuple_struct(ident);
            }
        }
    }
}

fn gen_reified_struct<'a, 'i: 'a>(name: &'i str, body: &'a mut Elem<'i>) -> Elem<'i> {
    let semicolon = if body.is("tuple_struct_body") {
        text("", ";")
    } else {
        node!("empty")
    };

    node!(
        "generated",
        text("\n", "struct"),
        text(" ", name),
        body.clone(),
        semicolon
    )
}

fn gen_tuple_struct<'i>(name: &'i str) -> Elem<'i> {
    node!("generated", text("", "("), text("", name), text("", ")"))
}
