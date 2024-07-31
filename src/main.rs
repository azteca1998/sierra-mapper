use cairo_lang_sierra::ProgramParser;
use cairo_lang_starknet_classes::contract_class::ContractClass;
use clap::Parser;
use num_bigint::BigUint;
use sierra_mapper::{mapper, utils};
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
            .with_env_filter(
                EnvFilter::builder()
                    .with_default_directive(LevelFilter::WARN.into())
                    .from_env_lossy(),
            )
            .with_writer(io::stderr)
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

    let (mut program, contract_class) = if program_data.trim_start().starts_with('{') {
        info!("Parsing the Starknet contract");
        let contract_class = serde_json::from_str::<ContractClass>(&program_data)?;
        let program = contract_class.extract_sierra_program()?;

        (program, Some(contract_class))
    } else {
        info!("Parsing the Sierra program");
        let program = ProgramParser::new()
            .parse(&program_data)
            .map_err(|e| e.to_string())?;

        (program, None)
    };

    let (type_mappings, function_mappings) = match contract_class {
        Some(contract_class) => {
            let (mut type_mappings, mut function_mappings) =
                utils::extract_contract_abi(contract_class);

            type_mappings.extend(args.type_names);
            function_mappings.extend(args.function_names);

            (type_mappings, function_mappings)
        }
        None => (
            args.type_names.into_iter().collect(),
            args.function_names.into_iter().collect(),
        ),
    };

    self::mapper::map(&mut program, type_mappings, function_mappings);
    println!("{program}");

    Ok(())
}

/// Sierra program deobfuscator.
#[derive(Debug, Parser)]
struct CmdArgs {
    /// Sierra program or contract to deobfuscate.
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
