use crate::utils::format_generic_args;
use cairo_lang_sierra::{
    algorithm::topological_order::get_topological_ordering,
    ids::{ConcreteTypeId, FunctionId, UserTypeId},
    program::{GenericArg, Program, StatementIdx},
};
use num_bigint::BigUint;
use smol_str::{SmolStr, ToSmolStr};
use std::{collections::HashMap, sync::LazyLock};
use tracing::debug;

static TUPLE_TYPE_ID: LazyLock<BigUint> = LazyLock::new(|| UserTypeId::from_string("Tuple").id);

pub fn map_types(
    program: &Program,
    func_names: &HashMap<FunctionId, SmolStr>,
) -> HashMap<ConcreteTypeId, SmolStr> {
    debug!("Topologically sorting the Sierra types.");
    let type_declarations = &program.type_declarations;
    let sorted_type_declarations = get_topological_ordering(
        true,
        (0..type_declarations.len()).map(StatementIdx),
        type_declarations.len(),
        |StatementIdx(idx)| {
            Ok(type_declarations[idx]
                .long_id
                .generic_args
                .iter()
                .filter_map(|generic_arg| match generic_arg {
                    GenericArg::Type(type_id) => Some(
                        type_declarations
                            .iter()
                            .position(|x| &x.id == type_id)
                            .unwrap(),
                    ),
                    _ => None,
                })
                .map(StatementIdx)
                .collect())
        },
        |_| unreachable!(),
        |_| panic!(),
    )
    .unwrap();

    debug!("Generating names for all declared types.");
    let mut memory = HashMap::<ConcreteTypeId, SmolStr>::new();
    for StatementIdx(idx) in sorted_type_declarations {
        let long_id = &type_declarations[idx].long_id;
        let name = match long_id.generic_args.first() {
            Some(GenericArg::UserType(id)) => {
                if long_id.generic_id.0 == "Struct" && id.id == *TUPLE_TYPE_ID {
                    let generic_args =
                        format_generic_args(&memory, func_names, &long_id.generic_args);

                    if generic_args.is_empty() {
                        "Unit".to_string()
                    } else {
                        format!("Tuple<{generic_args}>")
                    }
                } else {
                    id.debug_name
                        .as_deref()
                        .map(str::to_string)
                        .unwrap_or_else(|| format!("ut@{id}"))
                }
            }
            _ => {
                let generic_name = long_id.generic_id.0.as_str();
                let generic_args = format_generic_args(&memory, func_names, &long_id.generic_args);

                if generic_args.is_empty() {
                    generic_name.to_string()
                } else {
                    format!("{generic_name}<{generic_args}>")
                }
            }
        };

        memory.insert(type_declarations[idx].id.clone(), name.to_smolstr());
    }

    memory
}
