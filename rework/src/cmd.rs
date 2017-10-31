use std::{env, fs, io};
use std::io::{Read, Write};
use std::error::Error;
use docopt::Docopt;
use syntax::Elem;
use parser;

#[derive(Debug, Deserialize)]
struct Args {
    arg_name: Vec<String>,
    arg_output: Option<String>,
}

pub fn simple<F>(desc: &str, mut f: F)
where
    F: FnMut(&mut Elem),
{
    let name = env::args().nth(0).unwrap();
    let usage = format!(
        r#"{desc}

Usage:
    {name} <name>...
    {name} --output <output> <name>
    {name} --help

Options:
    -h --help     Show this message
    -o --output   Write results into a different name
"#,
        name = name,
        desc = desc,
    );

    let args: Args = Docopt::new(&usage[..])
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    for file in &args.arg_name {
        let output = args.arg_output.as_ref().unwrap_or(file);
        if let Err(e) = process_file(file, output, &mut f) {
            eprintln!("{}: {}", file, e);
        }
    }
}

fn process_file<F>(input: &str, output: &str, mut f: F) -> Result<(), Box<Error>>
where
    F: FnMut(&mut Elem),
{
    let buffer = read_file_to_string(input)?;
    let mut parsed = parser::program(&buffer)?;

    f(&mut parsed);

    write_to_file(output, &parsed)?;
    Ok(())
}

fn read_file_to_string(path: &str) -> io::Result<String> {
    let mut buffer = String::new();
    let mut file = fs::File::open(path)?;
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn write_to_file(path: &str, elem: &Elem) -> io::Result<()> {
    if path == "-" {
        let file = io::stdout();
        write!(file.lock(), "{}", elem)?;
    } else {
        let mut file = io::BufWriter::new(fs::File::create(path)?);
        write!(file, "{}", elem)?;
    };

    Ok(())
}
