#[macro_use]
extern crate rework;
use rework::prelude::*;

fn main() {
    let comma = Text {
        prefix: "",
        text: ",",
    };

    rework::cmd::simple(
        "Add comma to the last element of a comma-separated list if followed by a newline",
        |e| {
            find(
                e,
                query!(.. ({:last} {:prefix ~ "\n"}) list {:last} [<2]),
                &mut |e| e.push(comma.clone()),
            )
        },
    );
}
