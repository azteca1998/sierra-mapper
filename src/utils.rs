use cairo_lang_sierra::{
    ids::{ConcreteTypeId, FunctionId},
    program::GenericArg,
};
use cairo_lang_starknet_classes::{contract_class::ContractClass, keccak::starknet_keccak};
use itertools::Itertools;
use num_bigint::BigUint;
use smol_str::SmolStr;
use std::collections::HashMap;

pub fn format_generic_args(
    type_names: &HashMap<ConcreteTypeId, SmolStr>,
    func_names: &HashMap<FunctionId, SmolStr>,
    generic_args: &[GenericArg],
) -> String {
    Itertools::intersperse_with(
        generic_args
            .iter()
            .filter_map(|generic_arg| match generic_arg {
                GenericArg::UserType(_) => None,
                GenericArg::Type(ty) => Some(type_names[ty].to_string()),
                GenericArg::Value(val) => Some(val.to_string()),
                GenericArg::UserFunc(r#fn) => func_names.get(r#fn).map(SmolStr::to_string),
                GenericArg::Libfunc(_) => unreachable!(),
            }),
        || ", ".to_string(),
    )
    .collect::<String>()
}

pub fn extract_contract_abi(
    contract_class: ContractClass,
) -> (HashMap<BigUint, String>, HashMap<u64, String>) {
    let contract_abi = match contract_class.abi {
        Some(contract_abi) => contract_abi,
        None => return (HashMap::new(), HashMap::new()),
    };

    let mut type_names = HashMap::new();
    let mut function_names = HashMap::new();
    for abi_item in contract_abi {
        match abi_item {
            cairo_lang_starknet_classes::abi::Item::Function(function_abi) => {
                let selector = starknet_keccak(function_abi.name.as_bytes());
                let function_idx = contract_class
                    .entry_points_by_type
                    .external
                    .iter()
                    .find_map(|entry_point| {
                        (entry_point.selector == selector).then_some(entry_point.function_idx)
                    })
                    .unwrap();

                function_names.insert(function_idx as u64, function_abi.name);
            }
            cairo_lang_starknet_classes::abi::Item::Constructor(_) => todo!(),
            cairo_lang_starknet_classes::abi::Item::L1Handler(_) => todo!(),
            cairo_lang_starknet_classes::abi::Item::Event(_) => {
                // TODO: Handle events.
            },
            cairo_lang_starknet_classes::abi::Item::Struct(_) => todo!(),
            cairo_lang_starknet_classes::abi::Item::Enum(_) => todo!(),
            cairo_lang_starknet_classes::abi::Item::Interface(_) => todo!(),
            cairo_lang_starknet_classes::abi::Item::Impl(_) => todo!(),
        }
    }

    (type_names, function_names)
}
