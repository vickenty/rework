extern crate docopt;
#[macro_use]
extern crate serde_derive;

#[macro_use]
pub mod syntax;
pub mod parser {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}
pub mod cmd;

pub mod prelude {
    pub use syntax::{walk, Elem};
    pub use syntax::Elem::*;
}
