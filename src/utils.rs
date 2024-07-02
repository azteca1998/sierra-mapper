use cairo_lang_sierra::{
    ids::{ConcreteTypeId, FunctionId},
    program::GenericArg,
};
use itertools::Itertools;
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
