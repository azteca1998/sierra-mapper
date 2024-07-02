mod mapper;
mod utils;

use cairo_lang_sierra::ProgramParser;
use clap::Parser;
use num_bigint::BigUint;
use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
    str::FromStr,
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

    self::mapper::map(
        &mut program,
        args.type_names.into_iter().collect(),
        args.function_names.into_iter().collect(),
    );
    println!("{program}");

    Ok(())
}

/// Sierra program deobfuscator.
#[derive(Debug, Parser)]
struct CmdArgs {
    /// Sierra program to deobfuscate.
    pub input: Option<PathBuf>,

    /// Provide names for user functions.
    #[clap(short = 'f', long = "function-name", value_parser = parse_custom_mapping::<u64>)]
    pub function_names: Vec<(u64, String)>,
    /// Provide names for user types.
    #[clap(short = 't', long = "type-name", value_parser = parse_custom_mapping::<BigUint>)]
    pub type_names: Vec<(BigUint, String)>,
}

fn parse_custom_mapping<K>(input: &str) -> Result<(K, String), K::Err>
where
    K: FromStr,
{
    let (index, value) = input.split_once('=').unwrap();
    Ok((index.parse()?, value.to_string()))
}
