use std::io::{Read, Write};

use clap::Parser;

#[derive(Debug, clap::Parser)]
#[clap(
    author,
    version,
    about = "A simple program to expand environment variables in a string.",
)]
struct Args {
    #[arg(help = "The input string to expand. If not provided, the program will read from STDIN until EOF.")]
    input: Option<String>,

    #[arg(short, long = "variables", help = "One or more custom variables to use in the expansion. Each variable should be in the format NAME=VALUE.")]
    variables: Vec<String>,

    #[arg(short, long, help = "Only use the variables provided with the --variables flag, ignoring any environment variables.")]
    no_env: bool,

    #[arg(short, long, help = "If set, variables that are not found will be replaced with an empty string instead of being left unchanged.")]
    empty: bool,
}

fn main() {
    let args = Args::parse();

    let mut variables = Vec::with_capacity(args.variables.len());

    for var in args.variables {
        if let Some((name, value)) = var.split_once('=') {
            variables.push((name.to_string(), value.to_string()));
        } else {
            eprintln!("Invalid variable format: '{}'. Expected format is NAME=VALUE.", var);
            std::process::exit(1);
        }
    }

    let input = if let Some(input) = args.input {
        input
    } else {
        let mut buffer = String::new();
        std::io::stdin()
            .read_to_string(&mut buffer)
            .expect("Failed to read from stdin");
        buffer
    };

    let output = shellexpand::env_with_context_no_errors(&input, move |key| {
        if let Some((_, value)) = variables.iter().find(|(name, _)| name == key) {
            Some(value.to_string())
        } else if !args.no_env {
            std::env::var(key).ok().or_else(|| {
                if args.empty {
                    Some(String::new())
                } else {
                    None
                }
            })
        } else {
            if args.empty {
                Some(String::new())
            } else {
                None
            }
        }
    });

    print!("{}", output);
    std::io::stdout().flush().unwrap();
}
