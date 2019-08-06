use std::fs;
use std::io::{self, Read, Write};


pub struct Parsed {
    pub input: Box<dyn Read>,
    pub output: Box<dyn Write>,
    pub newline: bool,
    // @Note: It might be better to return a custom ParseError rather than use an Optional exit
    // struct. However, this felt a little cleaner. Going to stick with the optional exit for now
    pub exit: Option<ParseExit>,
}

pub struct ParseExit {
    pub error: bool,
    pub reason: String,
}

pub fn parse(args: Vec<String>) -> Result<Parsed, io::Error> {
    // Default input/output to stdin/stdout
    let mut parsed = Parsed {
        input: Box::new(io::stdin()),
        output: Box::new(io::stdout()),
        newline: true,
        exit: None,
    };

    // Parse args
    let mut args_iter = args[1..].iter();
    while let Some(arg) = args_iter.next() {
        if arg == "-i" || arg == "--input" {
            if let Some(arg) = args_iter.next() {
                parsed.input = Box::new(fs::File::open(arg)?);
            } else {
                parsed.exit = Some(
                    ParseExit {
                        error: true,
                        reason: String::from("Error, input file not provided\n"),
                    });
            }
        } else if arg == "-o" || arg == "--output" {
            if let Some(arg) = args_iter.next() {
                let file = fs::OpenOptions::new().write(true)
                    .truncate(true)
                    .create(true)
                    .open(arg)?;
                parsed.output = Box::new(file);
            } else {
                parsed.exit = Some(
                    ParseExit {
                        error: true,
                        reason: String::from("Error, output file not provided\n"),
                    });
            }
        } else if arg == "-n" || arg == "--no-newline" {
            parsed.newline = false;
        } else if arg == "-h" || arg == "--help" {
            parsed.exit = Some(
                ParseExit {
                    error: false,
                    reason: String::from("Help requested\n"),
                });
        } else {
            parsed.exit = Some(
                ParseExit {
                    error: true,
                    reason: format!("Error, unrecognized input: '{}'\n", arg),
                });
        }
    }

    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn args_with_input() {
        let temp_path = "/tmp/foo.txt";
        let _tf = fs::File::create(Path::new(temp_path)).unwrap();
        // @Note: We will need to cleanup the output file as well ...
        let args: Vec<String> = format!("b64-encode -i {} -o bar -n", temp_path).split(' ').map(|r| r.to_string()).collect();
        let parsed = parse(args).unwrap();
        assert!(!parsed.newline);

        fs::remove_file(Path::new(temp_path)).unwrap();
        assert!(true);
    }
}

