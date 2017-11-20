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
use rework::syntax::Elem;
use rework::query;
use docopt::Docopt;

#[derive(Debug, Deserialize)]
struct Args {
    arg_name: String,
    arg_query: String,
    flag_verbose: bool,
    flag_outline: bool,
}

const USAGE: &str = r"
Run a query over a file

Usage:
    rework-query [-v] [-o] <name> <query>
    rework-query -h

Options:
    -h --help     Show this message
    -v --verbose  Show verbose output
    -o --outline  Show outline of each matched item
";

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if let Err(e) = run(&args) {
        eprintln!("{}: {}", args.arg_name, e);
    }
}

const PADDING: &str = "  ";

fn outline(e: &Elem, depth: usize) {
    if let Elem::Node { kind, ref children } = *e {
        for _ in 0..depth {
            print!("{}", PADDING);
        }

        print!("{}:", kind);

        let mut seen = None;
        let mut count = 0;

        for child in children {
            match *child {
                Elem::Text { text, .. } => {
                    print!(" \x1b[1m{}\x1b[0m", text);
                    seen = None;
                },
                Elem::Node { kind, .. } => {
                    if seen == Some(kind) {
                        count += 1;
                        if count == 2 {
                            print!("\x1b[2m…\x1b[0m");
                        }
                    }
                    else {
                        print!(" \x1b[3m\x1b[2m{}\x1b[0m", kind);
                        seen = Some(kind);
                        count = 1;
                    }
                }
            }
        }

        print!("\n");

        for child in children {
            if let Elem::Node { kind: "empty", .. } = *child {
                continue
            }
            outline(child, depth + 1);
        }
    }
}

fn run(args: &Args) -> Result<(), Box<Error>> {
    let mut buffer = String::new();
    let mut file = File::open(&args.arg_name)?;
    file.read_to_string(&mut buffer)?;

    let mut root = parser::module(&buffer, true)?;
    let query = query_parser::query(&args.arg_query)?;
    query::find(&mut root, &query, &mut |e| if args.flag_outline {
        outline(e, 0);
    } else if args.flag_verbose {
        println!("{:#?}", e);
    } else {
        println!("{}", e);
    });

    Ok(())
}
