use std::{collections::HashMap, path::PathBuf};

use crate::{SavvyEnum, SavvyFn, SavvyImpl, SavvyStruct};

pub mod savvy_enum;
pub mod savvy_fn;
pub mod savvy_impl;
pub mod savvy_struct;

pub struct ParsedTestCase {
    pub label: String,
    pub code: String,
}

// For main.rs
pub struct ParsedResult {
    pub base_path: std::path::PathBuf,
    pub bare_fns: Vec<SavvyFn>,
    pub impls: Vec<SavvyImpl>,
    pub structs: Vec<SavvyStruct>,
    pub enums: Vec<SavvyEnum>,
    pub mods: Vec<String>,
    pub cur_mod: String,
    pub tests: Vec<ParsedTestCase>,
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
    pub enums: Vec<SavvyEnum>,
}

pub fn merge_parsed_results(results: Vec<ParsedResult>) -> MergedResult {
    let mut bare_fns: Vec<SavvyFn> = Vec::new();
    let mut impl_map: HashMap<String, SavvyMergedImpl> = HashMap::new();
    let mut enums: Vec<SavvyEnum> = Vec::new();

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

        // get documents from struct
        for s in result.structs {
            let key = s.ty.to_string();
            match impl_map.get_mut(&key) {
                Some(merged) => {
                    merged.docs = s.docs;
                }
                None => {
                    impl_map.insert(
                        key,
                        SavvyMergedImpl {
                            docs: Vec::new(),
                            ty: s.ty,
                            fns: Vec::new(),
                        },
                    );
                }
            }
        }

        for e in result.enums {
            let key = e.ty.to_string();
            match impl_map.get_mut(&key) {
                Some(merged) => {
                    merged.docs = e.docs.clone();
                }
                None => {
                    impl_map.insert(
                        key,
                        SavvyMergedImpl {
                            docs: e.docs.clone(),
                            ty: e.ty.clone(),
                            fns: Vec::new(),
                        },
                    );
                }
            }
            enums.push(e);
        }
    }

    let mut impls = impl_map.into_iter().collect::<Vec<_>>();

    // in order to make the wrapper generation deterministic, sort by the type
    impls.sort_by_key(|(k, _)| k.clone());

    MergedResult {
        bare_fns,
        impls,
        enums,
    }
}
