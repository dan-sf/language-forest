use std::env;
use std::error;
use std::io::{self, Read, Write};
use std::process;

mod arg_parse;
mod base64;


fn usage_string() -> String {
    let message = format!("{}{}{}{}{}",
        "Usage:  b64-encode [-h] [-i in_file] [-o out_file]\n",
        "-h, --help        display this message\n",
        "-i, --input       input file (default: stdin)\n",
        "-o, --output      output file (default: stdout)\n",
        "-n, --no-newline  remove trailing newline from output\n");
    message
}

fn usage_to_stdout() {
    io::stdout().write_all(usage_string().as_bytes()).expect(
                "Error: unexpected issue writing to stdout");
}

fn main() -> Result<(), Box<error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mut parsed_args = arg_parse::parse(args)?;

    // Check if we should exit early
    if let Some(e) = parsed_args.exit {
        if e.error {
            io::stdout().write_all(e.reason.as_bytes())?;
            usage_to_stdout();
            process::exit(1);
        } else {
            usage_to_stdout();
            process::exit(0);
        }
    };

    let read_bytes = parsed_args.input.bytes();
    let encoded = base64::base_64_encode(read_bytes.map(
            |r| r.expect(
                "b64-encode error: Unexpected issue reading input binary data")
            ).collect())?;
    parsed_args.output.write(encoded.as_bytes())?;
    if parsed_args.newline {
        parsed_args.output.write(String::from("\n").as_bytes())?;
    }

    Ok(())
}

