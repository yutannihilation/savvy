use std::{collections::HashMap, path::PathBuf};

use crate::{SavvyFn, SavvyImpl};

pub mod savvy_fn;
pub mod savvy_impl;
pub mod savvy_struct;

// For main.rs
pub struct ParsedResult {
    pub base_path: std::path::PathBuf,
    pub bare_fns: Vec<SavvyFn>,
    pub impls: Vec<SavvyImpl>,
    pub mods: Vec<String>,
}

impl ParsedResult {
    pub fn mod_dirs(&self) -> Vec<PathBuf> {
        self.mods.iter().map(|x| self.base_path.join(x)).collect()
    }
}

pub struct SavvyMergedImpl {
    /// Doc comments
    pub docs: Vec<String>,
    /// Original type name
    pub ty: syn::Ident,
    /// Methods and accociated functions
    pub fns: Vec<SavvyFn>,
}

pub struct MergedResult {
    pub bare_fns: Vec<SavvyFn>,
    pub impls: Vec<(String, SavvyMergedImpl)>,
}

pub fn merge_parsed_results(results: Vec<ParsedResult>) -> MergedResult {
    let mut bare_fns: Vec<SavvyFn> = Vec::new();
    let mut impl_map: HashMap<String, SavvyMergedImpl> = HashMap::new();

    for result in results {
        let mut fns = result.bare_fns;
        bare_fns.append(&mut fns);

        for i in result.impls {
            let key = i.ty.to_string();
            match impl_map.get_mut(&key) {
                Some(merged) => {
                    let mut fns = i.fns;
                    merged.fns.append(&mut fns);
                }
                None => {
                    impl_map.insert(
                        key,
                        SavvyMergedImpl {
                            docs: Vec::new(),
                            ty: i.ty,
                            fns: i.fns,
                        },
                    );
                }
            }
        }
    }

    let mut impls = impl_map.into_iter().collect::<Vec<_>>();

    // in order to make the wrapper generation deterministic, sort by the type
    impls.sort_by_key(|(k, _)| k.clone());

    MergedResult { bare_fns, impls }
}
