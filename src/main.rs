mod mapper;
mod utils;

use cairo_lang_sierra::ProgramParser;
use clap::Parser;
use std::{fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CmdArgs::parse();

    let program_data = fs::read_to_string(args.input)?;
    let mut program = ProgramParser::new()
        .parse(&program_data)
        .map_err(|e| e.to_string())?;

    self::mapper::map(&mut program, &args.function_names.into_iter().collect());
    println!("{program}");

    Ok(())
}

/// Sierra program deobfuscator.
#[derive(Debug, Parser)]
struct CmdArgs {
    /// Sierra program to deobfuscate.
    pub input: PathBuf,

    /// Provide names for the functions.
    #[clap(short = 'f', long = "function-name", value_parser = parse_function_name)]
    pub function_names: Vec<(u64, String)>,
}

fn parse_function_name(input: &str) -> Result<(u64, String), std::convert::Infallible> {
    let (index, value) = input.split_once('=').unwrap();
    Ok((index.parse().unwrap(), value.to_string()))
}
