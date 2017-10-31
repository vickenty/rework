#![feature(slice_patterns, advanced_slice_patterns)]
extern crate rework;
use rework::prelude::*;

fn is_comma_list<'a, 'i: 'a>(e: &'a mut Elem<'i>) -> Option<(&'a str, &'a mut Vec<Elem<'i>>)> {
    if let Node {
        ref mut children, ..
    } = *e
    {
        if let &mut [Text { .. }, Node {
            kind: "comma_list",
            ref mut children,
        }, Text { prefix, .. }] = &mut children[..]
        {
            return Some((prefix, children));
        }
    }

    None
}

fn main() {
    let comma = Text {
        prefix: "",
        text: ",",
    };

    rework::cmd::simple("Add comma to the last element of a comma-separated list if followed by a newline", |e| {
        walk(e, &mut |e| {
            if let Some((prefix, children)) = is_comma_list(e) {
                if prefix.contains('\n') {
                    // if list ends with a comma_elem without trailing comma...
                    if let Some(&mut Node {
                        kind: "comma_elem",
                        ref mut children,
                    }) = children.last_mut()
                    {
                        if children.len() < 2 {
                            children.push(comma.clone());
                        }
                    }
                }
            }
        })
    });
}
