extern crate peg;

fn main() {
    peg::cargo_build("grammar.rustpeg");
    peg::cargo_build("query.rustpeg");
}
