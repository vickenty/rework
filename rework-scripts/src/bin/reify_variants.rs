#![feature(slice_patterns, advanced_slice_patterns)]
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

    walk(e, &mut |e| reify_struct_variants(e, &mut reified));

    if let Node {
        kind: "program",
        ref mut children,
    } = *e
    {
        children.extend(reified);
    }
}

fn reify_struct_variants<'a, 'i: 'a>(e: &'a mut Elem<'i>, reified: &'a mut Vec<Elem<'i>>) {
    if let Node {
        kind: "variant",
        ref mut children,
    } = *e
    {
        if let &mut [Text { text: ident, .. }, ref mut body] = &mut children[..] {
            if let Node { kind: "empty", .. } = *body {
                return;
            }
            reified.push(gen_reified_struct(ident, body));
            *body = gen_tuple_struct(ident);
        }
    }
}

fn gen_reified_struct<'a, 'i: 'a>(name: &'i str, body: &'a mut Elem<'i>) -> Elem<'i> {
    let semicolon = match *body {
        Node {
            kind: "tuple_struct_body",
            ..
        } => Text {
            prefix: "",
            text: ";",
        },
        _ => Node {
            kind: "empty",
            children: vec![],
        },
    };

    Node {
        kind: "generated",
        children: vec![
            Text {
                prefix: "\n",
                text: "struct",
            },
            Text {
                prefix: "  ",
                text: name,
            },
            body.clone(),
            semicolon,
        ],
    }
}

fn gen_tuple_struct<'i>(name: &'i str) -> Elem<'i> {
    Node {
        kind: "generated",
        children: vec![
            Text {
                prefix: "",
                text: "(",
            },
            Text {
                prefix: "",
                text: name,
            },
            Text {
                prefix: "",
                text: ")",
            },
        ],
    }
}
