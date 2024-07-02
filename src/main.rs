mod mapper;
mod utils;

use cairo_lang_sierra::ProgramParser;
use clap::Parser;
use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = CmdArgs::parse();

    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_env_filter(EnvFilter::from_default_env().add_directive(LevelFilter::WARN.into()))
            .finish(),
    )?;

    let program_data = match args.input {
        Some(path) => {
            info!("Reading the Sierra program from disk.");
            fs::read_to_string(path)?
        }
        None => {
            info!("Reading the Sierra program from the standard input.");
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            buf
        }
    };

    info!("Parsing the Sierra program");
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
    pub input: Option<PathBuf>,

    /// Provide names for the functions.
    #[clap(short = 'f', long = "function-name", value_parser = parse_function_name)]
    pub function_names: Vec<(u64, String)>,
}

fn parse_function_name(input: &str) -> Result<(u64, String), std::convert::Infallible> {
    let (index, value) = input.split_once('=').unwrap();
    Ok((index.parse().unwrap(), value.to_string()))
}
