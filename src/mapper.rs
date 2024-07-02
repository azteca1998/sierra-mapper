use self::{libfuncs::map_libfuncs, types::map_types};
use cairo_lang_sierra::{debug_info::DebugInfo, program::Program};
use smol_str::ToSmolStr;
use std::collections::HashMap;
use tracing::{debug, warn};

mod libfuncs;
mod types;

pub fn map(program: &mut Program, function_names: &HashMap<u64, String>) {
    let type_names = map_types(program);
    let libfunc_names = map_libfuncs(program, &type_names);

    let user_func_names = function_names
        .iter()
        .filter_map(|(&id, name)| {
            let fn_mapping = program
                .funcs
                .iter()
                .find_map(|f| (f.id.id == id).then(|| (f.id.clone(), name.to_smolstr())));

            match &fn_mapping {
                Some((_, name)) => debug!("Mapping function {id} to '{name}'."),
                None => warn!("Function with id {id} doesn't exist. This mapping will be ignored."),
            }

            fn_mapping
        })
        .collect();

    let debug_info = DebugInfo {
        type_names,
        libfunc_names,
        user_func_names,
        annotations: Default::default(),
    };

    debug_info.populate(program);
}
