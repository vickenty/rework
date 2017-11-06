extern crate docopt;
extern crate rework;
extern crate serde;
#[macro_use]
extern crate serde_derive;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use rework::parser;
use rework::query_parser;
use rework::query;
use docopt::Docopt;

#[derive(Debug, Deserialize)]
struct Args {
    arg_name: String,
    arg_query: String,
    flag_verbose: bool,
}

const USAGE: &str = r"
Run a query over a file

Usage:
    rework-query [-v] <name> <query>
    rework-query -h

Options:
    -h --help     Show this message
    -v --verbose  Show verbose output
";

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if let Err(e) = run(&args) {
        eprintln!("{}: {}", args.arg_name, e);
    }
}

fn run(args: &Args) -> Result<(), Box<Error>> {
    let mut buffer = String::new();
    let mut file = File::open(&args.arg_name)?;
    file.read_to_string(&mut buffer)?;

    let mut root = parser::program(&buffer, true)?;
    let query = query_parser::query(&args.arg_query)?;
    query::find(&mut root, &query, &mut |e| if args.flag_verbose {
        println!("{:#?}", e);
    } else {
        println!("{}", e);
    });

    Ok(())
}
