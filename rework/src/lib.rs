extern crate docopt;
#[macro_use]
extern crate serde_derive;

#[macro_use]
pub mod syntax;
pub mod parser {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

#[macro_use]
pub mod query;
pub mod query_parser {
    include!(concat!(env!("OUT_DIR"), "/query.rs"));
}

pub mod cmd;

pub mod prelude {
    pub use syntax::{text, walk, Elem};
    pub use syntax::Elem::*;
    pub use query::find;
}

#[cfg(test)]
mod tests;
