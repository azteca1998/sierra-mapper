use cairo_lang_sierra::{ids::ConcreteTypeId, program::GenericArg};
use itertools::Itertools;
use smol_str::SmolStr;
use std::collections::HashMap;

pub fn format_generic_args(
    memory: &HashMap<ConcreteTypeId, SmolStr>,
    generic_args: &[GenericArg],
) -> String {
    Itertools::intersperse_with(
        generic_args
            .iter()
            .filter_map(|generic_arg| match generic_arg {
                GenericArg::UserType(_) => None,
                GenericArg::Type(ty) => Some(memory[ty].to_string()),
                GenericArg::Value(val) => Some(val.to_string()),
                GenericArg::UserFunc(_) => todo!(),
                GenericArg::Libfunc(_) => unreachable!(),
            }),
        || ", ".to_string(),
    )
    .collect::<String>()
}
