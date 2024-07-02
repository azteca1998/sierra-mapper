use crate::utils::format_generic_args;
use cairo_lang_sierra::{
    ids::{ConcreteLibfuncId, ConcreteTypeId},
    program::Program,
};
use smol_str::{SmolStr, ToSmolStr};
use std::collections::HashMap;

pub fn map_libfuncs(
    program: &Program,
    type_names: &HashMap<ConcreteTypeId, SmolStr>,
) -> HashMap<ConcreteLibfuncId, SmolStr> {
    program
        .libfunc_declarations
        .iter()
        .map(|libfunc_declaration| {
            let generic_name = libfunc_declaration.long_id.generic_id.0.as_str();
            let generic_args =
                format_generic_args(type_names, &libfunc_declaration.long_id.generic_args);

            (
                libfunc_declaration.id.clone(),
                if generic_args.is_empty() {
                    generic_name.to_smolstr()
                } else {
                    format!("{generic_name}<{generic_args}>").to_smolstr()
                },
            )
        })
        .collect()
}
